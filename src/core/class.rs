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
    pub superclass_name: String,
    pub instance_methods: Vec<Method>,
}

pub fn core_classes() -> Vec<Class> {
    
}