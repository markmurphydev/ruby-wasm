use crate::parse::func;
use crate::parse::parse_stream::ParseInput;
use crate::parse::util::*;
use crate::result::Result;
use proc_macro2::TokenStream;
use quote::quote;

/// Pre: `(type ...)` has been unwrapped. We are consuming one of:
///     - `(sub ...)`
///     - `<comp_type>`
pub fn parse_sub_type(input: ParseInput) -> Result<TokenStream> {
    if let Ok((mut input, _)) = expect_open_paren_named(&["sub"], input) {
        let input = &mut input;
        let is_final = match expect_ident_named(&["final"], input) {
            Ok(_) => quote![wat_defs::ty::Final::Final],
            Err(_) => quote![wat_defs::ty::Final::NotFinal],
        };
        let supertypes = parse_type_idxs(input)?;
        let comp_type = parse_comp_type(input)?;
        Ok(quote! {
            wat_defs::ty::SubType {
                is_final: #is_final,
                supertypes: #supertypes,
                comp_type: #comp_type,
            }
        })
    } else if let Ok(comp_type) = parse_comp_type(input) {
        Ok(quote! {
            wat_defs::ty::SubType {
                is_final: wat_defs::ty::Final::Final,
                supertypes: vec![],
                comp_type: #comp_type,
            }
        })
    } else {
        Err(error(input, "Expected `SubType`."))
    }
}

fn parse_comp_type(input: ParseInput) -> Result<TokenStream> {
    let res = if let Ok(ty) = parse_struct_type(input) {
        quote![ wat_defs::ty::CompType::Struct(#ty) ]
    } else if let Ok(ty) = parse_array_type(input) {
        quote![ wat_defs::ty::CompType::Array(#ty) ]
    } else {
        let ty = parse_func_type(input)?;
        quote![ wat_defs::ty::CompType::Func(#ty) ]
    };
    Ok(res)
}

fn parse_array_type(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["array"], input)?;
    let field_type = parse_field_type(&mut input)?;

    Ok(quote! {
        wat_defs::ty::ArrayType {
            field_type: #field_type
        }
    })
}

fn parse_struct_type(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["struct"], input)?;
    let fields = parse_fields(&mut input)?;

    Ok(quote! {
        wat_defs::ty::StructType {
            fields: #fields,
        }
    })
}

fn parse_fields(input: ParseInput) -> Result<TokenStream> {
    parse_while_ok(input, parse_field)
}

fn parse_field(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["field"], input)?;
    let input = &mut input;
    let name = parse_name(input)?;
    let ty = parse_field_type(input)?;

    Ok(quote! {
        wat_defs::ty::Field {
            name: #name,
            ty: #ty
        }
    })
}

fn parse_field_type(input: ParseInput) -> Result<TokenStream> {
    let (mutable, ty) = match expect_open_paren_named(&["mut"], input) {
        Ok((mut input, _)) => (
            quote![wat_defs::ty::Mutable::Mutable],
            parse_storage_type(&mut input)?,
        ),
        Err(_) => (
            quote![wat_defs::ty::Mutable::Const],
            parse_storage_type(input)?,
        ),
    };
    Ok(quote! {
        wat_defs::ty::FieldType {
            mutable: #mutable,
            ty: #ty,
        }
    })
}

fn parse_storage_type(input: ParseInput) -> Result<TokenStream> {
    check_quasi_quote!(input => {
        let res = if let Ok(ty) = parse_pack_type(input) {
            quote![ wat_defs::ty::StorageType::Pack(#ty) ]
        } else {
            let val_type = parse_val_type(input)?;
            quote![ wat_defs::ty::StorageType::Val(#val_type) ]
        };
        Ok(res)
    })
}

fn parse_pack_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::PackType];

    let name = expect_ident_named(&["i8", "i16"], input)?;
    let name = name.to_string();
    match name.as_str() {
        "i8" => Ok(quote![ #path::I8 ]),
        "i16" => Ok(quote![ #path::I16 ]),
        _ => Err(error(input, format!("Invalid `PackType`: {}", name))),
    }
}

fn parse_func_type(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["func"], input)?;
    let input = &mut input;
    let params = func::parse_params(input)?;
    let results = func::parse_results(input)?;
    Ok(quote! {
        wat_defs::ty::FuncType {
            params: #params,
            results: #results
        }
    })
}

pub fn parse_global_type(input: ParseInput) -> Result<TokenStream> {
    let (mutable, val_type) = match expect_open_paren_named(&["mut"], input) {
        Ok((mut body, _)) => {
            let mutable = quote![wat_defs::ty::Mutable::Mutable];
            let val_type = parse_val_type(&mut body)?;
            (mutable, val_type)
        }
        Err(_) => {
            let mutable = quote![wat_defs::ty::Mutable::Const];
            let val_type = parse_val_type(input)?;
            (mutable, val_type)
        }
    };

    Ok(quote! {
        wat_defs::ty::GlobalType {
            mutable: #mutable,
            val_type: #val_type,
        }
    })
}

/// Pre: `input` contains the open parens and block type name.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
pub fn parse_block_type(input: ParseInput) -> Result<TokenStream> {
    check_quasi_quote!(input => {
        let (mut input, name) = expect_open_paren_named(&["type", "result"], input)?;
        let input = &mut input;
        let name = name.to_string();

        let res = match name.as_str() {
            "type" => {
                // (type $name)
                expect_dollar(input)?;
                let ty_name = expect_ident(input)?;
                quote![ wat_defs::ty::BlockType::TypeUse(#ty_name.to_string()) ]
            }
            "result" => {
                let ty = parse_val_type(input)?;
                quote![ wat_defs::ty::BlockType::Result(#ty) ]
            }
            _ => unreachable!(),
        };
        Ok(res)
    })
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
pub fn parse_val_type(input: ParseInput) -> Result<TokenStream> {
    check_quasi_quote!(input => {
        let path = quote![wat_defs::ty::ValType];
        if let Ok(num_type) = parse_num_type(input) {
            Ok(quote![ #path::Num(#num_type) ])
        } else if let Ok(ref_type) = parse_ref_type(input) {
            Ok(quote![ #path::Ref(#ref_type) ])
        } else {
            Err(error(input, "Expected `ValType`."))
        }
    })
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_num_type(input: ParseInput) -> Result<TokenStream> {
    check_quasi_quote!(input => {
        let path = quote![wat_defs::ty::NumType];

        let name = expect_ident_named(&["i32", "i64"], input)?;
        let name = name.to_string();
        match name.as_str() {
            "i32" => Ok(quote![ #path::I32 ]),
            "i64" => Ok(quote![ #path::I64 ]),
            _ => Err(error(input, format!("Invalid `NumType`: {}", name))),
        }
    })
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
pub fn parse_ref_type(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["ref"], input)?;
    let input = &mut input;

    let nullable = match expect_ident_named(&["null"], input) {
        Ok(_) => quote![wat_defs::ty::Nullable::Nullable],
        Err(_) => quote![wat_defs::ty::Nullable::NonNullable],
    };

    let heap_type = parse_heap_type(input)?;

    Ok(quote! {
        wat_defs::ty::RefType {
            null: #nullable,
            heap_type: #heap_type,
        }
    })
}

pub fn parse_heap_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::HeapType];
    if let Ok(res) = parse_abs_heap_type(input) {
        Ok(quote![ #path::Abs(#res) ])
    } else if let Ok(res) = parse_name(input) {
        Ok(quote![ #path::TypeIdx(#res) ])
    } else {
        Err(error(input, "Expected `HeapType`"))
    }
}

fn parse_abs_heap_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::AbsHeapType];
    let ident = expect_ident_named(
        &[
            "func", "extern", "any", "none", "noextern", "nofunc", "eq", "struct", "array", "i31",
            "exn", "noexn",
        ],
        input,
    )?;
    Ok(match ident.to_string().as_str() {
        "func" => quote![ #path::Func ],
        "extern" => quote![ #path::Extern ],
        "any" => quote![ #path::Any ],
        "none" => quote![ #path::None ],
        "noextern" => quote![ #path::NoExtern ],
        "nofunc" => quote![ #path::NoFunc ],
        "eq" => quote![ #path::Eq ],
        "struct" => quote![ #path::Struct ],
        "array" => quote![ #path::Array ],
        "i31" => quote![ #path::I31 ],
        "exn" => quote![ #path::Exn ],
        "noexn" => quote![ #path::NoExn ],
        _ => unreachable!(),
    })
}

fn parse_type_idxs(input: ParseInput) -> Result<TokenStream> {
    parse_while_ok(input, parse_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ParseStream;
    use expect_test::expect;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    pub fn test_parse_num_type() {
        let input: TokenStream = quote! { i32 };
        let actual = parse_num_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let expected = expect![[r#"wat_defs :: ty :: NumType :: I32"#]];
        expected.assert_eq(&actual);
    }

    #[test]
    pub fn test_parse_block_type_result() {
        let input: TokenStream = quote! { (result i32) };
        let actual = parse_block_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let expected = expect![[
            r#"wat_defs :: ty :: BlockType :: Result (wat_defs :: ty :: ValType :: Num (wat_defs :: ty :: NumType :: I32))"#
        ]];
        expected.assert_eq(&actual);
    }

    #[test]
    pub fn test_array_i8() {
        let input = quote! { (array i8) };
        let actual = parse_array_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let actual = &format!("{}", actual);
        let expected = expect![
            "wat_defs :: ty :: ArrayType { field_type : wat_defs :: ty :: FieldType { mutable : wat_defs :: ty :: Mutable :: Const , ty : wat_defs :: ty :: StorageType :: Pack (wat_defs :: ty :: PackType :: I8) , } }"
        ];
        expected.assert_eq(actual);
    }

    #[test]
    pub fn test_ref_eq() {
        let input = quote! { (ref eq) };
        let actual = parse_ref_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let actual = &format!("{}", actual);
        let expected = expect!["wat_defs :: ty :: RefType { null : wat_defs :: ty :: Nullable :: NonNullable , heap_type : wat_defs :: ty :: HeapType :: Abs (wat_defs :: ty :: AbsHeapType :: Eq) , }"];
        expected.assert_eq(actual);
    }

    #[test]
    pub fn test_ref_name() {
        let input = quote! { (ref $obj) };
        let actual = parse_ref_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let actual = &format!("{}", actual);
        let expected = expect![[r#"wat_defs :: ty :: RefType { null : wat_defs :: ty :: Nullable :: NonNullable , heap_type : wat_defs :: ty :: HeapType :: TypeIdx ("obj" . to_string ()) , }"#]];
        expected.assert_eq(actual);
    }

    #[test]
    pub fn test_array_ref_eq() {
        let input = quote! { (array (ref eq)) };
        let actual = parse_comp_type(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let actual = &format!("{}", actual);
        let expected = expect!["wat_defs :: ty :: CompType :: Array (wat_defs :: ty :: ArrayType { field_type : wat_defs :: ty :: FieldType { mutable : wat_defs :: ty :: Mutable :: Const , ty : wat_defs :: ty :: StorageType :: Val (wat_defs :: ty :: ValType :: Ref (wat_defs :: ty :: RefType { null : wat_defs :: ty :: Nullable :: NonNullable , heap_type : wat_defs :: ty :: HeapType :: Abs (wat_defs :: ty :: AbsHeapType :: Eq) , })) , } })"];
        expected.assert_eq(actual);
    }

    #[test]
    pub fn test_comp_type_array() {
        let input = quote! { (array i8) };
        let actual = parse_comp_type(&mut ParseStream::new(input)).unwrap().to_string();
        let actual = &format!("{}", actual);
        let expected = expect!["wat_defs :: ty :: CompType :: Array (wat_defs :: ty :: ArrayType { field_type : wat_defs :: ty :: FieldType { mutable : wat_defs :: ty :: Mutable :: Const , ty : wat_defs :: ty :: StorageType :: Pack (wat_defs :: ty :: PackType :: I8) , } })"];
        expected.assert_eq(actual);
    }

    #[test]
    pub fn test_sub_type_array_abbreviated() {
        // When we call
        let input = quote! { (array i8) };
        let actual = parse_sub_type(&mut ParseStream::new(input)).unwrap().to_string();
        let actual = &format!("{}", actual);
        let expected = expect!["wat_defs :: ty :: SubType { is_final : wat_defs :: ty :: Final :: Final , supertypes : vec ! [] , comp_type : wat_defs :: ty :: CompType :: Array (wat_defs :: ty :: ArrayType { field_type : wat_defs :: ty :: FieldType { mutable : wat_defs :: ty :: Mutable :: Const , ty : wat_defs :: ty :: StorageType :: Pack (wat_defs :: ty :: PackType :: I8) , } }) , }"];
        expected.assert_eq(actual);
    }
}
