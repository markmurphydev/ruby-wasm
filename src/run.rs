use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::unitype::{Unitype, WasmtimeRefEq};
use crate::wasm;
use wasmtime::{Config, Engine, Instance, Module, Store};

pub fn run(module: wasm::module::Module) {
    let wat = module.to_pretty();
    let mut config = Config::new();
    config.wasm_gc(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, wat);
    let module = module.unwrap();
    // let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]);

    let top_level = instance.unwrap().get_typed_func::<(), WasmtimeRefEq>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME).unwrap();
    let res = top_level.call(&mut store, ()).unwrap();

    let output = Unitype::parse_ref_eq(res, &store).to_pretty();

    println!("{:}", output);
}