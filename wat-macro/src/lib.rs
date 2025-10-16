pub(crate) mod parse;
pub(crate) mod result;

use proc_macro2::TokenStream;

#[proc_macro]
pub fn wat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = parse::parse_module(input.into())
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()));
    res.into()
}
