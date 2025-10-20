use wat_defs::module::TypeDef;
use wat_macro::wat;
use crate::CompileCtx;
use crate::corelib::{alist, array};

pub const OBJECT_TYPE_IDENTIFIER: &str = "obj";
pub const CLASS_TYPE_IDENTIFIER: &str = "class";
pub const METHOD_TYPE_IDENTIFIER: &str = "method";

pub fn add_type_defs(ctx: &mut CompileCtx<'_>) {
    let mut type_defs = vec![
        string(),
        obj(),
        method(),
        class(),
    ];
    type_defs.append(&mut array::array_type_defs());
    type_defs.append(&mut alist::alist_type_defs());

    ctx.module.types.append(&mut type_defs);
}

// Not needed unless we need an `identity` field for object-equality.
//      At that time, $string, $obj, $class should all subtype this.
// fn unitype_heap_val() -> TypeDef {
//     wat! {
//         (type $unitype_heap
//             (sub (struct)))
//     }
// }

fn string() -> TypeDef {
    wat![ (type $str (array i8)) ]
}


fn boxnum() -> TypeDef {
    wat![ (type $boxnum (struct (field $val i64))) ]
}

/// The wasm type-definition of a Ruby object.
fn obj() -> TypeDef {
    wat! {
        (type $obj (sub (struct (field $parent (mut (ref null $class))))))
    }
}

/// The wasm type-definition of a Ruby class.
/// Each defined class (`BasicObject`, `Class`, ...)
///     is a global of type $class
fn class() -> TypeDef {
    wat! {
        (type $class
            (sub final $obj
                (struct (field $parent (mut (ref null $class)))
                        (field $superclass (mut (ref null $class)))
                        (field $name (ref $str))
                        (field $instance_methods (ref $alist_str_method)))))
    }
}

/// The wasm type-definition of a Ruby method.
/// Each method definition is a function subtyping $method.
fn method() -> TypeDef {
    wat! {
        (type $method
            (sub final
                (func (param $self (ref $obj))
                      (param $args (ref $arr_unitype))
                      (result (ref eq)))))
    }
}

