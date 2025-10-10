use crate::CompileCtx;
use crate::core::class;
use crate::unitype::Unitype;
use crate::wasm::module::GlobalBuilder;
use crate::wasm::types::{Mutability, RefType};

pub fn add_globals(ctx: &mut CompileCtx<'_>) {
    add_string_defs(ctx);
}

/// Add string definitions from:
/// - Class names
/// - Method names
fn add_string_defs(ctx: &mut CompileCtx<'_>) {
    add_class_string_defs(ctx);
}

fn add_class_string_defs(ctx: &mut CompileCtx<'_>) {
    class::classes()
        .into_iter()
        .map(|c| c.name)
        .for_each(|s| add_string_def(ctx, s));
}

fn add_string_def(ctx: &mut CompileCtx<'_>, string: String) {
    let ref_str = RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string());
    let identifier = format!("{}-{}", Unitype::STRING_TYPE_IDENTIFIER, &string);
    let global_builder = GlobalBuilder::new(
        ctx.module,
        ref_str.into_global_type(Mutability::Const),
        identifier,
    );
    let instr_seq_builder = global_builder.instr_seq();
    let bytes = string.as_bytes();
    for &byte in bytes {
        instr_seq_builder.i32_const(ctx, byte.into());
    }
    instr_seq_builder.array_new_fixed(
        ctx,
        Unitype::STRING_TYPE_IDENTIFIER.to_string(),
        bytes.len().try_into().unwrap(),
    );
    global_builder.finish(ctx);
}
