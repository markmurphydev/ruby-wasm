use wat_defs::module::TypeDef;
use wat_macro::wat;

pub fn array_unitype() -> TypeDef {
    wat! {
        (type $arr_unitype (array (ref eq)))
    }
}

pub fn array_type_defs() -> Vec<TypeDef> {
    vec![array_unitype()]
}
