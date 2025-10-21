use wat_defs::module::TypeDef;
use wat_macro::wat;

pub fn arr_unitype() -> TypeDef {
    wat! {
        (type $arr_unitype (array (ref eq)))
    }
}

pub fn array_type_defs() -> Vec<TypeDef> {
    vec![arr_unitype()]
}
