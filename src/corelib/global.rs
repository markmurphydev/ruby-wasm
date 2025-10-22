use crate::CompileCtx;
use crate::corelib::{class, method};
use crate::unitype::Unitype;
use wat_defs::global::Global;
use wat_defs::instr::Instr;
use wat_macro::wat;

pub fn add_globals(ctx: &mut CompileCtx) {
    let mut globals = vec![main(), empty_args()];
    ctx.module.globals.append(&mut globals);
    add_string_defs(ctx);
}

// (GLOBAL $EMPTY-ARGS (REF $ARR-UNITYPE) (ARRAY.NEW_FIXED $ARR-UNITYPE 0))
fn empty_args() -> Global {
    wat! {
       (global $empty_args
               (ref $arr_unitype)
               (array_new_fixed $arr_unitype 0))
    }
}

/// Top-level `main` object, instantiated in _start to `Object#new()`
fn main() -> Global {
    wat! {
        (global $main
                (mut (ref $obj))
                (struct_new $obj (ref_null $class)))
    }
}

/// Add string definitions from:
/// - Class names
/// - Method names
fn add_string_defs(ctx: &mut CompileCtx) {
    add_class_string_defs(ctx);
    add_method_string_defs(ctx);
}

fn add_class_string_defs(ctx: &mut CompileCtx) {
    let class_names: Vec<_> = ctx.classes.iter().map(|c| c.name.clone()).collect();
    for name in class_names {
        add_string_def(ctx, name);
    }
}

fn add_method_string_defs(ctx: &mut CompileCtx) {
    let method_names: Vec<_> = ctx.methods.iter().map(|m| m.name.clone()).collect();
    for name in method_names {
        add_string_def(ctx, name);
    }
}

pub fn add_string_def(ctx: &mut CompileCtx, string: String) {
    let name = string_identifier(&string);
    let bytes: Vec<Instr> = string
        .as_bytes()
        .iter()
        .map(|b| wat![(const_i32, (*b as i64))])
        .flatten()
        .collect();
    let len = bytes.len();

    let res = wat! {
        (global ,(name)
            (ref $str)
            (array_new_fixed $str ,(len.try_into().unwrap())
                ,(bytes)))
    };

    ctx.module.globals.push(res);
}

pub fn string_identifier(string: &str) -> String {
    format!("{}_{}", Unitype::STRING_TYPE_IDENTIFIER, &string)
}
