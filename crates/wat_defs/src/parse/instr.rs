use crate::instr::{Instr, UnfoldedInstr};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::parse::ParseStream;
use syn::{parenthesized, Error, Ident, LitInt};
use crate::ty::NumType;

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
        };
        res.to_tokens(tokens);
    }
}

impl ToTokens for Instr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let this = quote!(wat_defs::instr::Instr);

        let mut instr = TokenStream::new();
        self.instr.to_tokens(&mut instr);

        let mut folded_instrs = TokenStream::new();
        folded_instrs.append_all(&self.folded_instrs);

        let res = quote! {
            #this {
                instr: #instr,
                folded_instrs: vec![ #folded_instrs ],
            }
        };
        res.to_tokens(tokens)
    }
}

impl syn::parse::Parse for UnfoldedInstr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_ident: Ident = input.parse()?;
        let name = name_ident.to_string();

        match name.as_str() {
            "nop" => Ok(UnfoldedInstr::Nop),
            "const.i32" => parse_const(NumType::I32, input),
            _ => Err(Error::new(name_ident.span(), format!("`{}` is not an instruction name.", name)))
        }
    }
}

fn parse_const(ty: NumType, input: ParseStream) -> syn::Result<UnfoldedInstr> {
    let val: LitInt = input.parse()?;
    let val = val.base10_parse()?;
    Ok(UnfoldedInstr::Const {
        ty,
        val,
    })
}

impl syn::parse::Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        parenthesized!(body in input);
        let instr: UnfoldedInstr = body.parse()?;
        Ok(Self { instr, folded_instrs: vec![] })
    }
}
