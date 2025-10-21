use expect_test::expect;
use ruby_wasm::run;

#[test]
fn negative_literal() {
    let text = "-22";
    let expected = expect![["-22"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn add_pos_pos() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "1000 + 500";
    let expected = expect![["1500"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn add_neg_neg() {
    // TODO -- lex ` '2\'2' ` correctly
    let text = "-1000 + -500";
    let expected = expect![["-1500"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}


#[test]
fn and_or() {
    // && should have tighter binding
    let text = "false && true || true";
    let expected = expect![["true"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn lt() {
    let text = "1 < 2 && -2 < -1";
    let expected = expect![["true"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}
