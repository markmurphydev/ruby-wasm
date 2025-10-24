use crate::CompileCtx;
use crate::corelib::type_def::METHOD_TYPE_IDENTIFIER;
use crate::node::RequiredParam;
use crate::unitype::Unitype;
use wat_defs::func::{Func, Local};
use wat_defs::instr::Instr;
use wat_macro::wat;

/// A Ruby method. Compiles to:
/// - Definition of function type `$<METHOD_FUNC_NAME>`
///     with signature `(self: Object, args: Array Unitype) -> Unitype`
/// - Definition of global string `$<METHOD_NAME>`
#[derive(Debug, Clone)]
pub struct Method {
    pub class: String,
    pub name: String,
    pub method_def: Func,
}

impl Method {
    pub fn identifier(&self) -> String {
        method_identifier(&self.class, &self.name)
    }
}

pub fn method_identifier(class: &str, name: &str) -> String {
    format!("{}_{}_{}", METHOD_TYPE_IDENTIFIER, class, name)
}

const NEW_NAME: &str = "new";

pub fn class_new() -> Method {
    Method {
        class: "Class".to_string(),
        name: NEW_NAME.to_string(),
        method_def: new_method_def("Class"),
    }
}

fn new_method_def(class: &str) -> Func {
    make_method_def(
        class,
        "new",
        &vec![],
        vec![],
        wat! {
            (struct_new $obj
              // .class
              (ref_cast (ref $class) (local_get $self)))
        },
    )
}

const NAME_NAME: &str = "name";

pub fn class_name() -> Method {
    Method {
        class: "Class".to_string(),
        name: NAME_NAME.to_string(),
        method_def: name_method_def("Class"),
    }
}

fn name_method_def(class: &str) -> Func {
    make_method_def(
        class,
        "name",
        &vec![],
        vec![],
        wat! {
            (struct_get $class $name
              (ref_cast (ref $class) (local_get $self)))
        },
    )
}

const CLASS_NAME: &str = "class";

pub fn object_class() -> Method {
    Method {
        class: "Object".to_string(),
        name: CLASS_NAME.to_string(),
        method_def: class_method_def("Object"),
    }
}

fn class_method_def(class: &str) -> Func {
    make_method_def(
        class,
        "class",
        &vec![],
        vec![],
        wat! {
            (ref_cast (ref eq)
                (struct_get $obj $parent (local_get $self)))
        },
    )
}

pub fn make_method_def(
    class: &str,
    name: &str,
    params: &Vec<RequiredParam>,
    locals: Vec<String>,
    body: Vec<Instr>,
) -> Func {
    let param_local_defs: Vec<Local> = params
        .iter()
        .map(|p| {
            wat! { (local ,(p.name.to_string()) (ref eq)) }
        })
        .collect();
    let local_defs = locals.iter().map(|l| {
        wat! { (local ,(l.to_string()) (ref eq)) }
    }).collect();
    let local_defs = [param_local_defs, local_defs].concat();
    let param_local_setters: Vec<Instr> = params
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            wat! {
                (local_set ,(p.name.clone())
                    (array_get $arr_unitype (local_get $args) (const_i32 ,(idx as i64))))
            }
        })
        .flatten()
        .collect();
    let local_setters = locals.iter().map(|l| {
        wat! {
            (local_set ,(l.to_string())
                (ref_i31 (const_i32 ,(Unitype::NIL_BIT_PATTERN as i64))))
        }
    }).flatten().collect();
    let instrs = [param_local_setters, local_setters, body].concat();

    // TODO: Ughhh quasiquoting is broken.

    let no_locals = wat! {
        (func ,(method_identifier(class, name))
            (type $method)
            (param $self (ref $obj))
            (param $args (ref $arr_unitype))
            (result (ref eq))
            ,(instrs))
    };
    let Func {
        name,
        imported,
        exported,
        type_use,
        params,
        results,
        locals: _locals,
        instrs,
    } = no_locals;
    Func {
        name,
        imported,
        exported,
        type_use,
        params,
        results,
        locals: local_defs,
        instrs,
    }
}

pub fn corelib_methods() -> Vec<Method> {
    vec![class_new(), object_class(), class_name()]
}

pub fn add_method_defs(compile_ctx: &mut CompileCtx) {
    assert!(!compile_ctx.methods.is_empty());
    let methods = compile_ctx.methods.drain(..);
    for method in methods {
        compile_ctx.module.funcs.push(method.method_def)
    }
}
