use crate::ty::{BlockType, NumType};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Error;

impl ToTokens for NumType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let res = match self {
            NumType::I32 => quote! { UnfoldedInstr::Nop },
        };
        res.to_tokens(tokens)
    }
}

impl Parse for BlockType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // let body;
        // braced!(body in input);
        // Check if we have `(type ...)` or `(result ...)`.
        Err(Error::new(
                input.span(),
            "aaa"
        ))
    }
}