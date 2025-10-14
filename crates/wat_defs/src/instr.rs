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

#[derive(Debug)]
pub struct Instr {
    pub instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}
