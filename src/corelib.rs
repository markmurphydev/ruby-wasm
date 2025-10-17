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
    // global::add_globals(ctx);
    // method::add_method_defs(ctx);
    // function::add_functions(ctx);
}

// /// ```wat
// /// (global $unitype-true (ref i31)
// ///     (ref.i31 (i32.const `Unitype::TRUE_BIT_PATTERN`)))
// /// ```
// fn add_unitype_true(ctx: &mut CompileCtx) {
//     let true_ = wat! {
//         (global $unitype_true (ref i31)
//           (ref_i31 (i32_const ,(Unitype::TRUE_BIT_PATTERN))))
//     };
//     // let builder = GlobalBuilder::new(
//     //     ctx.module,
//     //     Unitype::REF_I31.into_global_type(Mutability::Const),
//     //     "unitype-true".to_string(),
//     // );
//     // builder
//     //     .instr_seq()
//     //     .i31_const(ctx, Unitype::TRUE_BIT_PATTERN);
//     // builder.finish(ctx);
// }

// /// ```wat
// /// (global $unitype-false (ref i31)
// ///     (ref.i31 (i32.const `Unitype::FALSE_BIT_PATTERN`)))
// /// ```
// fn add_unitype_false(ctx: &mut CompileCtx) {
//     let builder = GlobalBuilder::new(
//         ctx.module,
//         Unitype::REF_I31.into_global_type(Mutability::Const),
//         "unitype-false".to_string(),
//     );
//     builder
//         .instr_seq()
//         .i31_const(ctx, Unitype::FALSE_BIT_PATTERN);
//     builder.finish(ctx);
// }
//
// // fn is_string(ctx: &mut CompileCtx<'_>) -> Function {
// //     let name = "is_string";
// //     let exported = ExportStatus::NotExported;
// //     let params = Box::new([ParamType {
// //         name: "s".to_string(),
// //         ty: Unitype::UNITYPE.into_val_type(),
// //     }]);
// //     let results = Box::new([ResultType(Unitype::UNITYPE.into_val_type())]);
// //     let mut builder = FunctionBuilder::new(ctx, name, exported, params, results);
// //
// //     builder.func_body().if_else(
// //         ctx,
// //         |ctx, builder| {
// //             builder.ref_test(ctx, Unitype::string_ref_type());
// //         },
// //         |ctx, builder| {
// //             builder.i31_const(ctx, Unitype::TRUE_BIT_PATTERN);
// //         },
// //         |ctx, builder| {
// //             builder.i31_const(ctx, Unitype::FALSE_BIT_PATTERN);
// //         },
// //     );
// //
// //     todo!()
// // }
//
// // fn string_eq(ctx: &mut CompileCtx<'_>) -> Function {
// //     // let params = Box::new([])
// //     todo!()
// // }
// //
// // fn alist_get(ctx: &mut CompileCtx<'_>) -> Function {
// //     // TODO -- needs to get STRING_TYPE...
// //     // let params = Box::new([]);
// //     // let results = Box::new([]);
// //     // let builder = FunctionBuilder::new(ctx, "alist_get", ExportStatus::NotExported, params, )
// //     todo!()
// // }
//
// /// UNITYPE -> bool
// fn add_is_false(ctx: &mut CompileCtx<'_>) {
//     let name = "is_false";
//     let exported = ExportStatus::NotExported;
//     let params = Box::new([ParamType {
//         name: "b".to_string(),
//         ty: Unitype::UNITYPE.into_val_type(),
//     }]);
//     let results = Box::new([ResultType(NumType::I32.into_val_type())]);
//     let builder = FunctionBuilder::new(ctx, name, exported, None, params, results, vec![]);
//     builder.func_body().if_else(
//         ctx,
//         Some(NumType::I32.into_block_type_result()),
//         |ctx, builder| {
//             builder
//                 .local_get(ctx, "b".to_string())
//                 .ref_test(ctx, Unitype::REF_I31);
//         },
//         |ctx, builder| {
//             builder
//                 .local_get(ctx, "b".to_string())
//                 .ref_cast(ctx, Unitype::REF_I31)
//                 .unop(ctx, UnaryOp::I31GetU)
//                 .global_get(ctx, "unitype-false".to_string())
//                 .unop(ctx, UnaryOp::I31GetU)
//                 .binop(ctx, BinaryOp::I32Eq);
//         },
//         |ctx, builder| {
//             builder.i32_const(ctx, 0);
//         },
//     );
//
//     builder.finish(&mut ctx.module.funcs);
// }
//
// #[cfg(test)]
// mod tests {
//     // TODO -- How to test this easily?
//     //  Can't use the normal Ruby-program-text input.
//
//     // // TODO -- test `is_string()`
//     // #[test]
//     // fn is_string() {
//     //     let mut module = Module::new();
//     //     let ctx = CompileCtx {
//     //         module: &mut module,
//     //     };
//     //
//     //     let test_str_builder = GlobalBuilder::new(ctx.module, "test_str".to_string());
//     //     // test_str_builder.instr_seq()
//     // }
//
//     // #[test]
//     // fn is_false() {
//     //     // TODO
//     //     let mut module = Module::new();
//     //     let ctx = CompileCtx {
//     //         module: &mut module,
//     //     };
//     //
//     //     // let test_str_builder = GlobalBuilder::new(ctx.module, "test_str".to_string());
//     //     // test_str_builder.instr_seq()
//     // }
// }
