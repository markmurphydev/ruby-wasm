use crate::CompileCtx;
use crate::unitype::Unitype;
use crate::wasm::TypeDef;
use crate::wasm::types::{ArrayType, FieldType, Mutability};

pub const ARRAY_UNITYPE_TYPE_IDENTIFIER: &str = "arr-unitype";

pub fn array_unitype(ctx: &mut CompileCtx<'_>) -> TypeDef {
    let ty = ArrayType {
        field: Unitype::UNITYPE.into_field_type(),
    }
    .into_sub_type();
    TypeDef::new(ctx, ARRAY_UNITYPE_TYPE_IDENTIFIER, ty)
}

pub fn array_type_defs(ctx: &mut CompileCtx<'_>) -> Vec<TypeDef> {
    vec![array_unitype(ctx)]
}
