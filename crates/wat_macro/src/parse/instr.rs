use crate::parse::parse_input::{ParseInput, ParseStream};
use crate::result::{Error, Result};
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use wat_defs::instr::{Instr, UnfoldedInstr};
use wat_defs::ty::NumType;

/// `(input: ParseInput) -> Result<i64>`
macro_rules! expect_int_literal {
    ($input:expr) => {
        expect_int_literal_fn(file!(), line!(), $input)
    };
}

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_ident {
    ($input:expr) => {
        expect_ident_fn(file!(), line!(), $input)
    };
}

/// `(name: &str, input: ParseInput) -> Result<i64>`
macro_rules! expect_ident_named {
    ($name:expr, $input:expr) => {
        expect_ident_named_fn(file!(), line!(), $name, $input)
    };
}

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_parens {
    ($input:expr) => {
        expect_parens_fn(file!(), line!(), $input)
    };
}

/// `(input: ParseInput) -> Result<()>`
macro_rules! expect_dollar {
    ($input:expr) => {
        expect_dollar_fn(file!(), line!(), $input)
    };
}

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
    if let Some(mut body) = peek_parens(input) {
        eprintln!("PEEK");
        if let Some(name) = peek_ident(&mut body) {
            eprintln!("IDENT");
            if name == "if" {
                return parse_if(input);
            }
        }
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

fn parse_if(input: ParseInput) -> Result<TokenStream> {
    let input = &mut expect_parens!(input)?;
    expect_ident_named!("if", input)?;

    let label = if peek_dollar(input) {
        expect_dollar!(input)?;
        let ident = expect_ident!(input)?.to_string();
        Some(ident)
    } else {
        None
    };


    // if peek_open_paren_named(&["result", "type"], input) {
    //     let block_type = parse_block_type(input);
    // }

    let folded_instrs = parse_instr_seq(input)?;

    let then_block = {
        let input = &mut expect_parens!(input)?;
        expect_ident_named!("then", input)?;
        parse_instr_seq(input)?
    };

    Ok(quote! {
        Instr {
            instr: wat_defs::instr::UnfoldedInstr::If {
                label: #label,
                block_type: None,
                then_block: #then_block,
                else_block: vec![],
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

fn expect_int_literal_fn(file: &str, line: u32, input: ParseInput) -> Result<i64> {
    match input.next() {
        Some(TokenTree::Literal(lit)) => lit
            .to_string()
            .parse::<i64>()
            .map_err(|err| error(input, format!("{}", err))),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected int literal.", file, line),
        )),
    }
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

fn expect_ident_named_fn(file: &str, line: u32, name: &str, input: ParseInput) -> Result<Ident> {
    match input.next() {
        Some(TokenTree::Ident(ident)) if ident == name => Ok(ident),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected ident named {}.", file, line, name),
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

fn peek_parens(input: ParseInput) -> Option<ParseStream> {
    match input.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            Some(ParseStream::new(group.stream()))
        }
        _ => None,
    }
}

fn peek_ident(input: ParseInput) -> Option<Ident> {
    match input.peek() {
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => None,
    }
}

fn peek_dollar(input: ParseInput) -> bool {
    match input.peek() {
        Some(TokenTree::Punct(punct)) => punct.as_char() == '$',
        _ => false,
    }
}

fn peek_open_paren_named(names: &[&str], input: ParseInput) -> bool {
    if let Some(mut body) = peek_parens(input) {
        if let Some(ident) = peek_ident(&mut body) {
            return names.iter().any(|name| ident == name)
        }
    }
    false
}

fn expect_dollar_fn(file: &str, line: u32, input: ParseInput) -> Result<()> {
    match input.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '$' => Ok(()),
        _ => Err(error(input, format!("{}:{} -- Expected `$`.", file, line))),
    }
}

fn error<M>(input: ParseInput, message: M) -> Error
where
    M: Clone + Into<String>,
{
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
