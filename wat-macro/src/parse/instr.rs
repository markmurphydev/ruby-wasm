use crate::parse::parse_stream::ParseInput;
use crate::parse::ty;
use crate::parse::util::*;
use crate::result::Result;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use wat_defs::instr::Instr;
use wat_defs::ty::NumType;

pub fn parse_instr(input: ParseInput) -> Result<TokenStream> {
    check_quasi_quote!(input => {
        let (mut input, name) = expect_open_paren_ident(input)?;
        let input = &mut input;

        // so, for just the `if` instruction, the folded form is:
        // ```
        // (if <label>? <block_type>? <folded_instr>*
        //   (then <instr>*)
        //   (else <instr>*)?)
        // ```
        // so we need to parse folded instructions _first_, peeking 2 for `(then`
        if name == "if" {
            return parse_if(input);
        }

        let instr = parse_unfolded_instr(name, input)?;
        let folded_instrs = parse_instr_seq(input)?;

        Ok(quote! {
            wat_defs::instr::Instr {
                unfolded_instr: #instr,
                folded_instrs: #folded_instrs,
            }
        })
    })
}

/// Pre: Parens and `if` have been read.
fn parse_if(input: ParseInput) -> Result<TokenStream> {
    let label = match expect_sym(input) {
        Ok(name) => {
            let name = name.to_string();
            quote![ Some(#name.to_string()) ]
        }
        Err(_) => quote![None],
    };

    let block_type = match ty::parse_block_type(input) {
        Ok(res) => quote![ Some(#res) ],
        Err(_) => quote![None],
    };

    let folded_instrs = parse_instr_seq(input)?;

    let then_block = {
        let (mut input, _) = expect_open_paren_named(&["then"], input)?;
        parse_instr_seq(&mut input)?
    };

    let else_block = match expect_open_paren_named(&["else"], input) {
        Ok((mut input, _)) => parse_instr_seq(&mut input)?,
        Err(_) => quote![Vec::new()],
    };

    Ok(quote! {
        wat_defs::instr::Instr {
            unfolded_instr: wat_defs::instr::UnfoldedInstr::If {
                label: #label,
                block_type: #block_type,
                then_block: #then_block,
                else_block: #else_block,
            },
            folded_instrs: #folded_instrs,
        }
    })
}

/// Pre: `(<name>` has been consumed.
fn parse_unfolded_instr(name: Ident, input: ParseInput) -> Result<TokenStream> {
    let path = quote![wat_defs::instr::UnfoldedInstr];
    let name = name.to_string();

    eprintln!("parse_unfolded_instr: name={}", name);
    let res = match name.as_str() {
        "nop" => quote!(wat_defs::instr::UnfoldedInstr::Nop),
        "const_i32" => parse_const(NumType::I32, input)?,
        "i32_eqz" => quote![ #path::I32Eqz ],
        "i32_eq" => quote![ #path::I32Eq ],
        "i32_add" => quote![ #path::I32Add ],
        "const_i64" => parse_const(NumType::I64, input)?,
        "br" => {
            let label = parse_name(input)?;
            quote![ #path::Br { label: #label } ]
        }
        "br_if" => {
            let label = parse_name(input)?;
            quote![ #path::BrIf { label: #label } ]
        }
        "return" => quote![ #path::Return ],
        "block" => parse_block(input)?,
        "loop" => parse_loop(input)?,
        "if" => return Err(error(input, "panic!: Can't parse unfolded if.")),
        "ref_null" => {
            let ty = parse_name(input)?;
            quote![ #path::RefNull { ty: #ty } ]
        }
        "ref_func" => {
            let name = parse_name(input)?;
            quote![ #path::RefFunc { name: #name } ]
        }
        "ref_i31" => quote![ #path::RefI31 ],
        "ref_as_non_null" => quote![ #path::RefAsNonNull ],
        "ref_cast" => {
            let ty = ty::parse_ref_type(input)?;
            quote![ #path::RefCast { ty: #ty } ]
        }
        "call" => {
            let func = parse_name(input)?;
            quote![ #path::Call { func: #func } ]
        }
        "call_ref" => {
            let type_idx = parse_name(input)?;
            quote![ #path::CallRef { type_idx: #type_idx } ]
        }
        "local_get" => {
            let name = parse_name(input)?;
            quote![ #path::LocalGet { name: #name } ]
        }
        "local_set" => {
            let name = parse_name(input)?;
            quote![ #path::LocalSet { name: #name } ]
        }
        "global_get" => {
            let name = parse_name(input)?;
            quote![ #path::GlobalGet { name: #name } ]
        }
        "global_set" => {
            let name = parse_name(input)?;
            quote![ #path::GlobalSet { name: #name } ]
        }
        "array_new_fixed" => {
            let type_idx = parse_name(input)?;
            let len = expect_int_literal(input)?;
            quote! {
                #path::ArrayNewFixed {
                    type_idx: #type_idx,
                    len: #len,
                }
            }
        }
        "array_get" => {
            let ty = parse_name(input)?;
            quote![ #path::ArrayGet { ty: #ty } ]
        }
        "array_get_u" => {
            let ty = parse_name(input)?;
            quote![ #path::ArrayGetU { ty: #ty } ]
        }
        "array_len" => quote![ #path::ArrayLen ],
        "struct_new" => {
            let ty = parse_name(input)?;
            quote![ #path::StructNew { ty: #ty } ]
        }
        "struct_get" => {
            let ty = parse_name(input)?;
            let field = parse_name(input)?;
            quote![ #path::StructGet { ty: #ty, field: #field } ]
        }
        "struct_set" => {
            let ty = parse_name(input)?;
            let field = parse_name(input)?;
            quote![ #path::StructSet { ty: #ty, field: #field } ]
        }
        "unreachable" => quote![ #path::Unreachable ],
        _ => {
            return Err(error(
                input,
                format!("`{}` is not an instruction name.", name),
            ));
        }
    };
    Ok(res)
}

/// Pre: `input.next()` is an int literal.
fn parse_const(ty: NumType, input: ParseInput) -> Result<TokenStream> {
    let ty = num_type_to_tokens(ty);
    let val = expect_int_literal(input)?;
    Ok(quote! { wat_defs::instr::UnfoldedInstr::Const { ty: #ty, val: #val } })
}

fn parse_block(input: ParseInput) -> Result<TokenStream> {
    let label = parse_name(input)?;
    // let block_type = match ty::parse_block_type(input) {
    //     Ok(res) => quote![ Some(#res) ],
    //     Err(_) => quote![None],
    // };
    Ok(quote! { wat_defs::instr::UnfoldedInstr::Block { label: #label } })
}

fn parse_loop(input: ParseInput) -> Result<TokenStream> {
    let label = parse_name(input)?;
    let block_type = match ty::parse_block_type(input) {
        Ok(res) => quote![ Some(#res) ],
        Err(_) => quote![None],
    };
    Ok(quote! { wat_defs::instr::UnfoldedInstr::Loop { label: #label, block_type: #block_type } })
}

/// Parses an unwrapped, undelimited sequence of instructions into `vec![...]`.
pub fn parse_instr_seq(input: ParseInput) -> Result<TokenStream> {
    // I think we can check for a quote at the start for the vec![] case,
    // and then check each iteration for the single item case?
    check_quasi_quote!(input => {
        // Can't use `parse_while_ok`, because it will mis-identify `(then ...)` / `(else ...)` as instrs.
        let mut instrs = Vec::new();
        loop {
            if let Ok(res) = expect_quasi_quote(input) {
                instrs.push(res);
            };
            match peek_open_paren_ident(input) {
                Some((_, ident)) if Instr::is_instr(&ident.to_string()) => {
                    let res = parse_instr(input)?;
                    instrs.push(res);
                }
                _ => break,
            }
        }
        Ok(quote! { vec![ #(#instrs),* ] })
    })
}

fn num_type_to_tokens(ty: NumType) -> TokenStream {
    let path = quote![wat_defs::ty::NumType];
    match ty {
        NumType::I32 => quote![ #path::I32 ],
        NumType::I64 => quote![ #path::I64 ],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ParseStream;
    use expect_test::expect;
    use quote::quote;

    #[test]
    pub fn _if() {
        let input: TokenStream = quote! { (if $label (then (nop)) ) };
        let actual = parse_instr(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let expected = expect![[
            r#"wat_defs :: instr :: Instr { unfolded_instr : wat_defs :: instr :: UnfoldedInstr :: If { label : Some ("label" . to_string ()) , block_type : None , then_block : vec ! [wat_defs :: instr :: Instr { unfolded_instr : wat_defs :: instr :: UnfoldedInstr :: Nop , folded_instrs : vec ! [] , }] , else_block : Vec :: new () , } , folded_instrs : vec ! [] , }"#
        ]];
        expected.assert_eq(&actual);
    }
}
