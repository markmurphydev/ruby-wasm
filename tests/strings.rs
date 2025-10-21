use expect_test::expect;
use ruby_wasm::run;

#[test]
fn single_quote_string_roundtrip() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "\'22\'";
    let expected = expect![["\"22\""]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}