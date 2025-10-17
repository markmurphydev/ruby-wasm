// //! An AST representation of the final Wasm code.
// //! One style of Wasm's text representation is tree-structured s-expressions,
// //!     so this can be trivially written to the `.wat` format.
// //!
// //! attribution: Substantially copied from Walrus crate `walrus/src/ir/mod.rs`
// //! https://github.com/wasm-bindgen/walrus
// //! MIT licensed
//
// pub mod function;
// pub mod instr_seq;
// mod intern;
// pub mod module;
// pub mod types;
// pub mod wat;
//
// use crate::wasm::intern::InternedIdentifier;
// use crate::wasm::types::{BlockType, GlobalType, RefType, SubType};
// use crate::CompileCtx;
// use instr_seq::InstrSeqId;
// use wasm_instr::wasm_instr;
//
// /// Constant values that can show up in WebAssembly
// #[derive(Debug, Clone, Copy)]
// pub enum Value {
//     /// A constant 32-bit integer
//     I32(i32),
//     /// A constant 64-bit integer
//     I64(i64),
//     /// A constant 32-bit float
//     F32(f32),
//     /// A constant 64-bit float
//     F64(f64),
// }
//
// /// An enum of all the different kinds of wasm instructions.
// ///
// /// Note that the `#[wasm_expr]` wasm_instr rewrites this enum's variants from
// ///
// /// ```ignore
// /// enum Instr {
// ///     Variant { field: Ty, .. },
// ///     ...
// /// }
// /// ```
// ///
// /// into
// ///
// /// ```ignore
// /// enum Instr {
// ///     Variant(Variant),
// ///     ...
// /// }
// ///
// /// struct Variant {
// ///     field: Ty,
// ///     ...
// /// }
// /// ```
// #[wasm_instr]
// #[derive(Clone, Debug)]
// pub enum Instr {
//     /// `block ... end`
//     #[wasm(skip_builder)]
//     Block {
//         /// The id of this `block` instruction's inner `InstrSeq`.
//         seq: InstrSeqId,
//     },
//
//     /// `loop ... end`
//     #[wasm(skip_builder)]
//     Loop {
//         label: String,
//         /// The id of this `loop` instruction's inner `InstrSeq`.
//         seq: InstrSeqId,
//     },
//
//     /// `call`
//     Call {
//         /// The identifier of function being invoked.
//         func: String,
//     },
//
//     CallRef {
//         type_name: String,
//     },
//
//     // /// `call_indirect`
//     // CallIndirect {
//     //     /// The type signature of the function we're calling
//     //     ty: TypeId,
//     //     /// The table which `func` below is indexing into
//     //     table: TableId,
//     // },
//     /// `local.get n`
//     LocalGet {
//         /// The local being got.
//         name: String,
//     },
//
//     LocalSet {
//         name: String,
//     },
//
//     // /// `local.tee n`
//     // LocalTee {
//     //     /// The local being set.
//     //     local: LocalId,
//     // },
//     /// `global.get n`
//     GlobalGet {
//         /// The global being got.
//         name: String,
//     },
//
//     // /// `global.set n`
//     // GlobalSet {
//     //     /// The global being set.
//     //     global: GlobalId,
//     // },
//     /// `*.const`
//     Const {
//         /// The constant value.
//         value: Value,
//     },
//
//     /// Binary operations, those requiring two operands
//     Binop {
//         /// The operation being performed
//         op: BinaryOp,
//     },
//
//     /// Unary operations, those requiring one operand
//     Unop {
//         /// The operation being performed
//         op: UnaryOp,
//     },
//
//     // /// `select`
//     // Select {
//     //     /// Optionally listed type that the `select` instruction is expected to
//     //     /// produce, used in subtyping relations with the gc proposal.
//     //     #[walrus(skip_visit)]
//     //     ty: Option<ValType>,
//     // },
//
//     /// `unreachable`
//     Unreachable {},
//
//     /// `br`
//     Br {
//         /// The target block to branch to.
//         label: String,
//     },
//
//     /// `br_if`
//     BrIf {
//         /// The target block to branch to when the condition is met.
//         block: String,
//     },
//
//     /// `if <predicate> then <consequent> else <alternative> end`
//     #[wasm(skip_builder)]
//     IfElse {
//         /// The type that `consequent`, `alternative` return.
//         ty: Option<BlockType>,
//         /// The condition to evaluate.
//         predicate: InstrSeqId,
//         /// The block to execute when the condition is true.
//         consequent: InstrSeqId,
//         /// The block to execute when the condition is false.
//         alternative: InstrSeqId,
//     },
//
//     // /// `br_table`
//     // BrTable {
//     //     /// The table of target blocks.
//     //     #[walrus(skip_visit)] // should have already been visited
//     //     blocks: Box<[InstrSeqId]>,
//     //     /// The block that is branched to by default when `which` is out of the
//     //     /// table's bounds.
//     //     #[walrus(skip_visit)] // should have already been visited
//     //     default: InstrSeqId,
//     // },
//     /// `drop`
//     Drop {},
//
//     /// `return`
//     Return {},
//
//     // /// `memory.size`
//     // MemorySize {
//     //     /// The memory we're fetching the current size of.
//     //     memory: MemoryId,
//     // },
//     //
//     // /// `memory.grow`
//     // MemoryGrow {
//     //     /// The memory we're growing.
//     //     memory: MemoryId,
//     // },
//     //
//     // /// `memory.init`
//     // MemoryInit {
//     //     /// The memory we're growing.
//     //     memory: MemoryId,
//     //     /// The data to copy in
//     //     data: DataId,
//     // },
//     //
//     // /// `data.drop`
//     // DataDrop {
//     //     /// The data to drop
//     //     data: DataId,
//     // },
//     //
//     // /// `memory.copy`
//     // MemoryCopy {
//     //     /// The source memory
//     //     src: MemoryId,
//     //     /// The destination memory
//     //     dst: MemoryId,
//     // },
//     //
//     // /// `memory.fill`
//     // MemoryFill {
//     //     /// The memory to fill
//     //     memory: MemoryId,
//     // },
//     //
//     // /// `*.load`
//     // ///
//     // /// Loading a value from memory.
//     // Load {
//     //     /// The memory we're loading from.
//     //     memory: MemoryId,
//     //     /// The kind of memory load this is performing
//     //     #[walrus(skip_visit)]
//     //     kind: LoadKind,
//     //     /// The alignment and offset of this memory load
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// `*.store`
//     // ///
//     // /// Storing a value to memory.
//     // Store {
//     //     /// The memory we're storing to
//     //     memory: MemoryId,
//     //     /// The kind of memory store this is performing
//     //     #[walrus(skip_visit)]
//     //     kind: StoreKind,
//     //     /// The alignment and offset of this memory store
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// An atomic read/modify/write operation.
//     // AtomicRmw {
//     //     /// The memory we're modifying
//     //     memory: MemoryId,
//     //     /// The atomic operation being performed
//     //     #[walrus(skip_visit)]
//     //     op: AtomicOp,
//     //     /// The atomic operation being performed
//     //     #[walrus(skip_visit)]
//     //     width: AtomicWidth,
//     //     /// The alignment and offset from the base address
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// An atomic compare-and-exchange operation.
//     // Cmpxchg {
//     //     /// The memory we're modifying
//     //     memory: MemoryId,
//     //     /// The atomic operation being performed
//     //     #[walrus(skip_visit)]
//     //     width: AtomicWidth,
//     //     /// The alignment and offset from the base address
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// The `atomic.notify` instruction to wake up threads.
//     // AtomicNotify {
//     //     /// The memory we're notifying through
//     //     memory: MemoryId,
//     //     /// The alignment and offset from the base address
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// The `*.atomic.wait` instruction to block threads.
//     // AtomicWait {
//     //     /// The memory we're waiting through.
//     //     memory: MemoryId,
//     //     /// The alignment and offset from the base address.
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     //     /// Whether or not this is an `i32` or `i64` wait.
//     //     #[walrus(skip_visit)]
//     //     sixty_four: bool,
//     // },
//     //
//     // /// The `atomic.fence` instruction
//     // AtomicFence {},
//     //
//     // /// `table.get`
//     // TableGet {
//     //     /// The table we're fetching from.
//     //     table: TableId,
//     // },
//     //
//     // /// `table.set`
//     // TableSet {
//     //     /// The table we're storing to.
//     //     table: TableId,
//     // },
//     //
//     // /// `table.grow`
//     // TableGrow {
//     //     /// The table we're growing
//     //     table: TableId,
//     // },
//     //
//     // /// `table.size`
//     // TableSize {
//     //     /// The table we're getting the size of
//     //     table: TableId,
//     // },
//     //
//     // /// `table.fill`
//     // TableFill {
//     //     /// The table we're filling
//     //     table: TableId,
//     // },
//     //
//     // /// `ref.null $ty`
//     // RefNull {
//     //     /// The type of null that we're producing
//     //     #[walrus(skip_visit)]
//     //     ty: RefType,
//     // },
//     //
//     // /// `ref.is_null`
//     // RefIsNull {},
//     //
//     // /// `ref.func`
//     // RefFunc {
//     //     /// The function that this instruction is referencing
//     //     func: FunctionId,
//     // },
//     /// `ref.test`
//     RefTest {
//         ty: RefType,
//     },
//
//     /// `ref.cast`
//     RefCast {
//         result_ty: RefType,
//     },
//
//     // /// `v128.bitselect`
//     // V128Bitselect {},
//     //
//     // /// `i8x16.swizzle`
//     // I8x16Swizzle {},
//     //
//     // /// `i8x16.shuffle`
//     // I8x16Shuffle {
//     //     /// The indices that are used to create the final vector of this
//     //     /// instruction
//     //     #[walrus(skip_visit)]
//     //     indices: ShuffleIndices,
//     // },
//     //
//     // /// Various instructions to load a simd vector from memory
//     // LoadSimd {
//     //     /// The memory we're loading from.
//     //     memory: MemoryId,
//     //     /// The size of load this is performing
//     //     #[walrus(skip_visit)]
//     //     kind: LoadSimdKind,
//     //     /// The alignment and offset of this memory load
//     //     #[walrus(skip_visit)]
//     //     arg: MemArg,
//     // },
//     //
//     // /// `table.init`
//     // TableInit {
//     //     /// The table we're copying into.
//     //     table: TableId,
//     //     /// The element we're getting items from.
//     //     elem: ElementId,
//     // },
//     //
//     // /// `elem.drop`
//     // ElemDrop {
//     //     /// The elem segment to drop
//     //     elem: ElementId,
//     // },
//     //
//     // /// `table.copy`
//     // TableCopy {
//     //     /// The source table
//     //     src: TableId,
//     //     /// The destination table
//     //     dst: TableId,
//     // },
//     //
//     // /// `return_call`
//     // ReturnCall {
//     //     /// The function being invoked.
//     //     func: FunctionId,
//     // },
//     //
//     // /// `return_call_indirect`
//     // ReturnCallIndirect {
//     //     /// The type signature of the function we're calling
//     //     ty: TypeId,
//     //     /// The table which `func` below is indexing into
//     //     table: TableId,
//     // },
//
//     RefNull {
//         type_name: String,
//     },
//
//     ArrayNewFixed {
//         type_name: String,
//         length: i32,
//     },
//
//     ArrayGet {
//         type_name: String,
//     },
//
//     ArrayGetU {
//         type_name: String,
//     },
//
//     StructNew {
//         type_name: String,
//     },
//
//     StructGet {
//         type_name: String,
//         field_name: String,
//     },
//
//     StructSet {
//         type_name: String,
//         field_name: String,
//     },
//
//     RefFunc {
//         func_name: String,
//     }
// }
//
// /// Possible unary operations in wasm
// #[allow(missing_docs)]
// #[derive(Copy, Clone, Debug)]
// pub enum UnaryOp {
//     I32Eqz,
//     ArrayLen,
//     RefAsNonNull,
//     // I32Clz,
//     // I32Ctz,
//     // I32Popcnt,
//
//     // I64Eqz,
//     // I64Clz,
//     // I64Ctz,
//     // I64Popcnt,
//     //
//     // F32Abs,
//     // F32Neg,
//     // F32Ceil,
//     // F32Floor,
//     // F32Trunc,
//     // F32Nearest,
//     // F32Sqrt,
//     //
//     // F64Abs,
//     // F64Neg,
//     // F64Ceil,
//     // F64Floor,
//     // F64Trunc,
//     // F64Nearest,
//     // F64Sqrt,
//     //
//     // I32WrapI64,
//     // I32TruncSF32,
//     // I32TruncUF32,
//     // I32TruncSF64,
//     // I32TruncUF64,
//     // I64ExtendSI32,
//     // I64ExtendUI32,
//     // I64TruncSF32,
//     // I64TruncUF32,
//     // I64TruncSF64,
//     // I64TruncUF64,
//     //
//     // F32ConvertSI32,
//     // F32ConvertUI32,
//     // F32ConvertSI64,
//     // F32ConvertUI64,
//     // F32DemoteF64,
//     // F64ConvertSI32,
//     // F64ConvertUI32,
//     // F64ConvertSI64,
//     // F64ConvertUI64,
//     // F64PromoteF32,
//     //
//     // I32ReinterpretF32,
//     // I64ReinterpretF64,
//     // F32ReinterpretI32,
//     // F64ReinterpretI64,
//     //
//     // I32Extend8S,
//     // I32Extend16S,
//     // I64Extend8S,
//     // I64Extend16S,
//     // I64Extend32S,
//
//     // RefTestNonNull,
//     // RefTestNullable,
//     // RefCastNonNull,
//     // RefCastNullable,
//     // BrOnCast,
//     // BrOnCastFail,
//     // AnyConvertExtern,
//     // ExternConvertAny,
//     RefI31,
//     I31GetS,
//     I31GetU,
// }
//
// /// Possible binary operations in wasm
// #[allow(missing_docs)]
// #[derive(Copy, Clone, Debug)]
// pub enum BinaryOp {
//     I32Eq,
//     I32Add,
//     // I32Ne,
//     // I32LtS,
//     // I32LtU,
//     // I32GtS,
//     // I32GtU,
//     // I32LeS,
//     // I32LeU,
//     // I32GeS,
//     // I32GeU,
//     //
//     // I64Eq,
//     // I64Ne,
//     // I64LtS,
//     // I64LtU,
//     // I64GtS,
//     // I64GtU,
//     // I64LeS,
//     // I64LeU,
//     // I64GeS,
//     // I64GeU,
//     //
//     // F32Eq,
//     // F32Ne,
//     // F32Lt,
//     // F32Gt,
//     // F32Le,
//     // F32Ge,
//     //
//     // F64Eq,
//     // F64Ne,
//     // F64Lt,
//     // F64Gt,
//     // F64Le,
//     // F64Ge,
//     //
//     // I32Add,
//     // I32Sub,
//     // I32Mul,
//     // I32DivS,
//     // I32DivU,
//     // I32RemS,
//     // I32RemU,
//     // I32And,
//     // I32Or,
//     // I32Xor,
//     // I32Shl,
//     // I32ShrS,
//     // I32ShrU,
//     // I32Rotl,
//     // I32Rotr,
//     //
//     // I64Add,
//     // I64Sub,
//     // I64Mul,
//     // I64DivS,
//     // I64DivU,
//     // I64RemS,
//     // I64RemU,
//     // I64And,
//     // I64Or,
//     // I64Xor,
//     // I64Shl,
//     // I64ShrS,
//     // I64ShrU,
//     // I64Rotl,
//     // I64Rotr,
//     //
//     // F32Add,
//     // F32Sub,
//     // F32Mul,
//     // F32Div,
//     // F32Min,
//     // F32Max,
//     // F32Copysign,
//     //
//     // F64Add,
//     // F64Sub,
//     // F64Mul,
//     // F64Div,
//     // F64Min,
//     // F64Max,
//     // F64Copysign,
// }
//
// #[derive(Debug)]
// pub struct Global {
//     pub name: InternedIdentifier,
//     pub ty: GlobalType,
//     /// Id of the root of the instr-seq tree this global initializes
//     ///     Id into the module's `instr_seq_arena`
//     /// TODO -- actually ambiguous rn.
//     pub instr_seq: InstrSeqId,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub enum Finality {
//     Final,
//     NotFinal,
// }
//
// /// Module-level type definition.
// /// We ignore `sub`, `rec`, and non-`final` until we need them.
// #[derive(Debug)]
// pub struct TypeDef {
//     pub name: InternedIdentifier,
//     pub ty: SubType,
// }
//
// impl TypeDef {
//     pub fn new(ctx: &mut CompileCtx<'_>, name: &str, ty: SubType) -> TypeDef {
//         let name = ctx.module.interner.intern(name);
//         Self { name, ty }
//     }
// }
