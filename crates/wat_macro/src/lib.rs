use proc_macro::TokenStream;
use syn::Ident;
use syn::parse::{Parse, ParseStream, Result};

#[proc_macro]
pub fn wat(input: TokenStream) -> TokenStream {
    let _ = input;

    unimplemented!()
}
