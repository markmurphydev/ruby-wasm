use crate::compiler::RUBY_TOP_LEVEL_FUNCTION_NAME;
use crate::corelib::add_core_items;
use crate::lexeme::LexemeKind;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::unitype::{Unitype, WasmtimeRefEq};
use crate::{CompileCtx, compiler, print_wat, run};
use wasmtime::{Config, Engine, Instance, Module, Store};
use wat_defs::module;

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

pub fn run_text(text: String) -> String {
    run_wat(compile_ctx_to_wat(&text_to_compile_ctx(text)))
}

pub fn text_to_compile_ctx(text: String) -> CompileCtx {
    let parser = Parser::new(Lexer::new(&text));
    let program = parser.parse();
    let module = module::Module::new();
    let mut ctx = CompileCtx::new(module);
    add_core_items(&mut ctx);
    compiler::compile(&mut ctx, &program);
    ctx
}

/// Writes out `module` as a .wat file, includes the corelib definitions,
/// and runs it.
pub fn compile_ctx_to_wat(ctx: &CompileCtx) -> String {
    print_wat::module_to_pretty(&ctx.module)
}

pub fn run_wat(wat: String) -> String {
    let mut config = Config::new();
    config.wasm_function_references(true).wasm_gc(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::new(&engine, wat).unwrap();
    // let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    if let Ok(top_level) =
        instance.get_typed_func::<(), WasmtimeRefEq>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
    {
        // Ruby main is `() -> (ref eq)`
        let res = top_level.call(&mut store, ()).unwrap();

        Unitype::parse_ref_eq(res, &mut store).to_pretty()
    } else if let Ok(top_level) =
        instance.get_typed_func::<(), i32>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
    {
        let res = top_level.call(&mut store, ()).unwrap();
        format!("{}", res)
    } else if let Ok(top_level) =
        instance.get_typed_func::<(), i64>(&mut store, RUBY_TOP_LEVEL_FUNCTION_NAME)
    {
        let res = top_level.call(&mut store, ()).unwrap();
        format!("{}", res)
    } else {
        panic!("Can't find RUBY_TOP_LEVEL_FUNCTION_NAME");
    }
}
