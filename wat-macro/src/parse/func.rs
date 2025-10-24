use crate::parse::parse_stream::ParseInput;
use crate::parse::util::*;
use crate::parse::{instr, ty};
use crate::result::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_func(input: ParseInput) -> Result<TokenStream> {
    // (func $<name>
    //  (export \"<name>\")
    //  (type <sym>)?
    //  (param $<name> <val_type>)*
    //  (result <val_type>)*
    //  (local $<name> <val_type>)*
    //  <instr>*)

    let (input, _) = &mut expect_open_paren_named(&["func"], input)?;
    let name = parse_name(input)?;

    let imported = {
        match expect_open_paren_named(&["import"], input) {
            Ok((mut input, _)) => {
                let module = expect_string_literal(&mut input)?;
                let name = expect_string_literal(&mut input)?;
                quote![ wat_defs::func::Imported::Imported(#module, #name) ]
            }
            Err(_) => quote![wat_defs::func::Imported::NotImported],
        }
    };

    let exported = {
        match expect_open_paren_named(&["export"], input) {
            Ok((mut input, _)) => {
                let name = expect_string_literal(&mut input)?;
                quote![ wat_defs::func::Exported::Exported(#name) ]
            }
            Err(_) => quote![wat_defs::func::Exported::NotExported],
        }
    };

    let type_use = match expect_open_paren_named(&["type"], input) {
        Ok((mut input, _)) => {
            let name = parse_name(&mut input)?;
            quote![ Some(#name) ]
        }
        Err(_) => quote![None],
    };
    let params = parse_params(input)?;
    let results = parse_results(input)?;
    let locals = parse_locals(input)?;
    let instrs = instr::parse_instr_seq(input)?;

    Ok(quote! {
        wat_defs::func::Func {
            name: #name,
            imported: #imported,
            exported: #exported,
            type_use: #type_use,
            params: #params,
            results: #results,
            locals: #locals,
            instrs: #instrs,
        }
    })
}

pub fn parse_params(input: ParseInput) -> Result<TokenStream> {
    parse_while_ok(input, parse_param)
}

fn parse_param(input: ParseInput) -> Result<TokenStream> {
    match expect_open_paren_named(&["param"], input) {
        Ok((mut input, _)) => {
            let input = &mut input;
            let name = parse_name(input)?;
            let ty = ty::parse_val_type(input)?;

            Ok(quote! {
                wat_defs::func::Param {
                    name: #name,
                    ty: #ty,
                }
            })
        }
        Err(_) => Err(error(input, "Expected `(param ...)")),
    }
}

pub fn parse_results(input: ParseInput) -> Result<TokenStream> {
    parse_while_ok(input, parse_result)
}

fn parse_result(input: ParseInput) -> Result<TokenStream> {
    match expect_open_paren_named(&["result"], input) {
        Ok((mut input, _)) => ty::parse_val_type(&mut input),
        Err(_) => Err(error(input, "Expected `(result ...)")),
    }
}

fn parse_locals(input: ParseInput) -> Result<TokenStream> {
    parse_while_ok(input, parse_local)
}

pub fn parse_local(input: ParseInput) -> Result<TokenStream> {
    match expect_open_paren_named(&["local"], input) {
        Ok((mut input, _)) => {
            let input = &mut input;
            let name = parse_name(input)?;
            let ty = ty::parse_val_type(input)?;

            Ok(quote! {
                wat_defs::func::Local {
                    name: #name,
                    ty: #ty,
                }
            })
        }
        Err(_) => Err(error(input, "Expected `(param ...)")),
    }
}

//
// #[cfg(test)]
// mod test {
//     use crate::parse::ParseStream;
//     use expect_test::expect;
//     use quote::quote;
//     use super::*;
//
//     #[test]
//     pub fn _func() {
//         let input: TokenStream = quote! {
//             (func $my_func
//                 (param $a i32) (param $b (ref $str)))
//         };
//         let actual = parse_func(&mut ParseStream::new(input)).unwrap().to_string();
//         let expected = expect![[""]];
//         expected.assert_eq(&actual);
//     }
// }
//
