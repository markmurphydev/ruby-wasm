use crate::parse::parse_stream::{ParseInput, ParseStream};
use crate::result::{Error, Result};
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use wat_defs::instr::{Instr, UnfoldedInstr};
use wat_defs::ty::{BlockType, HeapType, Nullable, NumType, RefType, ValType};
use crate::parse::ty;
use crate::parse::util::*;

// so, for just the `if` instruction, the folded form is:
// ```
// (if <label>? <block_type>? <folded_instr>*
//   (then <instr>*)
//   (else <instr>*)?)
// ```
// so we need to parse folded instructions _first_, peeking 2 for `(then`
// if let ok(name) = input.fork().call(ident::parse_any) {
//     if name == "if" {
//         return parse_if(input);
//     }
// }
//
//         let instr: unfoldedinstr = body.parse()?;
//
//         // check for folded instrs
//         let folded_instrs = parse_instrs(input);
//
//         ok(self {
//             instr,
//             folded_instrs,
//         })

pub fn parse_instr(input: ParseInput) -> Result<TokenStream> {
    eprintln!("parse_instr -- input={:?}", input);
    let mut body = expect_parens!(input)?;
    let name = expect_ident!(&mut body)?;

    parse_instr_with_name(name, &mut body)
}

/// Pre:
fn parse_instr_with_name(name: Ident, input: ParseInput) -> Result<TokenStream> {
    eprintln!("parse_instr_with_name, name={}", name);

    eprintln!("parse_instr_with_name A -- {:?}", input);
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

    eprintln!("parse_instr_with_name B -- {:?}", input);
    let instr = parse_unfolded_instr(name, input)?;
    let folded_instrs = parse_instr_seq(input)?;

    Ok(quote! {
        wat_defs::instr::Instr {
            instr: #instr,
            folded_instrs: #folded_instrs,
        }
    })
}

/// Pre: Parens and `if` have been read.
fn parse_if(input: ParseInput) -> Result<TokenStream> {
    let label = if peek_dollar(input) {
        expect_dollar!(input)?;
        let ident = expect_ident!(input)?.to_string();
        quote![ Some(#ident.to_string()) ]
    } else {
        quote![ None ]
    };


    let block_type = if peek_open_paren_named(&["result", "type"], input) {
        let block_type = ty::parse_block_type(input);
        quote![ ]
    } else {
        quote![ None ]
    };

    let folded_instrs = parse_instr_seq(input)?;

    let then_block = {
        let input = &mut expect_parens!(input)?;
        expect_ident_named!("then", input)?;
        parse_instr_seq(input)?
    };

    let else_block = if peek_open_paren_named(&["else"], input) {
        let input = &mut expect_parens!(input)?;
        expect_ident_named!("else", input)?;
        parse_instr_seq(input)?
    } else {
        quote![ Vec::new() ]
    };

    Ok(quote! {
        wat_defs::instr::Instr {
            instr: wat_defs::instr::UnfoldedInstr::If {
                label: #label,
                block_type: None,
                then_block: #then_block,
                else_block: #else_block,
            },
            folded_instrs: #folded_instrs,
        }
    })
}

fn parse_unfolded_instr(name: Ident, input: ParseInput) -> Result<TokenStream> {
    let name = name.to_string();

    eprintln!("parse_unfolded_instr: name={}", name);
    let res = match name.as_str() {
        "nop" => quote!(wat_defs::instr::UnfoldedInstr::Nop),
        "const.i32" => parse_const(NumType::I32, input)?,
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

fn parse_const(ty: NumType, input: ParseInput) -> Result<TokenStream> {
    let val = expect_int_literal!(input)?;
    Ok(quote!{ wat_defs::instr::UnfoldedInstr::Const { ty: #ty, val: #val } })
}

fn parse_loop(input: ParseInput) -> Result<TokenStream> {
    eprintln!("parse_loop: input={:?}", input);
    expect_dollar!(input)?;
    let label: Ident = expect_ident!(input)?;
    let label = label.to_string();
    Ok(quote!{ wat_defs::instr::UnfoldedInstr::Loop { label: #label.to_string() } })
}

/// Parses an unwrapped, undelimited sequence of instructions into `vec![...]`.
fn parse_instr_seq(input: ParseInput) -> Result<TokenStream> {
    eprintln!("parse_instr_seq");
    let mut instrs = Vec::new();
    loop {
        match peek_parens(input) {
            Some(mut body) => {
                let body = &mut body;
                match peek_ident(body) {
                    Some(ident) if Instr::is_instr(&ident.to_string()) => {
                        let mut body = expect_parens!(input)?;
                        let body = &mut body;
                        let ident = expect_ident!(body)?;
                        instrs.push(parse_instr_with_name(ident, body)?)
                    }
                    _ => break,
                }
            }
            None => break,
        }
    }
    Ok(quote! { vec![ #(#instrs),* ] })
}

#[cfg(test)]
mod test {
    use expect_test::expect;
    use quote::quote;
    use super::*;

    #[test]
    pub fn the_if() {
        let input: TokenStream = quote! { (if $label (then (nop)) ) };
        let actual = parse_instr(&mut ParseStream::new(input)).unwrap().to_string();
        let expected = expect![[r#"wat_defs :: instr :: Instr { instr : wat_defs :: instr :: UnfoldedInstr :: If { label : Some ("label" . to_string ()) , block_type : None , then_block : vec ! [wat_defs :: instr :: Instr { instr : wat_defs :: instr :: UnfoldedInstr :: Nop , folded_instrs : vec ! [] , }] , else_block : Vec :: new () , } , folded_instrs : vec ! [] , }"#]];
        expected.assert_eq(&actual);
    }
}
