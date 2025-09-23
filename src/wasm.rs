//! An AST representation of the final Wasm code.
//! One style of Wasm's text representation is tree-structured s-expressions,
//!     so this can be trivially written to the `.wat` format.
//!
//! attribution: Substantially copied from Walrus crate `walrus/src/ir/mod.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

pub mod types;
pub mod values;
mod function_builder;

use wasm_macro::wasm_instr;
use crate::wasm::types::{GlobalType, Type, ValueType};
use crate::wasm::values::{I32, I64, U32};

/// An enum of all the different kinds of wasm instructions.
///
/// Note that the `#[walrus_expr]` macro rewrites this enum's variants from
///
/// ```ignore
/// enum Instr {
///     Variant { field: Ty, .. },
///     ...
/// }
/// ```
///
/// into
///
/// ```ignore
/// enum Instr {
///     Variant(Variant),
///     ...
/// }
///
/// struct Variant {
///     field: Ty,
///     ...
/// }
/// ```
#[wasm_instr]
#[derive(Clone, Debug)]
pub enum Instr {
    // /// `block ... end`
    // #[walrus(skip_builder)]
    // Block {
    //     /// The id of this `block` instruction's inner `InstrSeq`.
    //     seq: InstrSeqId,
    // },

    /// `loop ... end`
    #[wasm(skip_builder)]
    Loop {
        /// The id of this `loop` instruction's inner `InstrSeq`.
        seq: InstrSeqId,
    },

    // /// `call`
    // Call {
    //     /// The function being invoked.
    //     func: FunctionId,
    // },
    //
    // /// `call_indirect`
    // CallIndirect {
    //     /// The type signature of the function we're calling
    //     ty: TypeId,
    //     /// The table which `func` below is indexing into
    //     table: TableId,
    // },
    //
    // /// `local.get n`
    // LocalGet {
    //     /// The local being got.
    //     local: LocalId,
    // },
    //
    // /// `local.set n`
    // LocalSet {
    //     /// The local being set.
    //     local: LocalId,
    // },
    //
    // /// `local.tee n`
    // LocalTee {
    //     /// The local being set.
    //     local: LocalId,
    // },
    //
    // /// `global.get n`
    // GlobalGet {
    //     /// The global being got.
    //     global: GlobalId,
    // },
    //
    // /// `global.set n`
    // GlobalSet {
    //     /// The global being set.
    //     global: GlobalId,
    // },
    //
    // /// `*.const`
    // Const {
    //     /// The constant value.
    //     value: Value,
    // },
    //
    // /// Ternary operations, those requiring three operands
    // TernOp {
    //     /// The operation being performed
    //     #[walrus(skip_visit)]
    //     op: TernaryOp,
    // },
    //
    // /// Binary operations, those requiring two operands
    // Binop {
    //     /// The operation being performed
    //     #[walrus(skip_visit)]
    //     op: BinaryOp,
    // },
    //
    // /// Unary operations, those requiring one operand
    // Unop {
    //     /// The operation being performed
    //     #[walrus(skip_visit)]
    //     op: UnaryOp,
    // },
    //
    // /// `select`
    // Select {
    //     /// Optionally listed type that the `select` instruction is expected to
    //     /// produce, used in subtyping relations with the gc proposal.
    //     #[walrus(skip_visit)]
    //     ty: Option<ValType>,
    // },
    //
    // /// `unreachable`
    // Unreachable {},
    //
    // /// `br`
    // Br {
    //     /// The target block to branch to.
    //     #[walrus(skip_visit)] // should have already been visited
    //     block: InstrSeqId,
    // },
    //
    // /// `br_if`
    // BrIf {
    //     /// The target block to branch to when the condition is met.
    //     #[walrus(skip_visit)] // should have already been visited
    //     block: InstrSeqId,
    // },
    //
    // /// `if <consequent> else <alternative> end`
    // #[walrus(skip_builder)]
    // IfElse {
    //     /// The block to execute when the condition is true.
    //     consequent: InstrSeqId,
    //     /// The block to execute when the condition is false.
    //     alternative: InstrSeqId,
    // },
    //
    // /// `br_table`
    // BrTable {
    //     /// The table of target blocks.
    //     #[walrus(skip_visit)] // should have already been visited
    //     blocks: Box<[InstrSeqId]>,
    //     /// The block that is branched to by default when `which` is out of the
    //     /// table's bounds.
    //     #[walrus(skip_visit)] // should have already been visited
    //     default: InstrSeqId,
    // },
    //
    // /// `drop`
    // Drop {},
    //
    // /// `return`
    // Return {},
    //
    // /// `memory.size`
    // MemorySize {
    //     /// The memory we're fetching the current size of.
    //     memory: MemoryId,
    // },
    //
    // /// `memory.grow`
    // MemoryGrow {
    //     /// The memory we're growing.
    //     memory: MemoryId,
    // },
    //
    // /// `memory.init`
    // MemoryInit {
    //     /// The memory we're growing.
    //     memory: MemoryId,
    //     /// The data to copy in
    //     data: DataId,
    // },
    //
    // /// `data.drop`
    // DataDrop {
    //     /// The data to drop
    //     data: DataId,
    // },
    //
    // /// `memory.copy`
    // MemoryCopy {
    //     /// The source memory
    //     src: MemoryId,
    //     /// The destination memory
    //     dst: MemoryId,
    // },
    //
    // /// `memory.fill`
    // MemoryFill {
    //     /// The memory to fill
    //     memory: MemoryId,
    // },
    //
    // /// `*.load`
    // ///
    // /// Loading a value from memory.
    // Load {
    //     /// The memory we're loading from.
    //     memory: MemoryId,
    //     /// The kind of memory load this is performing
    //     #[walrus(skip_visit)]
    //     kind: LoadKind,
    //     /// The alignment and offset of this memory load
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// `*.store`
    // ///
    // /// Storing a value to memory.
    // Store {
    //     /// The memory we're storing to
    //     memory: MemoryId,
    //     /// The kind of memory store this is performing
    //     #[walrus(skip_visit)]
    //     kind: StoreKind,
    //     /// The alignment and offset of this memory store
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// An atomic read/modify/write operation.
    // AtomicRmw {
    //     /// The memory we're modifying
    //     memory: MemoryId,
    //     /// The atomic operation being performed
    //     #[walrus(skip_visit)]
    //     op: AtomicOp,
    //     /// The atomic operation being performed
    //     #[walrus(skip_visit)]
    //     width: AtomicWidth,
    //     /// The alignment and offset from the base address
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// An atomic compare-and-exchange operation.
    // Cmpxchg {
    //     /// The memory we're modifying
    //     memory: MemoryId,
    //     /// The atomic operation being performed
    //     #[walrus(skip_visit)]
    //     width: AtomicWidth,
    //     /// The alignment and offset from the base address
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// The `atomic.notify` instruction to wake up threads.
    // AtomicNotify {
    //     /// The memory we're notifying through
    //     memory: MemoryId,
    //     /// The alignment and offset from the base address
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// The `*.atomic.wait` instruction to block threads.
    // AtomicWait {
    //     /// The memory we're waiting through.
    //     memory: MemoryId,
    //     /// The alignment and offset from the base address.
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    //     /// Whether or not this is an `i32` or `i64` wait.
    //     #[walrus(skip_visit)]
    //     sixty_four: bool,
    // },
    //
    // /// The `atomic.fence` instruction
    // AtomicFence {},
    //
    // /// `table.get`
    // TableGet {
    //     /// The table we're fetching from.
    //     table: TableId,
    // },
    //
    // /// `table.set`
    // TableSet {
    //     /// The table we're storing to.
    //     table: TableId,
    // },
    //
    // /// `table.grow`
    // TableGrow {
    //     /// The table we're growing
    //     table: TableId,
    // },
    //
    // /// `table.size`
    // TableSize {
    //     /// The table we're getting the size of
    //     table: TableId,
    // },
    //
    // /// `table.fill`
    // TableFill {
    //     /// The table we're filling
    //     table: TableId,
    // },
    //
    // /// `ref.null $ty`
    // RefNull {
    //     /// The type of null that we're producing
    //     #[walrus(skip_visit)]
    //     ty: RefType,
    // },
    //
    // /// `ref.is_null`
    // RefIsNull {},
    //
    // /// `ref.func`
    // RefFunc {
    //     /// The function that this instruction is referencing
    //     func: FunctionId,
    // },
    //
    // /// `v128.bitselect`
    // V128Bitselect {},
    //
    // /// `i8x16.swizzle`
    // I8x16Swizzle {},
    //
    // /// `i8x16.shuffle`
    // I8x16Shuffle {
    //     /// The indices that are used to create the final vector of this
    //     /// instruction
    //     #[walrus(skip_visit)]
    //     indices: ShuffleIndices,
    // },
    //
    // /// Various instructions to load a simd vector from memory
    // LoadSimd {
    //     /// The memory we're loading from.
    //     memory: MemoryId,
    //     /// The size of load this is performing
    //     #[walrus(skip_visit)]
    //     kind: LoadSimdKind,
    //     /// The alignment and offset of this memory load
    //     #[walrus(skip_visit)]
    //     arg: MemArg,
    // },
    //
    // /// `table.init`
    // TableInit {
    //     /// The table we're copying into.
    //     table: TableId,
    //     /// The element we're getting items from.
    //     elem: ElementId,
    // },
    //
    // /// `elem.drop`
    // ElemDrop {
    //     /// The elem segment to drop
    //     elem: ElementId,
    // },
    //
    // /// `table.copy`
    // TableCopy {
    //     /// The source table
    //     src: TableId,
    //     /// The destination table
    //     dst: TableId,
    // },
    //
    // /// `return_call`
    // ReturnCall {
    //     /// The function being invoked.
    //     func: FunctionId,
    // },
    //
    // /// `return_call_indirect`
    // ReturnCallIndirect {
    //     /// The type signature of the function we're calling
    //     ty: TypeId,
    //     /// The table which `func` below is indexing into
    //     table: TableId,
    // },
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // Number instructions
    ConstI32(I32),
    ConstI64(I64),
    /// Convert an `i32` to a `(ref i31)`
    RefI31,
    /// Convert a `(ref i31)` to `i32`, treating as unsigned
    I31GetU,

    // Global instructions
    GlobalGet(GlobalIdx),

    I32Xor,
    I32Or,
    I32Eqz,
    I32Eq,

    // Control instructions
    If(If),
    Loop(Loop),
}

#[derive(Debug, Clone)]
/// (if label block_type? predicate_instrs* (then then_instrs*) (else else_instrs*)?)
pub struct If {
    /// Idk. Is it for named breaks?
    pub label: Option<String>,

    /// The return type of the if, else blocks
    /// TODO: This should be a union of something and valtype
    /// TODO: This might always be Unitype, or might sometimes be Unitype, sometimes Void
    pub block_type: Type,

    pub predicate_instrs: Vec<Instruction>,

    pub then_instrs: Vec<Instruction>,
    pub else_instrs: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub label: Option<String>,
    /// The return type of the loop instructions
    pub block_type: Type,
    pub instructions: Vec<Instruction>,
}

// ==== Wasm Module Items ====

#[derive(Debug, Clone)]
pub enum FunctionIdx {
    // TODO -- Spec defines indices to be wasm-u32
    // https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start
    Index(U32),
    Id(String),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Option<String>,
    pub body: Expr,
}

/// Sequence of instructions
/// https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-expr
#[derive(Debug, Clone)]
pub struct Expr(pub Vec<Instruction>);

#[derive(Debug, Clone)]
pub enum GlobalIdx {
    Idx(U32),
    Id(String),
}

#[derive(Debug, Clone)]
pub struct Global {
    pub id: Option<String>,
    pub global_type: GlobalType,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub exports: Vec<FunctionIdx>,
    pub globals: Vec<Global>,
    /// A function `() -> ()` which _initializes_ the wasm module
    /// NB: _not_ a main function
    pub start: Option<FunctionIdx>,
}
