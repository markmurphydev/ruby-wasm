use expect_test::expect;

#[test]
fn add_pos_pos() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "1000 + 500";
    let expected = expect![["1500"]];
    let actual = ruby_wasm::run_ruby_program(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn add_neg_neg() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "1000 + 500";
    let expected = expect![["1500"]];
    let actual = ruby_wasm::run_ruby_program(text.to_owned());
    expected.assert_eq(&actual);
}
