// use crate::CompileCtx;
// use crate::corelib::{class, method};
// use crate::corelib::array::array_unitype;
// use crate::unitype::Unitype;
// use crate::wasm::module::GlobalBuilder;
// use crate::wasm::types::{Mutability, RefType};
// 
// pub fn add_globals(ctx: &mut CompileCtx<'_>) {
//     add_string_defs(ctx);
//     add_empty_args(ctx);
//     class::add_class_defs(ctx);
// }
// 
// // (GLOBAL $EMPTY-ARGS (REF $ARR-UNITYPE) (ARRAY.NEW_FIXED $ARR-UNITYPE 0))
// fn add_empty_args(ctx: &mut CompileCtx<'_>) {
//     let global_builder = GlobalBuilder::new(
//         ctx.module,
//         RefType::new_identifier("arr-unitype".to_string()).into_global_type(Mutability::Const),
//         "empty-args".to_string(),
//     );
//     let instr_seq_builder = global_builder.instr_seq();
//     instr_seq_builder.array_new_fixed(ctx, "arr-unitype".to_string(), 0);
//     global_builder.finish(ctx);
// }
// 
// /// Add string definitions from:
// /// - Class names
// /// - Method names
// fn add_string_defs(ctx: &mut CompileCtx<'_>) {
//     add_class_string_defs(ctx);
//     add_method_string_defs(ctx);
// }
// 
// fn add_class_string_defs(ctx: &mut CompileCtx<'_>) {
//     class::classes()
//         .into_iter()
//         .map(|c| c.name)
//         .for_each(|s| add_string_def(ctx, s));
// }
// 
// fn add_method_string_defs(ctx: &mut CompileCtx<'_>) {
//     for method in method::methods() {
//         add_string_def(ctx, method.name);
//     }
// }
// 
// fn add_string_def(ctx: &mut CompileCtx<'_>, string: String) {
//     let ref_str = RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string());
//     let identifier = string_identifier(&string);
//     let global_builder = GlobalBuilder::new(
//         ctx.module,
//         ref_str.into_global_type(Mutability::Const),
//         identifier,
//     );
//     let instr_seq_builder = global_builder.instr_seq();
//     let bytes = string.as_bytes();
//     for &byte in bytes {
//         instr_seq_builder.i32_const(ctx, byte.into());
//     }
//     instr_seq_builder.array_new_fixed(
//         ctx,
//         Unitype::STRING_TYPE_IDENTIFIER.to_string(),
//         bytes.len().try_into().unwrap(),
//     );
//     global_builder.finish(ctx);
// }
// 
// pub fn string_identifier(string: &str) -> String {
//     format!("{}-{}", Unitype::STRING_TYPE_IDENTIFIER, &string)
// }