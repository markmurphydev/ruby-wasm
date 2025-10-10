use crate::core::class;
use crate::core::class::Class;
use crate::unitype::Unitype;
use crate::wasm::function::{ExportStatus, Local};
use crate::wasm::types::{NumType, ParamType, ParamsType, RefType};
use crate::wasm::{BinaryOp, UnaryOp};
use crate::{CompileCtx, FunctionBuilder};

pub fn add_functions(ctx: &mut CompileCtx<'_>) {
    add_start(ctx);
    add_str_eq(ctx);
}

/// The `start` function runs when the module is loaded.
/// We use it to set up cyclic object references:
/// - `Class.parent`
/// - `Class.superclass`
fn add_start(ctx: &mut CompileCtx<'_>) {
    let function_builder = FunctionBuilder::new(
        ctx,
        "start",
        ExportStatus::NotExported,
        None,
        Box::new([]),
        Box::new([]),
        vec![],
    );
    let instr_seq_builder = function_builder.func_body();

    let classes = class::classes();
    for class in classes {
        let class_identifier = class.identifier();
        let parent_identifier = Class::name_to_identifier(&class.parent_name);
        let superclass_identifier = class
            .superclass_name
            .map(|name| Class::name_to_identifier(&name));

        // Set class parent
        instr_seq_builder
            .global_get(ctx, class_identifier.clone())
            .global_get(ctx, parent_identifier)
            .struct_set(ctx, class_identifier.clone(), "parent".to_string());
        // set class superclass
        if let Some(superclass_identifier) = superclass_identifier {
            instr_seq_builder
                .global_get(ctx, class_identifier.clone())
                .global_get(ctx, superclass_identifier)
                .struct_set(ctx, class_identifier, "superclass".to_string());
        }
    }
    let start_func_id = function_builder.finish(&mut ctx.module.funcs);
    ctx.module.start = Some(start_func_id);
}

pub const STR_EQ_IDENTIFIER: &str = "str-eq";
/// `str-eq : (ref $str) (ref $str) -> Bool`
fn add_str_eq(ctx: &mut CompileCtx<'_>) {
    let params: ParamsType = Box::new([
        ParamType {
            name: "a".to_string(),
            ty: RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
        ParamType {
            name: "b".to_string(),
            ty: RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
    ]);
    let results = Box::new([NumType::I32.into_result_type()]);
    let locals = vec![
        Local {
            identifier: "idx".to_string(),
            ty: NumType::I32.into_val_type(),
        },
        Local {
            identifier: "a_ch".to_string(),
            ty: NumType::I32.into_val_type(),
        },
        Local {
            identifier: "b_ch".to_string(),
            ty: NumType::I32.into_val_type(),
        },
    ];
    let function_builder = FunctionBuilder::new(
        ctx,
        STR_EQ_IDENTIFIER,
        ExportStatus::NotExported,
        None,
        params,
        results,
        locals,
    );
    let instr_seq_builder = function_builder.func_body();

    // if (a.len != b.len) { return false }
    // for (a_ch, b_ch) in zip(a, b) {
    //   if (a_ch != b_ch) { return false }
    // }
    // return true
    instr_seq_builder
        .i32_const(ctx, 0)
        .local_set(ctx, "idx".to_string())
        .if_else(
            ctx,
            None,
            |ctx, instr_seq_builder| {
                instr_seq_builder
                    .local_get(ctx, "a".to_string())
                    .unop(ctx, UnaryOp::ArrayLen)
                    .local_get(ctx, "b".to_string())
                    .unop(ctx, UnaryOp::ArrayLen)
                    .binop(ctx, BinaryOp::I32Eq);
            },
            |ctx, instr_seq_builder| {
                instr_seq_builder.i32_const(ctx, 0).return_(ctx);
            },
            |ctx, instr_seq_builder| {
                instr_seq_builder.loop_(ctx, "for".to_string(), |ctx, instr_seq_builder| {
                    instr_seq_builder.if_else(
                        ctx,
                        None,
                        |ctx, instr_seq_builder| {
                            instr_seq_builder
                                .local_get(ctx, "idx".to_string())
                                .local_get(ctx, "a".to_string())
                                .unop(ctx, UnaryOp::ArrayLen)
                                .binop(ctx, BinaryOp::I32Eq)
                                .unop(ctx, UnaryOp::I32Eqz);
                        },
                        |ctx, instr_seq_builder| {
                            instr_seq_builder.i31_const(ctx, 1).return_(ctx);
                        },
                        |_, _| {},
                    );
                    instr_seq_builder
                        .local_get(ctx, "a".to_string())
                        .local_get(ctx, "idx".to_string())
                        .array_get_u(ctx, Unitype::STRING_TYPE_IDENTIFIER.to_string())
                        .local_set(ctx, "a_ch".to_string());
                    instr_seq_builder
                        .local_get(ctx, "b".to_string())
                        .local_get(ctx, "idx".to_string())
                        .array_get_u(ctx, Unitype::STRING_TYPE_IDENTIFIER.to_string())
                        .local_set(ctx, "b_ch".to_string());
                    instr_seq_builder.if_else(
                        ctx,
                        None,
                        |ctx, instr_seq_builder| {
                            instr_seq_builder.local_get(ctx, "a_ch".to_string())
                                .local_get(ctx, "b_ch".to_string())
                                .binop(ctx, BinaryOp::I32Eq)
                                .unop(ctx, UnaryOp::I32Eqz);
                        },
                        |ctx, instr_seq_builder| {
                            instr_seq_builder.i32_const(ctx, 0).return_(ctx);
                        },
                        |ctx, instr_seq_builder| {},
                    );
                    instr_seq_builder.local_get(ctx, "idx".to_string())
                        .i32_const(ctx, 1)
                        .binop(ctx, BinaryOp::I32Add)
                        .local_set(ctx, "idx".to_string());
                    instr_seq_builder.br(ctx, "for".to_string());
                });
            },
        )
        .unreachable(ctx);
    function_builder.finish(&mut ctx.module.funcs);
}
