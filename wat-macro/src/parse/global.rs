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

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use proc_macro2::TokenStream;
    use quote::quote;
    use crate::parse::parse_item;

    fn parse_to_string(input: TokenStream) -> String {
        parse_item(input).unwrap().to_string()
    }

    #[test]
    pub fn global() {
        let input: TokenStream = quote! {
            (global $asdf
                    (mut (ref null eq))
                    (ref_null eq))
        };
        let actual = parse_to_string(input);
        let expected = expect![[r#"wat_defs :: global :: Global { name : "asdf" . to_string () . to_string () , ty : wat_defs :: ty :: GlobalType { mutable : wat_defs :: ty :: Mutable :: Mutable , val_type : wat_defs :: ty :: ValType :: Ref (wat_defs :: ty :: RefType { null : wat_defs :: ty :: Nullable :: Nullable , heap_type : wat_defs :: ty :: HeapType :: Abs (wat_defs :: ty :: AbsHeapType :: Eq) , }) , } , instr_seq : vec ! [wat_defs :: instr :: Instr { unfolded_instr : wat_defs :: instr :: UnfoldedInstr :: RefNull { ty : wat_defs :: ty :: HeapType :: Abs (wat_defs :: ty :: AbsHeapType :: Eq) } , folded_instrs : vec ! [] , }] , }"#]];
        expected.assert_eq(&actual);
    }

}