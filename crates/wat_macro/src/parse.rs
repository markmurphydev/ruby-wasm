use proc_macro2::TokenStream;
use crate::parse::instr::parse_instr;
use crate::parse::parse_input::ParseStream;
use crate::result::Result;

mod instr;
mod parse_input;

pub fn parse_module(input: TokenStream) -> Result<TokenStream> {
    parse_instr(&mut ParseStream::new(input))
}