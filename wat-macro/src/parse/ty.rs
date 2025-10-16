use crate::parse::parse_stream::ParseInput;
use crate::parse::util::*;
use crate::result::Result;
use proc_macro2::TokenStream;
use quote::quote;

/// Pre: `input` contains the open parens and block type name.
pub fn parse_block_type(input: ParseInput) -> Result<TokenStream> {
    let input = &mut expect_parens!(input)?;
    let name = expect_ident!(input)?;
    let name = name.to_string();
    let res = match name.as_str() {
        "type" => {
            // (type $name)
            expect_dollar!(input)?;
            let ty_name = expect_ident!(input)?;
            quote![ wat_defs::ty::BlockType::TypeUse(#ty_name.to_string()) ]
        }
        "result" => {
            let ty = parse_val_type(input)?;
            quote![ wat_defs::ty::BlockType::Result(#ty) ]
        }
        _ => panic!(),
    };
    Ok(res)
}

/// Pre: `input` contains entire type.
fn parse_val_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::ValType];
    if let Ok(num_type) = parse_num_type(input) {
        Ok(quote![ #path::Num(#num_type) ])
    } else if let Ok(ref_type) = parse_ref_type(input) {
        Ok(quote![ #path::Ref(#ref_type) ])
    } else {
        todo!()
    }
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_num_type(input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::ty::NumType];
    if peek_open_paren_named(&["i32"], input) {
        let input = &mut expect_parens!(input)?;
        let name = expect_ident!(input)?.to_string();
        match name.as_str() {
            "i32" => Ok(quote![ #path::I32 ]),
            _ => Err(error(input, format!("Invalid `NumType`: {}", name))),
        }
    } else {
        Err(error(input, "Invalid `NumType`"))
    }
}

/// Pre: `input` contains entire type.
/// Post: If `input` _is_ valid input, but _cannot_ be parsed as NumType, does not consume any tokens.
fn parse_ref_type(input: ParseInput) -> Result<TokenStream> {
    if peek_open_paren_named(&["ref"], input) {
        let input = &mut expect_parens!(input)?;
        expect_ident_named!("ref", input)?;

        let nullable = if peek_ident_named("null", input) {
            input.next();
            quote![wat_defs::ty::Nullable::Nullable]
        } else {
            quote![wat_defs::ty::Nullable::NonNullable]
        };

        let heap_type = parse_heap_type(input)?;

        Ok(quote! {
            wat_defs::ty::RefType {
                null: #nullable,
                heap_type: #heap_type,
            };
        })
    } else {
        Err(error(input, "Invalid `RefType`"))
    }
}

fn parse_heap_type(input: ParseInput) -> Result<TokenStream> {

    todo!()
}
