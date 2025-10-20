use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::lexeme::LexemeKind;
use crate::lexer::Lexer;
use crate::unitype::{Unitype, WasmtimeRefEq};
use crate::{CompileCtx, print_wat};
use wasmtime::{Config, Engine, Instance, Module, Store};

pub fn lex(text: &str) -> String {
    let mut res = String::new();
    let mut lexer = Lexer::new(text);
    loop {
        let lexeme = lexer.next();
        res.push_str(&format!("{:?}\n", lexeme));
        if let LexemeKind::Eof = lexeme.kind {
            return res;
        }
    }
}

/// Writes out `module` as a .wat file, includes the corelib definitions,
/// and runs it.
pub fn run_module(ctx: &mut CompileCtx<'_>) -> String {
    let wat = print_wat::module_to_pretty(ctx.module);
    run_wat(wat)
}

pub fn run_wat(wat: String) -> String {
    let mut config = Config::new();
    config.wasm_function_references(true).wasm_gc(true);
    let mut engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, wat).unwrap();
    // let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    if let Ok(top_level) =
        instance.get_typed_func::<(), WasmtimeRefEq>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
    {
        // Ruby main is `() -> (ref eq)`
        let res = top_level.call(&mut store, ()).unwrap();
        Unitype::parse_ref_eq(res, &mut engine, &mut store).to_pretty()
    } else if let Ok(top_level) =
        instance.get_typed_func::<(), i32>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
    {
        let res = top_level.call(&mut store, ()).unwrap();
        format!("{}", res)
    } else if let Ok(top_level) =
        instance.get_typed_func::<(), i64>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME) {
        let res = top_level.call(&mut store, ()).unwrap();
        format!("{}", res)
    } else {
        panic!("Can't find RUBY_TOP_LEVEL_FUNCTION_NAME");
    }
}

// /// Takes a .wat file, and produces a version with corelib definitions included.
// fn include_corelib_definitions(wat: &str) -> String {
//     let corelib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/core_generated.wat"));
//     let mut wat = wat.to_string();
//     wat.push_str(corelib);
//     wat
// }
