use crate::instr::Instr;
use crate::ty::GlobalType;

#[derive(Debug)]
pub struct Global {
    pub name: String,
    pub ty: GlobalType,
    pub instr_seq: Vec<Instr>,
}
