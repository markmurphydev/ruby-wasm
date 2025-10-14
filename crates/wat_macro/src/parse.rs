use proc_macro2::TokenStream;
use crate::parse::instr::parse_instr;
use crate::result::Result;

mod instr;
mod buffer;
mod syn;

pub fn parse_module(input: TokenStream) -> Result<TokenStream> {
    parse_instr(input)
}