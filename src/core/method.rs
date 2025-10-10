use crate::CompileCtx;
use crate::core::type_def::METHOD_TYPE_IDENTIFIER;

/// A Ruby method. Compiles to:
/// - Definition of function type `$<METHOD_FUNC_NAME>`
///     with signature `(self: Object, args: Array Unitype) -> Unitype`
/// - Definition of global string `$<METHOD_NAME>`
pub struct Method {
    pub name: String,
    pub add_method_def: fn(ctx: CompileCtx<'_>, name: String)
}

impl Method {
    pub fn identifier(&self) -> String {
        format!("{}-{}", METHOD_TYPE_IDENTIFIER, self.name)
    }
}