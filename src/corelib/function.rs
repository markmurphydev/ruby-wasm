use crate::corelib::alist::{AListTypeDef, alist_str_method};
use crate::corelib::class;
use crate::corelib::class::Class;
use crate::corelib::type_def::{method_params_type, method_results_type, CLASS_TYPE_IDENTIFIER, METHOD_TYPE_IDENTIFIER, OBJECT_TYPE_IDENTIFIER};
use crate::unitype::Unitype;
use crate::wasm::function::{ExportStatus, Local};
use crate::wasm::types::{NumType, ParamType, ParamsType, RefType};
use crate::wasm::{BinaryOp, UnaryOp};
use crate::{CompileCtx, FunctionBuilder, InstrSeqBuilder};
use crate::corelib::array::ARRAY_UNITYPE_TYPE_IDENTIFIER;

pub fn add_functions(ctx: &mut CompileCtx<'_>) {
    add_start(ctx);
    add_str_eq(ctx);
    add_alist_str_method_get(ctx);
    add_call(ctx);
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
            .struct_set(ctx, CLASS_TYPE_IDENTIFIER.to_string(), "parent".to_string());
        // set class superclass
        if let Some(superclass_identifier) = superclass_identifier {
            instr_seq_builder
                .global_get(ctx, class_identifier.clone())
                .global_get(ctx, superclass_identifier)
                .struct_set(
                    ctx,
                    CLASS_TYPE_IDENTIFIER.to_string(),
                    "superclass".to_string(),
                );
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
            |ctx, instr_seq_builder| {},
        );
    instr_seq_builder
        .loop_(ctx, "for".to_string(), |ctx, instr_seq_builder| {
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
                    instr_seq_builder.i32_const(ctx, 1).return_(ctx);
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
                    instr_seq_builder
                        .local_get(ctx, "a_ch".to_string())
                        .local_get(ctx, "b_ch".to_string())
                        .binop(ctx, BinaryOp::I32Eq)
                        .unop(ctx, UnaryOp::I32Eqz);
                },
                |ctx, instr_seq_builder| {
                    instr_seq_builder.i32_const(ctx, 0).return_(ctx);
                },
                |_, _| {},
            );
            instr_seq_builder
                .local_get(ctx, "idx".to_string())
                .i32_const(ctx, 1)
                .binop(ctx, BinaryOp::I32Add)
                .local_set(ctx, "idx".to_string());
            instr_seq_builder.br(ctx, "for".to_string());
        })
        .unreachable(ctx);
    function_builder.finish(&mut ctx.module.funcs);
}

const ALIST_STR_METHOD_GET_IDENTIFIER: &str = "alist-str-method-get";

/// TODO: This should be genericized for any type of alist we have.
fn add_alist_str_method_get(ctx: &mut CompileCtx<'_>) {
    let params: ParamsType = Box::new([
        ParamType {
            name: "alist".to_string(),
            ty: RefType::new_identifier(alist_str_method().alist_type_identifier()).into_val_type(),
        },
        ParamType {
            name: "name".to_string(),
            ty: RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
    ]);
    let results =
        Box::new([RefType::new_identifier(METHOD_TYPE_IDENTIFIER.to_string()).into_result_type()]);
    let locals = vec![
        Local {
            identifier: "idx".to_string(),
            ty: NumType::I32.into_val_type(),
        },
        Local {
            identifier: "pair".to_string(),
            ty: RefType::new_identifier(alist_str_method().alist_pair_type_identifier())
                .into_val_type(),
        },
        Local {
            identifier: "key".to_string(),
            ty: RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
        Local {
            identifier: "val".to_string(),
            ty: RefType::new_identifier(METHOD_TYPE_IDENTIFIER.to_string()).into_val_type(),
        },
    ];
    let function_builder = FunctionBuilder::new(
        ctx,
        ALIST_STR_METHOD_GET_IDENTIFIER,
        ExportStatus::NotExported,
        None,
        params,
        results,
        locals,
    );
    let instr_seq_builder = function_builder.func_body();
    instr_seq_builder
        .i32_const(ctx, 0)
        .local_set(ctx, "idx".to_string());
    helper_add_for_in(
        ctx,
        instr_seq_builder,
        alist_str_method(),
        |ctx, instr_seq_builder| {
            instr_seq_builder.if_else(
                ctx,
                None,
                |ctx, instr_seq_builder| {
                    instr_seq_builder
                        .local_get(ctx, "key".to_string())
                        .local_get(ctx, "name".to_string())
                        .call(ctx, STR_EQ_IDENTIFIER.to_string());
                },
                |ctx, instr_seq_builder| {
                    instr_seq_builder.local_get(ctx, "val".to_string()).return_(ctx);
                },
                |_, _| {},
            );
        },
    );
    function_builder.finish(&mut ctx.module.funcs);
}

const CALL_IDENTIFIER: &str = "call";

fn add_call(ctx: &mut CompileCtx<'_>) {
    let params: ParamsType = Box::new([
        ParamType {
            name: "receiver".to_string(),
            ty: RefType::new_identifier(OBJECT_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
        ParamType {
            name: "message".to_string(),
            ty: RefType::new_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
        ParamType {
            name: "args".to_string(),
            ty: RefType::new_identifier(ARRAY_UNITYPE_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
    ]);
    let results = Box::new([Unitype::UNITYPE.into_result_type()]);
    let locals = vec![
        Local {
            identifier: "parent".to_string(),
            ty: RefType::new_identifier(CLASS_TYPE_IDENTIFIER.to_string()).into_val_type()
        },
        Local {
            identifier: "method".to_string(),
            ty: RefType::new_identifier(METHOD_TYPE_IDENTIFIER.to_string()).into_val_type()
        },
    ];
    let function_builder = FunctionBuilder::new(
        ctx,
        CALL_IDENTIFIER,
        ExportStatus::NotExported,
        None,
        params,
        results,
        locals
    );
    let instr_seq_builder = function_builder.func_body();

    instr_seq_builder.local_get(ctx, "receiver".to_string())
        .struct_get(ctx, OBJECT_TYPE_IDENTIFIER.to_string(), "parent".to_string())
        .unop(ctx, UnaryOp::RefAsNonNull)
        .local_set(ctx, "parent".to_string());

    instr_seq_builder.local_get(ctx, "parent".to_string())
        .struct_get(ctx, CLASS_TYPE_IDENTIFIER.to_string(), "instance-methods".to_string())
        .local_get(ctx, "message".to_string())
        .call(ctx, ALIST_STR_METHOD_GET_IDENTIFIER.to_string())
        .ref_cast(ctx, RefType::new_identifier(METHOD_TYPE_IDENTIFIER.to_string()))
        .local_set(ctx, "method".to_string());

    instr_seq_builder.local_get(ctx, "receiver".to_string())
        .local_get(ctx, "args".to_string())
        .local_get(ctx, "method".to_string())
        .call_ref(ctx, METHOD_TYPE_IDENTIFIER.to_string());

    function_builder.finish(&mut ctx.module.funcs);
}

fn helper_add_for_in(
    ctx: &mut CompileCtx<'_>,
    instr_seq_builder: InstrSeqBuilder,
    alist_type_def: AListTypeDef,
    body: impl FnOnce(&mut CompileCtx<'_>, &InstrSeqBuilder),
) {
    let alist_identifier = alist_type_def.alist_type_identifier();
    let pair_identifier = alist_type_def.alist_pair_type_identifier();

    instr_seq_builder
        .i32_const(ctx, 0)
        .local_set(ctx, "idx".to_string());
    instr_seq_builder
        .loop_(ctx, "for".to_string(), |ctx, instr_seq_builder| {
            instr_seq_builder.if_else(
                ctx,
                None,
                |ctx, instr_seq_builder| {
                    instr_seq_builder
                        .local_get(ctx, "idx".to_string())
                        .local_get(ctx, "alist".to_string())
                        .unop(ctx, UnaryOp::ArrayLen)
                        .binop(ctx, BinaryOp::I32Eq);
                },
                |ctx, instr_seq_builder| {
                    instr_seq_builder.unreachable(ctx);
                },
                |ctx, instr_seq_builder| {},
            );
            instr_seq_builder
                .local_get(ctx, "alist".to_string())
                .local_get(ctx, "idx".to_string())
                .array_get(ctx, alist_identifier.clone())
                .local_set(ctx, "pair".to_string());
            instr_seq_builder
                .local_get(ctx, "pair".to_string())
                .struct_get(ctx, pair_identifier.clone(), "key".to_string())
                .local_set(ctx, "key".to_string());
            instr_seq_builder
                .local_get(ctx, "pair".to_string())
                .struct_get(ctx, pair_identifier.clone(), "val".to_string())
                .local_set(ctx, "val".to_string());
            body(ctx, instr_seq_builder);
            instr_seq_builder
                .local_get(ctx, "idx".to_string())
                .i32_const(ctx, 1)
                .binop(ctx, BinaryOp::I32Add)
                .local_set(ctx, "idx".to_string());
            instr_seq_builder.br(ctx, "for".to_string());
        })
        .unreachable(ctx);
}
