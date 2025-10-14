use crate::ty::NumType;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for NumType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let res = match self {
            NumType::I32 => quote! { UnfoldedInstr::Nop },
        };
        res.to_tokens(tokens)
    }
}