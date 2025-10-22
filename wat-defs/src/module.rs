use crate::func::Func;
use crate::global::Global;
use crate::ty::SubType;

#[derive(Debug, Default)]
pub struct Module {
    pub types: Vec<TypeDef>,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
    pub start_fn: Option<String>,
}

impl Module {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: String,
    pub ty: SubType,
}
