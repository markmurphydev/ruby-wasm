use crate::{CompileCtx, InstrSeqBuilder};
use crate::core::alist::alist_str_method;
use crate::core::global;
use crate::core::method::Method;
use crate::core::type_def::CLASS_TYPE_IDENTIFIER;
use crate::wasm::module::GlobalBuilder;
use crate::wasm::types::{Mutability, RefType};

/// A Ruby class. Compiles to:
/// - Definition of global string `$<CLASS_NAME>`
/// - Global definition of class data
/// - Definition of class methods
pub struct Class {
    pub name: String,
    /// The name of this class's class.
    /// `some_class.parent.new().type == some_class.type`
    pub parent_name: String,
    pub superclass_name: Option<String>,
    pub instance_methods: Vec<Method>,
}

impl Class {
    pub fn name_to_identifier(name: &str) -> String {
        format!("class-{}", name)
    }

    pub fn identifier(&self) -> String {
        Self::name_to_identifier(&self.name)
    }

    pub fn add_def(self, ctx: &mut CompileCtx<'_>) {
        let ty = RefType::new_identifier(CLASS_TYPE_IDENTIFIER.to_string()).into_global_type(Mutability::Const);
        let global_builder = GlobalBuilder::new(ctx.module, ty, self.identifier());
        let instr_seq_builder = global_builder.instr_seq();
        // Parent and superclass get ref.null for now.
        // We build the cyclic references in the _start function

        instr_seq_builder.ref_null(ctx, CLASS_TYPE_IDENTIFIER.to_string()); // parent
        instr_seq_builder.ref_null(ctx, CLASS_TYPE_IDENTIFIER.to_string()); // superclass
        instr_seq_builder.global_get(ctx, global::string_identifier(&self.name)); // name

        self.compile_methods_arr(ctx, &instr_seq_builder);

        instr_seq_builder.struct_new(ctx, CLASS_TYPE_IDENTIFIER.to_string());
        global_builder.finish(ctx);
    }

    fn compile_methods_arr(&self, ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder) {

        for method in &self.instance_methods {
            let alist_pair_type_identifier = alist_str_method().alist_pair_type_identifier();
            builder.global_get(ctx, method.name.clone()).ref_func(ctx, method.identifier())
                .struct_new(ctx, alist_pair_type_identifier);
        }
        let alist_type_identifier = alist_str_method().alist_type_identifier();
        let len = self.instance_methods.len().try_into().unwrap();
        builder.array_new_fixed(ctx, alist_type_identifier, len);
    }
}

/// The `Module` class.
fn module() -> Class {
    Class {
        name: "Module".to_string(),
        parent_name: "Class".to_string(),
        superclass_name: Some("Object".to_string()),
        instance_methods: vec![],
    }
}

/// The `Class` class.
fn class() -> Class {
    Class {
        name: "Class".to_string(),
        parent_name: "Class".to_string(),
        superclass_name: Some("Module".to_string()),
        instance_methods: vec![],
    }
}

/// The `BasicObject` class.
fn basic_object() -> Class {
    Class {
        name: "BasicObject".to_string(),
        parent_name: "Class".to_string(),
        superclass_name: None,
        instance_methods: vec![],
    }
}

/// The `BasicObject` class.
fn object() -> Class {
    Class {
        name: "Object".to_string(),
        parent_name: "Class".to_string(),
        superclass_name: Some("BasicObject".to_string()),
        instance_methods: vec![],
    }
}

/// A Vec of all classes defined in `core`.
pub fn classes() -> Vec<Class> {
    vec![
        module(),
        class(),
        basic_object(),
        object()
    ]
}

pub fn add_class_defs(ctx: &mut CompileCtx<'_>) {
    for class in classes() {
        class.add_def(ctx)
    }
}