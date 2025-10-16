use crate::parse::parse_stream::ParseInput;
use crate::parse::util::*;
use crate::result::Result;
use proc_macro2::TokenStream;
use quote::quote;
use wat_defs::ty::AbsHeapType;

/// Pre: `input` contains the open parens and block type name.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
pub fn parse_block_type(input: ParseInput) -> Result<TokenStream> {
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
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_val_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::ValType];
    if let Ok(num_type) = parse_num_type(input) {
        Ok(quote![ #path::Num(#num_type) ])
    } else if let Ok(ref_type) = parse_ref_type(input) {
        Ok(quote![ #path::Ref(#ref_type) ])
    } else {
        Err(error(input, "Expected `ValType`."))
    }
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_num_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::NumType];

    let name = expect_ident_named(&["i32"], input)?;
    let name = name.to_string();
    match name.as_str() {
        "i32" => Ok(quote![ #path::I32 ]),
        _ => Err(error(input, format!("Invalid `NumType`: {}", name))),
    }
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_ref_type(input: ParseInput) -> Result<TokenStream> {
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
        };
    })
}

fn parse_heap_type(input: ParseInput) -> Result<TokenStream> {
    if let Ok(res) = parse_abs_heap_type(input) {
        Ok(res)
    } else if let Ok(res) = parse_type_sym(input) {
        Ok(res)
    } else {
        Err(error(input, "Expected `HeapType`"))
    }
}

fn parse_abs_heap_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::AbsHeapType];
    let ident = expect_ident_named(&["i31"], input)?;
    Ok(match ident.to_string().as_str() {
        "i31" => quote![ #path::I31 ],
        _ => unreachable!(),
    })
}

fn parse_type_sym(input: ParseInput) -> Result<TokenStream> {
    let name = expect_sym(input)?.to_string();
    Ok(quote![ #name ])
}

#[cfg(test)]
mod test {
    use expect_test::expect;
    use proc_macro2::TokenStream;
    use quote::quote;
    use crate::parse::ParseStream;
    use super::*;

    #[test]
    pub fn test_parse_num_type() {
        let input: TokenStream = quote! { i32 };
        let actual = parse_num_type(&mut ParseStream::new(input)).unwrap().to_string();
        let expected = expect![[r#"wat_defs :: ty :: NumType :: I32"#]];
        expected.assert_eq(&actual);
    }


    #[test]
    pub fn test_parse_block_type_result() {
        let input: TokenStream = quote! { (result i32) };
        let actual = parse_block_type(&mut ParseStream::new(input)).unwrap().to_string();
        let expected = expect![[r#"wat_defs :: ty :: BlockType :: Result (wat_defs :: ty :: ValType :: Num (wat_defs :: ty :: NumType :: I32))"#]];
        expected.assert_eq(&actual);
    }
}