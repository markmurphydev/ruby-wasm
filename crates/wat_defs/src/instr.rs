use crate::ty::NumType;
use derive_quote_to_tokens::ToTokens;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};
use syn::{parse_quote, Expr};

#[derive(Debug)]
pub enum UnfoldedInstr {
    /// No-op instruction.
    Nop,

    Const {
        ty: NumType,
        val: i64,
    },
}

impl UnfoldedInstr {
    /// Is `name` a "simple instruction" -- an instruction with no immediate args?
    pub fn is_simple_instr(name: &str) -> bool {
        match name {
            "nop" => true,
            _ => false,
        }
    }
}

impl ToTokens for UnfoldedInstr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let res = match self {
            UnfoldedInstr::Nop => quote! { UnfoldedInstr::Nop },
            UnfoldedInstr::Const { ty, val } => quote! {
                UnfoldedInstr::Const {
                    ty: #ty,
                    val: #val,
                }
            },
        };
        res.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct Instr {
    pub instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}

impl ToTokens for Instr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        eprintln!("A {:?}", tokens);

        let mut instr = TokenStream::new();
        self.instr.to_tokens(&mut instr);
        eprintln!("B {:?}", instr);

        let mut folded_instrs = TokenStream::new();
        folded_instrs.append_all(&self.folded_instrs);
        eprintln!("C {:?}", folded_instrs);

        let res = quote_spanned! { Span::mixed_site() =>
            $crate::Instr {
                instr: #instr,
                folded_instrs: vec![ #folded_instrs ],
            }
        };
        res.to_tokens(tokens)
    }
}
