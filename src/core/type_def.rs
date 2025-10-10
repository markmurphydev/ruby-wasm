use crate::core::array::ARRAY_UNITYPE_TYPE_IDENTIFIER;
use crate::core::{alist, array};
use crate::unitype::Unitype;
use crate::wasm::types::{ArrayType, CompType, FieldType, FuncType, HeapType, Mutability, Nullability, PackType, ParamType, ParamsType, RefType, ResultsType, StorageType, StructType, SubType, ValType};
use crate::wasm::{Finality, TypeDef};
use crate::CompileCtx;

pub const OBJECT_TYPE_IDENTIFIER: &str = "obj";
pub const CLASS_TYPE_IDENTIFIER: &str = "class";
pub const METHOD_TYPE_IDENTIFIER: &str = "method";

pub fn add_type_defs(ctx: &mut CompileCtx<'_>) {
    let mut type_defs = vec![
        string_type_def(ctx),
        object_type_def(ctx),
        class_type_def(ctx),
        method_type_def(ctx),
    ];
    type_defs.append(&mut array::array_type_defs(ctx));
    type_defs.append(&mut alist::alist_type_defs(ctx));

    for def in type_defs {
        ctx.module.type_def_arena.alloc(def);
    }
}

/// `(type $str (array i8))`
pub fn string_type_def(ctx: &mut CompileCtx<'_>) -> TypeDef {
    let ty = ArrayType {
        field: PackType::I8.into_field_type(),
    }
    .into_sub_type();

    TypeDef::new(ctx, Unitype::STRING_TYPE_IDENTIFIER, ty)
}


/// The wasm type-definition of a Ruby object.
/// `(type $obj (sub (struct (field $parent (mut (ref null $class))))))
pub fn object_type_def(ctx: &mut CompileCtx<'_>) -> TypeDef {
    let ty = SubType {
        is_final: Finality::NotFinal,
        supertypes: vec![],
        comp_type: StructType {
            fields: Box::new([
                ("parent".to_string(), mut_ref_null_class())
            ]),
        }.into_comp_type()
    };

    TypeDef::new(
        ctx,
        OBJECT_TYPE_IDENTIFIER,
        ty
    )
}

/// The wasm type-definition of a Ruby class.
/// Each defined class (`BasicObject`, `Class`, ...)
///     is a global of type $class
pub fn class_type_def(ctx: &mut CompileCtx<'_>) -> TypeDef {

    let ty = SubType {
        is_final: Finality::Final,
        supertypes: vec!["obj".to_string()],
        comp_type: CompType::Struct(StructType {
            fields: Box::new([
                ("parent".to_string(), mut_ref_null_class()),
                ("superclass".to_string(), mut_ref_null_class()),
                (
                    "name".to_string(),
                    FieldType::ref_identifier(Unitype::STRING_TYPE_IDENTIFIER.to_string()),
                ),
                (
                    "instance-methods".to_string(),
                    FieldType::ref_identifier(alist::alist_str_method().alist_type_identifier()),
                ),
            ]),
        }),
    };

    TypeDef::new(ctx, CLASS_TYPE_IDENTIFIER, ty)
}

pub fn method_params_type() -> ParamsType {
    Box::new([
        ParamType {
            name: "self".to_string(),
            ty: RefType::new_identifier(OBJECT_TYPE_IDENTIFIER.to_string()).into_val_type(),
        },
        ParamType {
            name: "args".to_string(),
            ty: RefType::new_identifier(ARRAY_UNITYPE_TYPE_IDENTIFIER.to_string())
                .into_val_type(),
        },
    ])
}

pub fn method_results_type() -> ResultsType {
    Box::new([Unitype::UNITYPE.into_result_type()])
}

/// The wasm type-definition of a Ruby method.
/// Each method definition is a function subtyping $method.
pub fn method_type_def(ctx: &mut CompileCtx<'_>) -> TypeDef {
    let ty = FuncType {
        params: method_params_type(),
        results: method_results_type(),
    };
    TypeDef::new(ctx, METHOD_TYPE_IDENTIFIER, ty.into_sub_type())
}

/// `(mut (ref null $class))`
/// Wasm global definitions can't cyclic,
///  so we set the initial $class fields to `ref.null`,
///  then tie them together in the `start` function.
fn mut_ref_null_class() -> FieldType {
    FieldType {
        mutability: Mutability::Mut,
        ty: StorageType::Val(ValType::Ref(RefType {
            nullable: Nullability::Nullable,
            heap_type: HeapType::Identifier(CLASS_TYPE_IDENTIFIER.to_string()),
        })),
    }
}
