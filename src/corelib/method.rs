use crate::corelib::type_def;
use crate::corelib::type_def::{
    CLASS_TYPE_IDENTIFIER, METHOD_TYPE_IDENTIFIER, OBJECT_TYPE_IDENTIFIER,
};
use crate::unitype::Unitype;
use crate::wasm::function::ExportStatus;
use crate::wasm::types::RefType;
use crate::{CompileCtx, FunctionBuilder};

/// A Ruby method. Compiles to:
/// - Definition of function type `$<METHOD_FUNC_NAME>`
///     with signature `(self: Object, args: Array Unitype) -> Unitype`
/// - Definition of global string `$<METHOD_NAME>`
pub struct Method {
    pub name: String,
    pub add_method_def: fn(ctx: &mut CompileCtx<'_>),
}

impl Method {
    pub fn identifier(&self) -> String {
        format!("{}-{}", METHOD_TYPE_IDENTIFIER, self.name)
    }
}

const NEW_NAME: &str = "new";

pub fn new() -> Method {
    Method {
        name: NEW_NAME.to_string(),
        add_method_def: new_add_method_def,
    }
}

fn new_add_method_def(ctx: &mut CompileCtx<'_>) {
    let method_builder = method_builder(ctx, &new().identifier());
    let instr_seq_builder = method_builder.func_body();
    instr_seq_builder
        .local_get(ctx, "self".to_string())
        .ref_cast(
            ctx,
            RefType::new_identifier(CLASS_TYPE_IDENTIFIER.to_string()),
        )
        .struct_new(ctx, OBJECT_TYPE_IDENTIFIER.to_string());
    method_builder.finish(&mut ctx.module.funcs);
}

const NAME_NAME: &str = "name";

pub fn name() -> Method {
    Method {
        name: NAME_NAME.to_string(),
        add_method_def: name_add_method_def,
    }
}

fn name_add_method_def(ctx: &mut CompileCtx<'_>) {
    let method_builder = method_builder(ctx, &name().identifier());
    let instr_seq_builder = method_builder.func_body();
    instr_seq_builder
        .local_get(ctx, "self".to_string())
        .ref_cast(
            ctx,
            RefType::new_identifier(CLASS_TYPE_IDENTIFIER.to_string()),
        )
        .struct_get(ctx, CLASS_TYPE_IDENTIFIER.to_string(), "name".to_string());
    method_builder.finish(&mut ctx.module.funcs);
}

const CLASS_NAME: &str = "class";

pub fn class() -> Method {
    Method {
        name: CLASS_NAME.to_string(),
        add_method_def: class_add_method_def,
    }
}

fn class_add_method_def(ctx: &mut CompileCtx<'_>) {
    let method_builder = method_builder(ctx, &class().identifier());
    let instr_seq_builder = method_builder.func_body();
    // self.parent as (ref eq)
    instr_seq_builder
        .local_get(ctx, "self".to_string())
        .struct_get(
            ctx,
            OBJECT_TYPE_IDENTIFIER.to_string(),
            "parent".to_string(),
        )
        .ref_cast(ctx, Unitype::UNITYPE);
    method_builder.finish(&mut ctx.module.funcs);
}

/// Make a new FunctionBuilder with the $method param & return types.
fn method_builder(ctx: &mut CompileCtx<'_>, name: &str) -> FunctionBuilder {
    FunctionBuilder::new(
        ctx,
        name,
        ExportStatus::NotExported,
        Some(METHOD_TYPE_IDENTIFIER),
        type_def::method_params_type(),
        type_def::method_results_type(),
        vec![]
    )
}

pub fn methods() -> Vec<Method> {
    vec![new(), class(), name()]
}

pub fn add_method_defs(ctx: &mut CompileCtx<'_>) {
    for method in methods() {
        (method.add_method_def)(ctx)
    }
}
