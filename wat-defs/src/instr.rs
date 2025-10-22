use crate::ty::{BlockType, HeapType, NumType, RefType};

#[derive(Debug, Clone)]
pub enum UnfoldedInstr {
    Nop,
    Drop,
    I32Eqz,
    I32Eq,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32Add,
    I32Sub,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32WrapI64,
    I64Eqz,
    I64Eq,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64Add,
    I64Sub,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64ExtendI32U,
    I64ExtendI32S,

    Const {
        ty: NumType,
        val: i64,
    },

    Br {
        label: String,
    },

    BrIf {
        label: String,
    },

    Return,

    Block {
        label: String,
    },

    Loop {
        label: String,
        block_type: Option<BlockType>,
    },

    /// Loop instruction. The test instructions are on the stack (or in `folded_instrs`).
    If {
        label: Option<String>,
        block_type: Option<BlockType>,
        then_block: Vec<Instr>,
        else_block: Vec<Instr>,
    },

    RefNull {
        ty: HeapType,
    },

    RefFunc {
        name: String,
    },

    RefI31,

    I31GetS,

    I31GetU,

    RefAsNonNull,

    RefEq,

    RefTest {
        ty: RefType,
    },

    RefCast {
        ty: RefType,
    },

    Call {
        func: String,
    },

    CallRef {
        type_idx: String,
    },

    LocalGet {
        name: String,
    },

    LocalSet {
        name: String,
    },

    GlobalGet {
        name: String,
    },

    GlobalSet {
        name: String,
    },

    ArrayNewFixed {
        type_idx: String,
        len: i64,
    },

    ArrayGet {
        ty: String,
    },

    ArrayGetU {
        ty: String,
    },

    ArraySet {
        ty: String,
    },

    ArrayLen,

    StructNew {
        ty: String,
    },

    StructGet {
        ty: String,
        field: String,
    },

    StructSet {
        ty: String,
        field: String,
    },

    Unreachable,
}

#[derive(Debug, Clone)]
pub struct Instr {
    pub unfolded_instr: UnfoldedInstr,
    pub folded_instrs: Vec<Instr>,
}

impl Instr {
    pub fn is_instr(str: &str) -> bool {
        str == "nop"
            || str == "drop"
            || str == "const_i32"
            || str == "const_i64"
            || str == "i32_eqz"
            || str == "i32_eq"
            || str == "i32_lt_s"
            || str == "i32_lt_u"
            || str == "i32_gt_s"
            || str == "i32_gt_u"
            || str == "i32_add"
            || str == "i32_sub"
            || str == "i32_and"
            || str == "i32_or"
            || str == "i32_xor"
            || str == "i32_shl"
            || str == "i32_shr_s"
            || str == "i32_shr_u"
            || str == "i32_wrap_i64"
            || str == "i64_eqz"
            || str == "i64_eq"
            || str == "i64_lt_s"
            || str == "i64_lt_u"
            || str == "i64_gt_s"
            || str == "i64_gt_u"
            || str == "i64_add"
            || str == "i64_sub"
            || str == "i64_and"
            || str == "i64_or"
            || str == "i64_xor"
            || str == "i64_shl"
            || str == "i64_shr_s"
            || str == "i64_shr_u"
            || str == "i64_extend_i32_u"
            || str == "i64_extend_i32_s"
            || str == "br"
            || str == "br_if"
            || str == "return"
            || str == "block"
            || str == "loop"
            || str == "if"
            || str == "ref_null"
            || str == "ref_func"
            || str == "ref_i31"
            || str == "i31_get_s"
            || str == "i31_get_u"
            || str == "ref_as_non_null"
            || str == "ref_eq"
            || str == "ref_test"
            || str == "ref_cast"
            || str == "call"
            || str == "call_ref"
            || str == "local_get"
            || str == "local_set"
            || str == "global_get"
            || str == "global_set"
            || str == "array_new_fixed"
            || str == "array_get"
            || str == "array_get_u"
            || str == "array_set"
            || str == "array_len"
            || str == "struct_new"
            || str == "struct_get"
            || str == "struct_set"
            || str == "unreachable"
    }
}
