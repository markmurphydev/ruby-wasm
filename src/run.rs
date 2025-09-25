use wasmtime::{Config, Engine, Instance, Linker, Module, Store};
use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::wasm;

pub fn run(module: wasm::module::Module) {
    let wat = module.to_pretty();
    let mut config = Config::new();
    config.wasm_gc(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, wat).unwrap();
    // let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]);
    let top_level = instance.unwrap().get_typed_func::<(), ()>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME).unwrap();
    top_level.call(&mut store, ()).unwrap();

    println!("Jesus.");
}