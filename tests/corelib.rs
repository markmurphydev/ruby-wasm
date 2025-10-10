//! Test that [ruby_wasm::corelib] (or `./generate-corelib.lisp` for now)
//! is producing Wasm that:
//! - Compiles and type-checks
//! - Runs correctly

use ruby_wasm::CompileCtx;
use ruby_wasm::unitype::Unitype;
use ruby_wasm::wasm::TypeDef;
use ruby_wasm::wasm::types::{
    ArrayType, CompType, FieldType, HeapType, Mutability, Nullability, RefType, StorageType,
    StructType, SubType, ValType,
};

/// Wraps `body` in a function definition, includes the corelib definitions,
/// and runs the file.
/// `body` must be the body of a wasm function `() -> (ref eq)`
fn run_main_fn_body(body: &str) -> Unitype {
    let main_fn = format!(
        "
    (func $__ruby_top_level_function
        (export \"__ruby_top_level_function\")
        (result (ref eq))
        {body})"
    );

    ruby_wasm::run::run_wat(main_fn)
}

#[test]
pub fn run_without_panicking() {
    // `run_main_fn_body` expects a function `() -> (ref eq)`, so just return `I31::const(0)`
    // let main_fn = "\
    // (ref.i31 (i32.const 0))
    // ";
    // let res = run_main_fn_body(main_fn);
    // println!("{}", res.to_pretty());
}
