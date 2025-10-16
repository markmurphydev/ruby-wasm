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

    /// Loop instruction. The test instructions are on the stack (or in `folded_instrs`).
    If {
        label: Option<String>,
        block_type: Option<()>,
        then_block: Vec<Instr>,
        else_block: Vec<Instr>,
    },
}

#[derive(Debug)]
pub struct Instr {
    pub instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}

impl Instr {
    pub fn is_instr(str: &str) -> bool {
        str == "nop" || str == "const" || str == "loop" || str == "if"
    }
}
