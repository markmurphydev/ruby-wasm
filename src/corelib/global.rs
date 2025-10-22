// use crate::CompileCtx;
// use crate::corelib::{class, method};
// use crate::corelib::array::array_unitype;
// use crate::unitype::Unitype;
// use crate::wasm::module::GlobalBuilder;
// use crate::wasm::types::{Mutability, RefType};

use crate::CompileCtx;
use crate::corelib::{class, method};
use crate::unitype::Unitype;
use wat_defs::global::Global;
use wat_defs::instr::Instr;
use wat_macro::wat;

pub fn add_globals(ctx: &mut CompileCtx) {
    let mut globals = vec![empty_args()];
    ctx.module.globals.append(&mut globals);
    add_string_defs(ctx);
    class::add_class_defs(ctx);
}

// (GLOBAL $EMPTY-ARGS (REF $ARR-UNITYPE) (ARRAY.NEW_FIXED $ARR-UNITYPE 0))
fn empty_args() -> Global {
    wat! {
       (global $empty_args
               (ref $arr_unitype)
               (array_new_fixed $arr_unitype 0))
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
    class::classes()
        .into_iter()
        .map(|c| c.name)
        .for_each(|s| add_string_def(ctx, s));
}

fn add_method_string_defs(ctx: &mut CompileCtx) {
    for method in method::methods() {
        add_string_def(ctx, method.name);
    }
}

fn add_string_def(ctx: &mut CompileCtx, string: String) {
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
