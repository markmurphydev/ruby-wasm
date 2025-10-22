use crate::parse::parse_stream::ParseInput;
use crate::parse::ty;
use crate::parse::util::*;
use crate::result::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub fn parse_type_def(input: ParseInput) -> Result<TokenStream> {
    let (mut input, _) = expect_open_paren_named(&["type"], input)?;
    let input = &mut input;
    let name = parse_name(input)?;
    let ty = ty::parse_sub_type(input)?;

    Ok(quote! {
        wat_defs::module::TypeDef {
            name: #name,
            ty: #ty
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ParseStream;
    use expect_test::expect;
    use quote::quote;

    #[test]
    pub fn type_def() {
        let input = quote! { (type $str (array i8)) };
        let actual = parse_type_def(&mut ParseStream::new(input))
            .unwrap()
            .to_string();
        let actual = &format!("{}", actual);
        let expected = expect![[
            r#"wat_defs :: module :: TypeDef { name : "str" . to_string () , ty : wat_defs :: ty :: SubType { is_final : wat_defs :: ty :: Final :: Final , supertypes : vec ! [] , comp_type : wat_defs :: ty :: CompType :: Array (wat_defs :: ty :: ArrayType { field_type : wat_defs :: ty :: FieldType { mutable : wat_defs :: ty :: Mutable :: Const , ty : wat_defs :: ty :: StorageType :: Pack (wat_defs :: ty :: PackType :: I8) , } }) , } }"#
        ]];
        expected.assert_eq(actual);
    }
}
