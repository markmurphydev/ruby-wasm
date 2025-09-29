//! Functions added to the base module

use crate::unitype::Unitype;
use crate::wasm::function::ExportStatus;
use crate::wasm::module::Module;
use crate::wasm::types::{ParamType, ResultType};
use crate::wasm::{BinaryOp, UnaryOp};
use crate::{CompileCtx, FunctionBuilder};

pub fn add_core_items(module: &mut Module) -> CompileCtx<'_> {
    let mut ctx = CompileCtx { module };
    add_is_false(&mut ctx);
    ctx
}

// fn add_globals(module: Module) {
//     // TODO -- Name mangling or something...
//     // let globals = [
//     //     Global { name:  }
//     // ];
//     // for global in globals {
//     //     module.global_arena.alloc(global);
//     // }
// }

// fn is_string(ctx: &mut CompileCtx<'_>) -> Function {
//     let name = "is_string";
//     let exported = ExportStatus::NotExported;
//     let params = Box::new([ParamType {
//         name: "s".to_string(),
//         ty: Unitype::UNITYPE.into_val_type(),
//     }]);
//     let results = Box::new([ResultType(Unitype::UNITYPE.into_val_type())]);
//     let mut builder = FunctionBuilder::new(ctx, name, exported, params, results);
//
//     builder.func_body().if_else(
//         ctx,
//         |ctx, builder| {
//             builder.ref_test(ctx, Unitype::string_ref_type());
//         },
//         |ctx, builder| {
//             builder.i31_const(ctx, Unitype::TRUE_BIT_PATTERN);
//         },
//         |ctx, builder| {
//             builder.i31_const(ctx, Unitype::FALSE_BIT_PATTERN);
//         },
//     );
//
//     todo!()
// }

// fn string_eq(ctx: &mut CompileCtx<'_>) -> Function {
//     // let params = Box::new([])
//     todo!()
// }
//
// fn alist_get(ctx: &mut CompileCtx<'_>) -> Function {
//     // TODO -- needs to get STRING_TYPE...
//     // let params = Box::new([]);
//     // let results = Box::new([]);
//     // let builder = FunctionBuilder::new(ctx, "alist_get", ExportStatus::NotExported, params, )
//     todo!()
// }

/// UNITYPE -> UNITYPE_BOOL
fn add_is_false(ctx: &mut CompileCtx<'_>) {
    let name = "is_false";
    let exported = ExportStatus::NotExported;
    let params = Box::new([ParamType {
        name: "b".to_string(),
        ty: Unitype::UNITYPE.into_val_type(),
    }]);
    let results = Box::new([ResultType(Unitype::UNITYPE.into_val_type())]);
    let builder = FunctionBuilder::new(ctx, name, exported, params, results);
    builder.func_body().if_else(
        ctx,
        |ctx, builder| {
            builder
                .local_get(ctx, "b".to_string())
                .ref_test(ctx, Unitype::REF_I31);
        },
        |ctx, builder| {
            builder
                .local_get(ctx, "b".to_string())
                .ref_cast(ctx, Unitype::REF_I31)
                .unop(ctx, UnaryOp::I31GetU)
                .global_get(ctx, "false".to_string())
                .unop(ctx, UnaryOp::I31GetU)
                .binop(ctx, BinaryOp::I32Eq);
        },
        |ctx, builder| {
            builder.i32_const(ctx, 0);
        },
    );

    builder.finish(&mut ctx.module.funcs);
}

#[cfg(test)]
mod tests {
    use crate::wasm::module::{GlobalBuilder, Module};
    use crate::CompileCtx;

    // // TODO -- test `is_string()`
    // #[test]
    // fn is_string() {
    //     let mut module = Module::new();
    //     let ctx = CompileCtx {
    //         module: &mut module,
    //     };
    //
    //     let test_str_builder = GlobalBuilder::new(ctx.module, "test_str".to_string());
    //     // test_str_builder.instr_seq()
    // }

    #[test]
    fn is_false() {
        let mut module = Module::new();
        let ctx = CompileCtx {
            module: &mut module,
        };

        let test_str_builder = GlobalBuilder::new(ctx.module, "test_str".to_string());
        // test_str_builder.instr_seq()
    }
}
