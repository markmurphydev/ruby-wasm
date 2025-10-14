use derive_quote_to_tokens::ToTokens;
use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::parse_quote;

#[derive(Debug)]
pub enum NumType {
    I32,
}

impl ToTokens for NumType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            NumType::I32 => tokens.append::<TokenTree>(parse_quote! { UnfoldedInstr::Nop }),
        }
    }
}
