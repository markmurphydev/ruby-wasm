use crate::parse::parse_stream::ParseInput;
use crate::result::Result;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use wat_defs::instr::Instr;
use wat_defs::ty::NumType;
use crate::parse::ty;
use crate::parse::util::*;

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
        Err(_) => quote![ None ]
    };

    let block_type = match ty::parse_block_type(input) {
        Ok(res) => quote![ Some(#res) ],
        Err(_) => quote![ None ]
    };

    let folded_instrs = parse_instr_seq(input)?;

    let then_block = {
        let (mut input, _) = expect_open_paren_named(&["then"], input)?;
        parse_instr_seq(&mut input)?
    };

    let else_block = match expect_open_paren_named(&["else"], input) {
        Ok((mut input, _)) => {
            parse_instr_seq(&mut input)?
        }
        Err(_) => quote![ Vec::new() ]
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
    let name = name.to_string();

    eprintln!("parse_unfolded_instr: name={}", name);
    let res = match name.as_str() {
        "nop" => quote!(wat_defs::instr::UnfoldedInstr::Nop),
        "const_i32" => parse_const(NumType::I32, input)?,
        "loop" => parse_loop(input)?,
        "if" => return Err(error(input, "panic!: Can't parse unfolded if.")),
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
    Ok(quote!{ wat_defs::instr::UnfoldedInstr::Const { ty: #ty, val: #val } })
}

fn parse_loop(input: ParseInput) -> Result<TokenStream> {
    let label = expect_sym(input)?.to_string();
    Ok(quote!{ wat_defs::instr::UnfoldedInstr::Loop { label: #label.to_string() } })
}

/// Parses an unwrapped, undelimited sequence of instructions into `vec![...]`.
pub fn parse_instr_seq(input: ParseInput) -> Result<TokenStream> {
    eprintln!("parse_instr_seq");
    let mut instrs = Vec::new();
    loop {
        match peek_open_paren_ident(input) {
            Some((_, ident)) if Instr::is_instr(&ident.to_string()) => {
                let res = parse_instr(input)?;
                instrs.push(res);
            }
            _ => break
        }
    }
    Ok(quote! { vec![ #(#instrs),* ] })
}

fn num_type_to_tokens(ty: NumType) -> TokenStream {
    let path = quote![ wat_defs::ty::NumType ];
    match ty {
        NumType::I32 => quote![ #path::I32 ]
    }
}

#[cfg(test)]
mod test {
    use crate::parse::ParseStream;
    use expect_test::expect;
    use quote::quote;
    use super::*;

    #[test]
    pub fn _if() {
        let input: TokenStream = quote! { (if $label (then (nop)) ) };
        let actual = parse_instr(&mut ParseStream::new(input)).unwrap().to_string();
        let expected = expect![[r#"wat_defs :: instr :: Instr { unfolded_instr : wat_defs :: instr :: UnfoldedInstr :: If { label : Some ("label" . to_string ()) , block_type : None , then_block : vec ! [wat_defs :: instr :: Instr { unfolded_instr : wat_defs :: instr :: UnfoldedInstr :: Nop , folded_instrs : vec ! [] , }] , else_block : Vec :: new () , } , folded_instrs : vec ! [] , }"#]];
        expected.assert_eq(&actual);
    }
}
