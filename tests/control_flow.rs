use expect_test::expect;
use ruby_wasm::run;

#[test]
fn if_else() {
    let text = " if true then 1 else 0 end ";
    let expected = expect![["1"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}
