use crate::instr::Instr;
use crate::ty::ValType;

#[derive(Debug)]
pub enum Exported {
    NotExported,
    Exported(String),
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: ValType,
}

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub ty: ValType,
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub exported: Exported,
    pub type_use: Option<String>,
    pub params: Vec<Param>,
    pub results: Vec<ValType>,
    pub locals: Vec<Local>,
    pub instrs: Vec<Instr>,
}
