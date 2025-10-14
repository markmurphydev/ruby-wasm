use crate::ty::NumType;

#[derive(Debug)]
pub enum UnfoldedInstr {
    /// No-op instruction.
    Nop,

    Const {
        ty: NumType,
        val: i64,
    },
}

impl UnfoldedInstr {
    /// Is `name` a "simple instruction" -- an instruction with no immediate args?
    pub fn is_simple_instr(name: &str) -> bool {
        match name {
            "nop" => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Instr {
    pub instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}
