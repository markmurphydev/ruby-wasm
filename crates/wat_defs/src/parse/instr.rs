//! We implement [ToTokens] so the wat defs can be used in `quote!`.
//! We implement [Parse] to convert from `wat!` syntax to wat structures.
//! We do both of these here to dodge the orphan rule...

use crate::instr::{Instr, UnfoldedInstr};
use crate::ty::NumType;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident, LitInt, parenthesized, token, Token};

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

impl Parse for UnfoldedInstr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // name_ident can be a keyword like `loop`.
        // Parse it as described in `syn` docs:
        // https://docs.rs/syn/latest/syn/struct.Ident.html
        let name_ident: Ident = input.call(Ident::parse_any)?;
        let name = name_ident.to_string();

        match name.as_str() {
            "nop" => Ok(UnfoldedInstr::Nop),
            "const.i32" => parse_const(NumType::I32, input),
            "loop" => parse_loop(input),
            _ => Err(Error::new(
                name_ident.span(),
                format!("`{}` is not an instruction name.", name),
            )),
        }
    }
}

fn parse_const(ty: NumType, input: ParseStream) -> syn::Result<UnfoldedInstr> {
    let val: LitInt = input.parse()?;
    let val = val.base10_parse()?;
    Ok(UnfoldedInstr::Const { ty, val })
}

fn parse_loop(input: ParseStream) -> syn::Result<UnfoldedInstr> {
    input.parse::<Token![$]>()?;
    let label: Ident = input.call(Ident::parse_any)?;
    eprintln!("A");
    let label = label.to_string();
    eprintln!("B");
    Ok(UnfoldedInstr::Loop { label })
}

impl Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        parenthesized!(body in input);
        let instr: UnfoldedInstr = body.parse()?;

        // Check for folded instrs
        let mut folded_instrs = Vec::new();
        loop {
            if body.peek(token::Paren) {
                let instr: Instr = body.parse()?;
                folded_instrs.push(instr);
            } else {
                break;
            }
        }

        Ok(Self {
            instr,
            folded_instrs,
        })
    }
}
