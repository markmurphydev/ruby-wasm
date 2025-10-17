//! Functions added to the base module
//! Strategy: Aiming for MVP functionality.
//!     Do everything as simply/composably as possible,
//!     and see if Binaryen can sort out the inlining and obvious optimizations.

mod alist;
mod array;
pub mod type_def;
pub mod class;
pub mod global;
mod method;
mod function;

use crate::CompileCtx;

pub fn add_core_items(ctx: &mut CompileCtx) {
    type_def::add_type_defs(ctx);
    global::add_globals(ctx);
    method::add_method_defs(ctx);
    function::add_functions(ctx);
}
