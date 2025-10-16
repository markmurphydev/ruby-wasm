use proc_macro2::TokenStream;
use crate::parse::instr::parse_instr;
use crate::parse::parse_stream::ParseStream;
use crate::parse::util::check_quasi_quote;
use crate::result::Result;

mod instr;
mod parse_stream;
mod ty;
mod util;

pub fn parse_module(input: TokenStream) -> Result<TokenStream> {
    let input = &mut ParseStream::new(input);
    check_quasi_quote!(input => { parse_instr(input) })
}