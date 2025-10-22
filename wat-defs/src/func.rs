use crate::instr::Instr;
use crate::ty::ValType;

#[derive(Debug, Clone)]
pub enum Exported {
    NotExported,
    Exported(String),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: ValType,
}

#[derive(Debug, Clone)]
pub struct Local {
    pub name: String,
    pub ty: ValType,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub exported: Exported,
    pub type_use: Option<String>,
    pub params: Vec<Param>,
    pub results: Vec<ValType>,
    pub locals: Vec<Local>,
    pub instrs: Vec<Instr>,
}
