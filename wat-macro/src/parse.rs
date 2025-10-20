use crate::parse::func::parse_func;
use crate::parse::global::parse_global;
use crate::parse::instr::parse_instr_seq;
use crate::parse::parse_stream::ParseStream;
use crate::parse::ty::parse_ref_type;
use crate::parse::type_def::parse_type_def;
use crate::parse::util::{check_quasi_quote, peek_open_paren_named};
use crate::parse::util::{parse_name, peek_sym};
use crate::result::Result;
use proc_macro2::TokenStream;

mod instr;
mod parse_stream;
mod ty;
mod util;
mod global;
mod func;
mod type_def;

// TODO: Parse typed
// pub trait Parse: Sized {
//     fn parse(input: TokenStream) -> Self;
// }
//
// pub fn parse<T: Parse>(input: TokenStream) -> T {
//     T::parse(input)
// }
//
// impl Parse for TypeDef {
//     fn parse(input: TokenStream) -> Self {
//         parse_
//     }
// }

pub fn parse_item(input: TokenStream) -> Result<TokenStream> {
    let input = &mut ParseStream::new(input);
    check_quasi_quote!(input => {
        if peek_open_paren_named(&["type"], input).is_some() {
            parse_type_def(input)
        } else if peek_open_paren_named(&["global"], input).is_some() {
            parse_global(input)
        } else if peek_open_paren_named(&["func"], input).is_some() {
            parse_func(input)
        } else if peek_open_paren_named(&["ref"], input).is_some() {
            parse_ref_type(input)
        } else if peek_sym(input).is_some() {
            parse_name(input)
        } else {
            parse_instr_seq(input)
        }
    })
}