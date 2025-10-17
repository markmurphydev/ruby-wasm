use crate::CompileCtx;
use crate::corelib::type_def::METHOD_TYPE_IDENTIFIER;
use wat_defs::func::Func;
use wat_defs::instr::Instr;
use wat_macro::wat;

/// A Ruby method. Compiles to:
/// - Definition of function type `$<METHOD_FUNC_NAME>`
///     with signature `(self: Object, args: Array Unitype) -> Unitype`
/// - Definition of global string `$<METHOD_NAME>`
pub struct Method {
    pub name: String,
    pub method_def: fn() -> Func,
}

impl Method {
    pub fn identifier(&self) -> String {
        format!("{}_{}", METHOD_TYPE_IDENTIFIER, self.name)
    }
}

const NEW_NAME: &str = "new";

pub fn new() -> Method {
    Method {
        name: NEW_NAME.to_string(),
        method_def: new_method_def,
    }
}

fn new_method_def() -> Func {
    make_method(
        new().identifier(),
        wat! {
            (struct_new $obj
              // .class
              (ref_cast (ref $class) (local_get $self)))
        },
    )
}

const NAME_NAME: &str = "name";

pub fn name() -> Method {
    Method {
        name: NAME_NAME.to_string(),
        method_def: name_method_def,
    }
}

fn name_method_def() -> Func {
    make_method(
        name().identifier(),
        wat! {
            (struct_get $class $name
              (ref_cast (ref $class) (local_get $self)))
        },
    )
}

const CLASS_NAME: &str = "class";

pub fn class() -> Method {
    Method {
        name: CLASS_NAME.to_string(),
        method_def: class_method_def,
    }
}

fn class_method_def() -> Func {
    make_method(
        class().identifier(),
        wat! {
            (local_get $self)
            (struct_get $obj $parent)
            (ref_cast (ref eq))
        },
    )
}

fn make_method(name: String, body: Vec<Instr>) -> Func {
    wat! {
        (func ,(name)
            (type $method)
            (param $self (ref $obj)) (param $args (ref $arr_unitype))
            (result (ref eq))
            ,(body))
    }
}

pub fn methods() -> Vec<Method> {
    vec![new(), class(), name()]
}

pub fn add_method_defs(compile_ctx: &mut CompileCtx) {
    for method in methods() {
        compile_ctx.module.funcs.push((method.method_def)())
    }
}
