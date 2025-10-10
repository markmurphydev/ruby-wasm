use crate::core::Method;

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