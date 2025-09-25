use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::wasm;
use wasmtime::{AnyRef, Config, Engine, Instance, Module, Rooted, Store};

type WasmUnitype = Rooted<AnyRef>;

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

    let top_level = instance.unwrap().get_typed_func::<(), WasmUnitype>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME).unwrap();
    let res = top_level.call(&mut store, ()).unwrap();
    // let top_level = instance.unwrap().get_func(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME).unwrap();
    // let mut res = vec![Val::AnyRef(None)];
    // top_level.call(&mut store, &[], &mut res).unwrap();
    // println!("{:?}", top_level.ty(&store));

    println!("{:?}", res.unwrap_i31(&store));
}