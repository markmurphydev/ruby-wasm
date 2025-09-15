use std::fs;
use std::path::PathBuf;
use ruby_wasm::wat::WatPrinter;
use ruby_wasm::wasm::{Expr, Function, FunctionIdx, Instruction, Module};
use ruby_wasm::wasm::values::I32;

fn compare(test_name: &str, module: Module) {
    let mut expected_out_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    expected_out_file.push(format!("tests/output/{}", test_name));

    let expected_out =
        fs::read_to_string(expected_out_file).expect(&format!("output/{} should exist", test_name));

    let test_out = WatPrinter::new().print_module(&module);

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
            id: Some("main".to_string()),
            body: Expr(vec![Instruction::ConstI32(I32(0))]),
        }],
        start: Some(FunctionIdx::Id("main".to_string())),
    };

    compare("return_zero", return_zero);
}
