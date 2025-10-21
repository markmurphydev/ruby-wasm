use expect_test::expect;
use ruby_wasm::run;

#[test]
fn array_literal() {
    let text = "[1, 2, 3]";
    let expected = expect![["[1, 2, 3]"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn array_indexing() {
    let text = "[1, 2, 3][1]";
    let expected = expect![["2"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn array_index_assignment() {
    let text = "$x = [1, 2, 3]\
    $x[1] = 0
    $x";
    let expected = expect![["[1, 0, 3]"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}
