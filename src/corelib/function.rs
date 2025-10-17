// use crate::corelib::alist::{AListTypeDef, alist_str_method};
// use crate::corelib::class;
// use crate::corelib::class::Class;
// use crate::corelib::type_def::{method_params_type, method_results_type, CLASS_TYPE_IDENTIFIER, METHOD_TYPE_IDENTIFIER, OBJECT_TYPE_IDENTIFIER};
// use crate::unitype::Unitype;
// use crate::wasm::function::{ExportStatus, Local};
// use crate::wasm::types::{NumType, ParamType, ParamsType, RefType};
// use crate::wasm::{BinaryOp, UnaryOp};
// use crate::{CompileCtx, FunctionBuilder, InstrSeqBuilder};
// use crate::corelib::array::ARRAY_UNITYPE_TYPE_IDENTIFIER;

use wat_defs::instr::Instr;
use wat_defs::ty::BlockType;
use crate::CompileCtx;
use crate::corelib::class;
use crate::corelib::class::Class;
use wat_macro::wat;
use crate::corelib::alist::AListTypeDef;

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
    let mut instrs = vec![];
    let classes = class::classes();
    for class in classes {
        let class_identifier = class.identifier();
        let parent_identifier = Class::name_to_identifier(&class.parent_name);
        let superclass_identifier = class
            .superclass_name
            .map(|name| Class::name_to_identifier(&name));

        instrs.append(&mut wat! {
            (struct_set $class $parent
                (global_get ,(class_identifier.clone()))
                (global_get ,(parent_identifier)))
        });

        // set class superclass
        if let Some(superclass_identifier) = superclass_identifier {
            instrs.append(&mut wat! {
                (struct_set $class $superclass
                    (global_get ,(class_identifier))
                    (global_get ,(superclass_identifier)))
            })
        }
    }

    let start_fn = wat! {
        (func $_start
            ,(instrs))
    };

    ctx.module.funcs.push(start_fn);
    ctx.module.start_fn = Some("_start".to_string());
}

pub const STR_EQ_IDENTIFIER: &str = "str-eq";
/// `str-eq : (ref $str) (ref $str) -> Bool`
fn add_str_eq(ctx: &mut CompileCtx<'_>) {
    // if (a.len != b.len) { return false }
    // for (a_ch, b_ch) in zip(a, b) {
    //   if (a_ch != b_ch) { return false }
    // }
    // return true
    // instr_seq_builder
    //     .i32_const(ctx, 0)
    //     .local_set(ctx, "idx".to_string())
    //     .if_else(
    //         ctx,
    //         None,
    //         |ctx, instr_seq_builder| {
    //             instr_seq_builder
    //                 .local_get(ctx, "a".to_string())
    //                 .unop(ctx, UnaryOp::ArrayLen)
    //                 .local_get(ctx, "b".to_string())
    //                 .unop(ctx, UnaryOp::ArrayLen)
    //                 .binop(ctx, BinaryOp::I32Eq);
    //         },
    //         |ctx, instr_seq_builder| {
    //             instr_seq_builder.i32_const(ctx, 0).return_(ctx);
    //         },
    //         |ctx, instr_seq_builder| {},
    //     );
    // instr_seq_builder
    //     .loop_(ctx, "for".to_string(), |ctx, instr_seq_builder| {
    //         instr_seq_builder.if_else(
    //             ctx,
    //             None,
    //             |ctx, instr_seq_builder| {
    //                 instr_seq_builder
    //                     .local_get(ctx, "idx".to_string())
    //                     .local_get(ctx, "a".to_string())
    //                     .unop(ctx, UnaryOp::ArrayLen)
    //                     .binop(ctx, BinaryOp::I32Eq)
    //                     .unop(ctx, UnaryOp::I32Eqz);
    //             },
    //             |ctx, instr_seq_builder| {
    //                 instr_seq_builder.i32_const(ctx, 1).return_(ctx);
    //             },
    //             |_, _| {},
    //         );
    //         instr_seq_builder
    //             .local_get(ctx, "a".to_string())
    //             .local_get(ctx, "idx".to_string())
    //             .array_get_u(ctx, Unitype::STRING_TYPE_IDENTIFIER.to_string())
    //             .local_set(ctx, "a_ch".to_string());
    //         instr_seq_builder
    //             .local_get(ctx, "b".to_string())
    //             .local_get(ctx, "idx".to_string())
    //             .array_get_u(ctx, Unitype::STRING_TYPE_IDENTIFIER.to_string())
    //             .local_set(ctx, "b_ch".to_string());
    //         instr_seq_builder.if_else(
    //             ctx,
    //             None,
    //             |ctx, instr_seq_builder| {
    //                 instr_seq_builder
    //                     .local_get(ctx, "a_ch".to_string())
    //                     .local_get(ctx, "b_ch".to_string())
    //                     .binop(ctx, BinaryOp::I32Eq)
    //                     .unop(ctx, UnaryOp::I32Eqz);
    //             },
    //             |ctx, instr_seq_builder| {
    //                 instr_seq_builder.i32_const(ctx, 0).return_(ctx);
    //             },
    //             |_, _| {},
    //         );
    //         instr_seq_builder
    //             .local_get(ctx, "idx".to_string())
    //             .i32_const(ctx, 1)
    //             .binop(ctx, BinaryOp::I32Add)
    //             .local_set(ctx, "idx".to_string());
    //         instr_seq_builder.br(ctx, "for".to_string());
    //     })
    //     .unreachable(ctx);
    // function_builder.finish(&mut ctx.module.funcs);

    let res = wat! {
        (func $str_eq
            (param $a (ref $str))
            (param $b (ref $str))
            (result i32)
            (local $idx i32) (local $a_ch i32) (local $b_ch i32)
            // if (a.len != b.len) { return false }
            // for (a_ch, b_ch) in zip(a, b) {
            //   if (a_ch != b_ch) { return false }
            // }
            // return true
            (local_set $idx (const_i32 0))
            (if (i32_eqz (i32_eq (array_len (local_get $a))
                                 (array_len (local_get $b))))
                (then (return (const_i32 0))))
            (loop $for (result (ref eq))
                (if (i32_eq (local_get $idx)
                            (array_len (local_get $a)))
                    (then (return (const_i32 1))))
                (local_set $a_ch (array_get_u $str (local_get $a) (local_get $idx)))
                (local_set $b_ch (array_get_u $str (local_get $b) (local_get $idx)))
                (if (i32_eqz (i32_eq (local_get $a_ch)
                                     (local_get $b_ch)))
                    (then (return (const_i32 0))))
                (local_set $idx (i32_add (local_get $idx)
                                         (const_i32 1)))
                (br $for))
            (unreachable))
    };
    ctx.module.funcs.push(res);
}

const ALIST_STR_METHOD_GET_IDENTIFIER: &str = "alist-str-method-get";

/// TODO: This should be genericized for any type of alist we have.
fn add_alist_str_method_get(ctx: &mut CompileCtx<'_>) {
    let res = wat! {
        (func $alist_str_method_get
            (param $alist (ref $alist_str_method))
            (param $name (ref $str))
            (result (ref $method))
            (local $idx i32)
            (local $pair (ref $alist_str_method_pair))
            (local $key (ref $str))
            (local $val (ref $method))

            (local.set $idx (const_i32 0))
            (loop $for (result (ref eq))
                (if (i32_eq (local_get $idx)
                            (array_len (local_get $alist)))
                    (then (unreachable)))
                (local_set $pair
                    (array_get $alist_str_method
                        (local_get $alist)
                        (local_get $idx)))
                (local_set $key
                    (struct_get $alist_str_method_pair $key
                        (local_get $pair)))
                (local_set $val
                    (struct_get $alist_str_method_pair $val
                        (local_get $pair)))
                (if (call $str_eq
                        (local_get $key)
                        (local_get $name))
                    (then (return (local_get $val))))
                (local_set $idx
                    (i32_add (local_get $idx) (const_i32 1)))
                (br $for))
            (unreachable))
    };
    ctx.module.funcs.push(res);
}

fn add_call(ctx: &mut CompileCtx<'_>) {
    let res = wat! {
        (func $call
            (param $receiver (ref eq))
            (param $message (ref $str))
            (param $args (ref $arr_unitype))
            (result (ref eq))
            (local $receiver_obj (ref $obj))
            (local $parent (ref $class))
            (local $method (ref $method))

            (local_set $receiver_obj (ref_cast (ref $obj) (local_get $receiver)))
            (local_set $parent
                (ref_as_non_null
                    (struct_get $obj $parent
                        (local_get $receiver_obj))))
            (local_set $method
                (ref_cast (ref $method)
                    (call $alist_str_method_get
                        (struct_get $class $instance_methods
                            (local_get $parent))
                        (local_get $message))))
            (call_ref $method
                (local_get $receiver_obj)
                (local_get $args)
                (local_get $method))
        )
    };
    ctx.module.funcs.push(res);
}

fn helper_add_for_in(
    ctx: &mut CompileCtx<'_>,
    alist_type_def: AListTypeDef,
    body: Vec<Instr>
) -> Vec<Instr> {
    let alist_identifier = alist_type_def.alist_type_identifier();
    let pair_identifier = alist_type_def.alist_pair_type_identifier();

    wat! {
        (local_set $idx (const_i32 0))
        (loop $for
            (if (i32_eq (local_get $idx)
                        (array_len (local_get $alist)))
                (then (unreachable)))
            (local_set $pair
                (array_get ,(alist_identifier)
                    (local_get $alist)
                    (local_get $idx)))
            (local_set $key
                (struct_get ,(pair_identifier.clone()) $key
                    (local_get $pair)))
            (local_set $val
                (struct_get ,(pair_identifier.clone()) $val
                    (local_get $pair)))
            (block $body ,(body))
            (local_set $idx (i32_add (local_get $idx)
                                   (const_i32 1)))
            (br $for))
    }
}
