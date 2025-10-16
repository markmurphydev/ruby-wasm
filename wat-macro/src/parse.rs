use crate::parse::global::parse_global;
use proc_macro2::TokenStream;
use crate::parse::instr::parse_instr;
use crate::parse::parse_stream::ParseStream;
use crate::parse::util::{check_quasi_quote, peek_open_paren_named};
use crate::result::Result;

mod instr;
mod parse_stream;
mod ty;
mod util;
mod global;
mod func;

pub fn parse_item(input: TokenStream) -> Result<TokenStream> {
    let input = &mut ParseStream::new(input);
    check_quasi_quote!(input => {
        if peek_open_paren_named(&["global"], input).is_some() {
            parse_global(input)
        } else if peek_open_paren_named(&["func"], input).is_some() {
            parse_func(input)
        } else {
            parse_instr(input)
        }
    })
}