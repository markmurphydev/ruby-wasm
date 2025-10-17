use proc_macro2::TokenStream;
use quote::{quote};
use crate::parse::{ty, instr};
use crate::parse::parse_stream::ParseInput;
use crate::parse::util::*;
use crate::result::Result;

pub fn parse_global(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["global"], input)?;
    let input = &mut input;

    let id = parse_name(input)?;
    let ty = ty::parse_global_type(input)?;
    let instr_seq = instr::parse_instr_seq(input)?;

    Ok(quote!{
        wat_defs::global::Global {
            name: #id.to_string(),
            ty: #ty,
            instr_seq: #instr_seq,
        }
    })
}