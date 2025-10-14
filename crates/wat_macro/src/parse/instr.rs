use crate::parse::parse_input::{ParseInput, ParseStream};
use crate::result::{Error, Result};
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use wat_defs::instr::{Instr, UnfoldedInstr};
use wat_defs::ty::NumType;

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_ident {
    ($input:expr) => {
        expect_ident_fn(file!(), line!(), $input)
    };
}

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_parens {
    ($input:expr) => {
        expect_parens_fn(file!(), line!(), $input)
    };
}

//         eprintln!("parse a: input={:?}", input);
//         let body;
//         parenthesized!(body in input);
//         eprintln!("parse b: input={:?}", input);
//
//         // so, for just the `if` instruction, the folded form is:
//         // ```
//         // (if <label>? <block_type>? <folded_instr>*
//         //   (then <instr>*)
//         //   (else <instr>*)?)
//         // ```
//         // so we need to parse folded instructions _first_, peeking 2 for `(then`
//         // if let ok(name) = input.fork().call(ident::parse_any) {
//         //     if name == "if" {
//         //         return parse_if(input);
//         //     }
//         // }
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
    let mut body = expect_parens!(input)?;
    let name = expect_ident!(&mut body)?;

    parse_instr_with_name(name, &mut body)
}

fn parse_instr_with_name(name: Ident, input: ParseInput) -> Result<TokenStream> {
    let instr = parse_unfolded_instr(name, input)?;
    parse_unfolded_instrs(input);

    Ok(quote! {
        wat_defs::instr::Instr {
            instr: #instr
            folded_instrs: vec![]
        }
    })
}

fn parse_unfolded_instr(name: Ident, input: ParseInput) -> Result<TokenStream> {
    let name = name.to_string();

    let res = match name.as_str() {
        "nop" => UnfoldedInstr::Nop,
        // "const.i32" => parse_const(NumType::I32, input)?,
        // "loop" => parse_loop(input)?,
        _ => {
            return Err(error(
                input,
                format!("`{}` is not an instruction name.", name),
            ));
        }
    };
    Ok(quote! { #res })
}

fn parse_folded_instrs(input: ParseInput) -> Result<TokenStream> {
    
}

/// Parses an unwrapped, undelimited sequence of instructions into `vec![...]`.
fn parse_instr_seq(input: ParseInput) -> Result<TokenStream> {
    let mut instrs = Vec::new();
    loop {
        match expect_parens!(input) {
            Ok(mut body) => {
                let body = &mut body;
                match expect_ident!(body) {
                    Ok(ident) if Instr::is_instr(&ident.to_string()) => instrs.push(parse_instr_with_name(ident, body)?),
                    _ => break
                }
            }
            Err(_) => break
        }
    }
    Ok(quote! { vec![ #(#instrs),* ] })
}

fn expect_ident_fn(file: &str, line: u32, input: ParseInput) -> Result<Ident> {
    match input.next() {
        Some(TokenTree::Ident(ident)) => Ok(ident),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected ident.", file, line),
        )),
    }
}

/// Expects a [Delimiter::Parenthesis] group, and returns the inner stream.
fn expect_parens_fn(file: &str, line: u32, input: ParseInput) -> Result<ParseStream> {
    match input.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            Ok(ParseStream::new(group.stream()))
        }
        _ => Err(error(
            input,
            format!("{}:{} -- Expected parens.", file, line),
        )),
    }
}

fn error<M>(input: ParseInput, message: M) -> Error
where
    M: Clone + Into<String>,
{
    eprintln!("error A -- message={}", message.clone().into());
    let span = input.current_span();
    Error::new(span, message.into())
}

// impl Parse for UnfoldedInstr {
//     fn parse(input: ParseStream) -> Result<Self> {
//         // name_ident can be a keyword like `loop`.
//         // Parse it as described in `syn` docs:
//         // https://docs.rs/syn/latest/syn/struct.Ident.html
//         let name_ident: Ident = input.call(Ident::parse_any)?;
//         let name = name_ident.to_string();
//
//         match name.as_str() {
//             "nop" => Ok(UnfoldedInstr::Nop),
//             "const.i32" => parse_const(NumType::I32, input),
//             "loop" => parse_loop(input),
//             _ => Err(Error::new(
//                 name_ident.span(),
//                 format!("`{}` is not an instruction name.", name),
//             )),
//         }
//     }
// }
//
// fn parse_const(ty: NumType, input: ParseStream) -> Result<UnfoldedInstr> {
//     let val: LitInt = input.parse()?;
//     let val = val.base10_parse()?;
//     Ok(UnfoldedInstr::Const { ty, val })
// }
//
// fn parse_loop(input: ParseStream) -> Result<UnfoldedInstr> {
//     input.parse::<Token![$]>()?;
//     let label: Ident = input.call(Ident::parse_any)?;
//     let label = label.to_string();
//     Ok(UnfoldedInstr::Loop { label })
// }
//
// impl Parse for Instr {
//     fn parse(input: ParseStream) -> Result<Self> {
//         eprintln!("parse a: input={:?}", input);
//         // syn::parse::parse_(input, delimiter::parenthesis).map(|(span, content)| parens {
//         //     token: token::paren(span),
//         //     content,
//         // })
//         // syn::pars
//         // let body;
//         let body = match syn::__private::parse_parens(&input) {
//             ok(parens) => {
//                 body = parens.content;
//                 parens.token
//             }
//             err(error) => {
//                 return err(error);
//             }
//         }
//         let body;
//         parenthesized!(body in input);
//         eprintln!("parse b: input={:?}", input);
//
//         // so, for just the `if` instruction, the folded form is:
//         // ```
//         // (if <label>? <block_type>? <folded_instr>*
//         //   (then <instr>*)
//         //   (else <instr>*)?)
//         // ```
//         // so we need to parse folded instructions _first_, peeking 2 for `(then`
//         // if let ok(name) = input.fork().call(ident::parse_any) {
//         //     if name == "if" {
//         //         return parse_if(input);
//         //     }
//         // }
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
//     }
// }
//
// /// pre: `input.next() == "if"`
// fn parse_if(input: ParseStream) -> Result<Instr> {
//     let name = input.call(Ident::parse_any)?;
//     if name != "if" {
//         unreachable!("parse_if: `name` != \"if\"")
//     }
//
//     let label = if input.peek(Token![$]) {
//         input.parse::<Token![$]>()?;
//         let ident = input.call(Ident::parse_any)?;
//         Some(ident.to_string())
//     } else {
//         None
//     };
//
//     let block_type: Option<BlockType> = input.parse().ok();
//
//     let folded_instrs = parse_instrs(input);
//
//     let then_block = {
//         let body;
//         braced!(body in input);
//         let name = body.call(Ident::parse_any)?;
//         if name != "then" {
//             return Err(Error::new(
//                 name.span(),
//                 format!(
//                     "{} {}: Expected \"then\", found {} .",
//                     file!(),
//                     line!(),
//                     name
//                 ),
//             ));
//         };
//
//         parse_instrs(&body)
//     };
//
//     Ok(Instr {
//         instr: UnfoldedInstr::If {
//             label,
//             block_type: None,
//             then_block,
//             else_block: vec![],
//         },
//         folded_instrs,
//     })
// }
//
// /// Parses an unwrapped, undelimited sequence of instructions.
// fn parse_instrs(input: ParseStream) -> Vec<Instr> {
//     let mut instrs = Vec::new();
//     loop {
//         eprintln!("parse_instrs A: instrs={:?} input={:?}", instrs, input);
//         let fork = input.fork();
//         let instr = fork.parse::<Instr>();
//         if let Ok(instr) = instr {
//             input.advance_to(&fork);
//             instrs.push(instr)
//         } else {
//             break;
//         }
//     }
//     instrs
// }
//
// fn parse_delimited<'a>(
//     input: &ParseBuffer<'a>,
//     delimiter: Delimiter,
// ) -> Result<(DelimSpan, ParseBuffer<'a>)> {
// }
