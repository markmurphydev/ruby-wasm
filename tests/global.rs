use expect_test::expect;
use ruby_wasm::run;

#[test]
fn global_set() {
    let text = "$asdf = 44";
    let expected = expect![["nil"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn global_set_then_get() {
    let text = "$asdf = 44
    $asdf";
    let expected = expect![["44"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn mutate_global() {
    let text = "$asdf = 44
    $asdf = 100
    $asdf";
    let expected = expect![["100"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn add_to_global() {
    let text = "$asdf = 44
    $asdf = $asdf + 56
    $asdf";
    let expected = expect![["100"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}
