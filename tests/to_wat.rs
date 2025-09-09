use std::fs;
use std::path::PathBuf;
use ruby_wasm::wat::Printer;
use ruby_wasm::wasm::{Expr, Function, FunctionIndex, Instruction, Module};
use ruby_wasm::wasm::values::Integer;

fn compare(test_name: &str, module: Module) {
    let mut expected_out_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    expected_out_file.push(format!("tests/output/{}", test_name));

    let expected_out =
        fs::read_to_string(expected_out_file).expect(&format!("output/{} should exist", test_name));

    let test_out = Printer::new().print_module(&module);

    assert_eq!(expected_out, test_out);
}

#[test]
pub fn empty_module() {
    let empty_module = Module {
        functions: vec![],
        start: None,
    };

    compare("empty_module", empty_module);
}

#[test]
pub fn return_zero() {
    let return_zero = Module {
        functions: vec![Function {
            name: Some("main".to_string()),
            body: Expr(vec![Instruction::ConstI32(Integer(0))]),
        }],
        start: Some(FunctionIndex::Name("main".to_string())),
    };

    compare("return_zero", return_zero);
}
