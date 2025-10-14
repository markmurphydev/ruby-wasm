use proc_macro2::TokenTree;
use crate::instr::{Instr, UnfoldedInstr};
use syn::parse::ParseStream;
use syn::{parenthesized, Error, Ident};

impl syn::parse::Parse for UnfoldedInstr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_ident: Ident = input.parse()?;
        let name = name_ident.to_string();

        eprintln!("name: {}", name);
        match name.as_str() {
            "nop" => Ok(UnfoldedInstr::Nop),
            "const.i32" => parse_const(name, input),
            _ => Err(Error::new(name_ident.span(), format!("{} is not an instruction name.", name)))
        }
        // if UnfoldedInstr::is_simple_instr(name.as_str()) {
        //     let name = name.to_case(Case::UpperCamel);
        //     Ident::new(&name, Span::call_site())
        // } else {
        // return Err(Error::new(name_ident.span(),
        //                       format!("{} is not an instruction name.", name)
        //     ));
        // };
        //
        // todo!()
    }
}

fn parse_const(name: String, input: ParseStream) -> syn::Result<UnfoldedInstr> {

    todo!()
}

impl syn::parse::Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        parenthesized!(body in input);
        eprintln!("{:?}", body);
        body.parse::<Ident>()?;
        eprintln!("{:?}", body);
        // let instr: UnfoldedInstr = body.parse()?;
        // eprintln!("{:?}", instr);
        Ok(Self { instr: UnfoldedInstr::Nop, folded_instrs: vec![] })
    }
}
