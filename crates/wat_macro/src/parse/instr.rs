use crate::result::{Error, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};

type TokenIter = proc_macro2::token_stream::IntoIter;

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

pub fn parse_instr(input: ParseStream) -> Result<TokenStream> {
    let mut input = input.into_iter();
    expect_parens(&mut input);
    input.next();

    todo!()
}

/// Expects a [Delimiter::Parenthesis] group, and returns the inner stream.
fn expect_parens(input: &mut TokenIter) -> Result<TokenStream> {
    match input.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            Ok(group.stream())
        }
        _ =>
    }
}

fn error(input: &TokenStream, message: String) -> Error {

    Error::new()
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
