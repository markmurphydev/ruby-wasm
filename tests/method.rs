use expect_test::expect;
use ruby_wasm::run;

#[test]
fn method_def() {
    let text = "
            def x(n)
                n
            end
        ";
    let expected = expect![["nil"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn def_no_args() {
    let text = "
            def x()
                22
            end
        ";
    let expected = expect![["nil"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn def_multiple_args() {
    let text = "
            def x(a, b)
                a + b
            end
        ";
    let expected = expect![["nil"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}

#[test]
fn def_then_call() {
    let text = "
            def x(n)
                n
            end
            x(22)
        ";
    let expected = expect![["22"]];
    let actual = run::run_text(text.to_owned());
    expected.assert_eq(&actual);
}
