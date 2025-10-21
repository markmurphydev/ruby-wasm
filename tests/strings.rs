use expect_test::expect;

#[test]
fn single_quote_string_roundtrip() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "\'22\'";
    let expected = expect![["\"22\""]];
    let actual = ruby_wasm::run::text_to_compile_ctx(text.to_owned());
    expected.assert_eq(&actual);
}