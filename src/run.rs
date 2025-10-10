use std::fs;
use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::unitype::{Unitype, WasmtimeRefEq};
use crate::{wasm, CompileCtx};
use wasmtime::{Config, Engine, Instance, Module, Store};
use crate::corelib::add_core_items;

/// Writes out `module` as a .wat file, includes the corelib definitions,
/// and runs it.
pub fn run_module(ctx: &mut CompileCtx<'_>) -> Unitype {
    let wat = ctx.module.to_pretty();
    run_wat(wat)
}

pub fn run_wat(wat: String) -> Unitype {
    let mut config = Config::new();
    config.wasm_function_references(true).wasm_gc(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, wat).unwrap();
    // let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]);

    let top_level = instance
        .unwrap()
        .get_typed_func::<(), WasmtimeRefEq>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
        .unwrap();
    let res = top_level.call(&mut store, ()).unwrap();

    Unitype::parse_ref_eq(res, &mut store)
}

/// Takes a .wat file, and produces a version with corelib definitions included.
fn include_corelib_definitions(wat: &str) -> String {
    let corelib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/core_generated.wat"));
    let mut wat = wat.to_string();
    wat.push_str(corelib);
    wat
}