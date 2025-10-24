use crate::CompileCtx;
use std::mem;
use wat_defs::func::Func;
use wat_macro::wat;

pub fn add_imports(ctx: &mut CompileCtx) {
    ctx.module.funcs = [imports(), mem::take(&mut ctx.module.funcs)].concat();
}

pub fn imports() -> Vec<Func> {
    vec![js_i64_to_ref(), js_arr_new(), js_arr_push()]
}

fn js_i64_to_ref() -> Func {
    wat! {
        (func $js_i64_to_ref
            (import ,("i64".to_string()) ,("toRef".to_string()))
            (param $x i64)
            (result (ref null extern)))
    }
}

fn js_arr_new() -> Func {
    wat! {
        (func $js_arr_new
            (import ,("arr".to_string()) ,("new".to_string()))
            (result (ref null extern)))
    }
}

fn js_arr_push() -> Func {
    wat! {
        (func $js_arr_push
            (import ,("arr".to_string()) ,("push".to_string()))
            (param $arr (ref null extern))
            (param $val (ref null extern)))
    }
}
