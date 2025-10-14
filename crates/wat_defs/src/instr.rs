use crate::ty::NumType;

#[derive(Debug)]
pub enum UnfoldedInstr {
    /// No-op instruction.
    Nop,

    Const {
        ty: NumType,
        val: i64,
    },

    Loop {
        label: String,
    },
}

#[derive(Debug)]
pub struct Instr {
    pub instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}
