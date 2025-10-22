use crate::CompileCtx;
use crate::corelib::global::string_identifier;
use crate::corelib::method;
use crate::corelib::method::Method;
use wat_defs::global::Global;
use wat_defs::instr::Instr;
use wat_macro::wat;

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
        format!("class_{}", name)
    }

    pub fn identifier(&self) -> String {
        Self::name_to_identifier(&self.name)
    }

    pub fn def(self) -> Global {
        // Parent and superclass get ref.null for now.
        // We build the cyclic references in the _start function

        wat! {
            (global ,(self.identifier())
                    (ref $class)
                    (struct_new $class
                                (ref_null $class)               // .parent
                                (ref_null $class)               // .superclass
                                (global_get ,(string_identifier(&self.name)))       // .name
                                ,(self.methods_arr()))) // .instance-methods
        }
    }

    fn methods_arr(&self) -> Instr {
        let struct_defs: Vec<_> = self
            .instance_methods
            .iter()
            .map(|method| {
                wat! {
                    (struct_new $alist_str_method_pair
                        (global_get ,(string_identifier(&method.name)))
                        (ref_func ,(method.identifier())))
                }
            })
            .flatten()
            .collect();
        let len: i64 = struct_defs.len().try_into().unwrap();
        wat! {
            (array_new_fixed $alist_str_method ,(len)
                             ,(struct_defs))
        }
        .remove(0)
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
pub fn object() -> Class {
    Class {
        name: "Object".to_string(),
        parent_name: "Class".to_string(),
        superclass_name: Some("BasicObject".to_string()),
        instance_methods: vec![],
    }
}

/// A Vec of all classes defined in `corelib`.
pub fn corelib_classes() -> Vec<Class> {
    vec![module(), class(), basic_object(), object()]
}

pub fn add_class_defs(ctx: &mut CompileCtx) {
    assert!(!ctx.classes.is_empty());
    let mut classes: Vec<_> = ctx.classes.drain(..).collect();
    for method in &ctx.methods {
        add_instance_method(&mut classes, method)
    }
    for class in classes {
        ctx.module.globals.push(class.def())
    }
}

fn add_instance_method(classes: &mut Vec<Class>, method: &Method) {
    let class = classes.iter_mut().find(|c| c.name == method.class).unwrap();
    class.instance_methods.push(method.clone())
}
