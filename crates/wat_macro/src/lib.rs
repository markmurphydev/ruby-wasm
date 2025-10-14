use quote::ToTokens;
use syn::parse_macro_input;
use wat_defs::instr::Instr;

#[proc_macro]
pub fn wat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res: Instr = parse_macro_input!(input as Instr);
    res.to_token_stream().into()
}
