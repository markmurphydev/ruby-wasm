use std::mem;
use wat_defs::func::Func;
use wat_macro::wat;
use crate::CompileCtx;

pub fn add_imports(ctx: &mut CompileCtx) {
    ctx.module.funcs = [imports(), mem::take(&mut ctx.module.funcs)].concat();
}

pub fn imports() -> Vec<Func> {
    vec![
        js_i64_to_ref(),
    ]
}

fn js_i64_to_ref() -> Func {
    wat! {
        (func $js_i64_to_ref
            (import ,("i64".to_string()) ,("toRef".to_string()))
            (param $x i64)
            (result (ref null extern)))
    }
}
