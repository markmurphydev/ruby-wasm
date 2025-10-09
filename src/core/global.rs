use crate::CompileCtx;
use crate::core::class;

pub fn add_globals(ctx: &mut CompileCtx<'_>) {
}

/// Add string definitions from:
/// - Class names
/// - Method names
fn string_defs() {
}

fn class_string_defs() {
    class::core_classes();
    
    todo!()
}
