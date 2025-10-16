//! We implement [ToTokens] so the wat defs can be used in `quote!`.
//! We implement [Parse] to convert from `wat!` syntax to wat structures.
//! We do both of these here to dodge the orphan rule...

use crate::instr::{Instr, UnfoldedInstr};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for UnfoldedInstr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let this = quote!(wat_defs::instr::UnfoldedInstr);
        let res = match self {
            UnfoldedInstr::Nop => quote! { #this::Nop },
            UnfoldedInstr::Const { ty, val } => quote! {
                #this::Const {
                    ty: #ty,
                    val: #val,
                }
            },
            UnfoldedInstr::Loop { label } => quote! {
                #this::Loop {
                    label: #label.to_string(),
                }
            },
            _ => panic!()
        };
        res.to_tokens(tokens);
    }
}

impl ToTokens for Instr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Instr {
            instr,
            folded_instrs,
        } = self;
        let this = quote!(wat_defs::instr::Instr);
        let instr = instr.to_token_stream();
        let folded_instrs = vec_to_token_stream(folded_instrs);

        let res = quote! {
            #this {
                instr: #instr,
                folded_instrs: vec![ #folded_instrs ],
            }
        };
        res.to_tokens(tokens)
    }
}

fn vec_to_token_stream(vec: &Vec<impl ToTokens>) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(vec);
    stream
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
//         eprintln!("parse A: input={:?}", input);
//         // syn::parse::parse_(input, Delimiter::Parenthesis).map(|(span, content)| Parens {
//         //     token: token::Paren(span),
//         //     content,
//         // })
//         // syn::pars
//         // let body;
//         let body = match syn::__private::parse_parens(&input) {
//             Ok(parens) => {
//                 body = parens.content;
//                 parens.token
//             }
//             Err(error) => {
//                 return Err(error);
//             }
//         }
//         let body;
//         parenthesized!(body in input);
//         eprintln!("parse B: input={:?}", input);
//
//         // So, for just the `if` instruction, the folded form is:
//         // ```
//         // (if <label>? <block_type>? <folded_instr>*
//         //   (then <instr>*)
//         //   (else <instr>*)?)
//         // ```
//         // So we need to parse folded instructions _first_, peeking 2 for `(then`
//         // if let Ok(name) = input.fork().call(Ident::parse_any) {
//         //     if name == "if" {
//         //         return parse_if(input);
//         //     }
//         // }
//
//         let instr: UnfoldedInstr = body.parse()?;
//
//         // Check for folded instrs
//         let folded_instrs = parse_instrs(input);
//
//         Ok(Self {
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
