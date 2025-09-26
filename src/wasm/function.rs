//! attribution: Adapted from Walrus crate `walrus/src/{function.rs, function_builder.rs}`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::compiler::CompileCtx;
use crate::wasm::intern::{IdentifierInterner, InternedIdentifier};
use crate::wasm::module::ModuleFunctions;
use crate::wasm::types::{ParamsType, ResultsType};
use crate::wasm::{Block, IfElse, Instr, Loop, Value};
use id_arena::{Arena, Id};
use std::ops::{Deref, DerefMut};

pub type FunctionId = Id<Function>;

#[derive(Debug)]
pub struct Function {
    pub(super) builder: FunctionBuilder,
}

impl Function {
    fn new(builder: FunctionBuilder) -> Self {
        Self { builder }
    }

    pub fn name(&self) -> &str {
        self.builder.ident_interner.get(self.builder.name)
    }

    pub fn exported(&self) -> ExportStatus {
        self.builder.exported
    }

    pub fn params(&self) -> &ParamsType {
        &self.builder.params
    }

    pub fn results(&self) -> &ResultsType {
        &self.builder.results
    }
}

/// Is this item exported?
#[derive(Debug, Copy, Clone)]
pub enum ExportStatus {
    Exported,
    NotExported
}

/// Build instances of `LocalFunction`.
#[derive(Debug)]
pub struct FunctionBuilder {
    ident_interner: IdentifierInterner,
    /// The function's instruction sequence is conceptually a tree.
    ///
    /// ```wat
    /// ...
    /// (if ((instr) (instr))
    ///   (then
    ///     (if ((instr) (instr))
    ///       (then ...
    ///       (else ...))
    ///   (else ...))
    /// (instr)
    /// ...
    /// ```
    ///
    /// But like other trees, we prefer `(arena, idx's)` to `(heap, Box<T>)`
    pub(super) instr_seq_arena: Arena<InstrSeq>,
    name: InternedIdentifier,
    exported: ExportStatus,
    params: ParamsType,
    results: ResultsType,
    /// The entry-point into this function.
    pub(super) entry_point: InstrSeqId,
}

impl FunctionBuilder {
    /// Creates a new, empty function builder.
    pub fn new(name: &str, exported: ExportStatus, params: ParamsType, results: ResultsType) -> Self {
        let mut ident_interner = IdentifierInterner::new();
        let name = ident_interner.intern(name);

        let mut instr_seq_arena = Arena::new();
        let entry_point = instr_seq_arena.alloc(InstrSeq::new());

        Self {
            ident_interner,
            instr_seq_arena,
            name,
            exported,
            params,
            results,
            entry_point,
        }
    }

    /// Get a `InstrSeqBuilder` for building and mutating this function's body.
    pub fn func_body(&mut self) -> InstrSeqBuilder {
        let entry = self.entry_point;
        self.instr_seq(entry)
    }

    /// Continue building and mutating an existing instruction sequence.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut module = walrus::Module::default();
    /// let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    ///
    /// let mut block = builder.dangling_instr_seq(None);
    /// let id = block.id();
    /// // Build up the block some.
    /// block
    ///     .f64_const(1337.0)
    ///     .drop();
    ///
    /// // Do some other stuff...
    /// drop(block);
    ///
    /// // Use `instr_seq` to get the builder for the block again, and build
    /// // some more things onto it.
    /// let mut block = builder.instr_seq(id);
    /// block
    ///     .i32_const(42)
    ///     .drop();
    /// ```
    pub fn instr_seq(&mut self, id: InstrSeqId) -> InstrSeqBuilder {
        InstrSeqBuilder { id }
    }

    /// Finishes this builder, wrapping it all up and inserting it into the
    /// specified `Module`.
    pub fn finish(self, funcs: &mut ModuleFunctions) -> FunctionId {
        let func = Function::new(self);
        funcs.add(func)
    }
}

/// A builder returned by instruction sequence-construction methods to build up
/// instructions within a block/loop/if-else over time.
pub struct InstrSeqBuilder {
    pub id: InstrSeqId,
}

//noinspection GrazieInspection
impl InstrSeqBuilder {
    /// Construct a builder for a new `InstrSeq`, inserted into the given arena.
    pub fn new(arena: &mut Arena<InstrSeq>) -> Self {
        Self {
            id: arena.alloc(InstrSeq::new()),
        }
    }

    /// Pushes a new instruction onto this builder's sequence.
    #[inline]
    pub fn instr(&self, ctx: &mut CompileCtx, instr: impl Into<Instr>) -> &Self {
        ctx.module.instr_seq_arena[self.id].0.push(instr.into());
        self
    }

    /// Creates an `i32.const` instruction for the specified value.
    #[inline]
    pub fn i32_const(&self, ctx: &mut CompileCtx<'_>, val: i32) -> &Self {
        self.const_(ctx, Value::I32(val))
    }

    /// Creates an `i64.const` instruction for the specified value.
    #[inline]
    pub fn i64_const(&self,ctx: &mut CompileCtx<'_>,  val: i64) -> &Self {
        self.const_(ctx, Value::I64(val))
    }

    /// Creates an `f32.const` instruction for the specified value
    #[inline]
    pub fn f32_const(&self,ctx: &mut CompileCtx<'_>,  val: f32) -> &Self {
        self.const_(ctx, Value::F32(val))
    }

    /// Creates an `f64.const` instruction for the specified value
    #[inline]
    pub fn f64_const(&self,ctx: &mut CompileCtx<'_>,  val: f64) -> &Self {
        self.const_(ctx, Value::F64(val))
    }

    /// Append a new, nested `block ... end` to this builder's sequence.
    pub fn block(&self, ctx: &mut CompileCtx<'_>, make_block: impl FnOnce(&Self)) -> &Self {
        let mut builder = self.dangling_instr_seq(ctx);
        make_block(&builder);
        let seq = builder.id;
        self.instr(ctx, Block { seq })
    }

    /// Create a new `loop ... end` instruction sequence.
    pub fn loop_(
        &self,
        ctx: &mut CompileCtx<'_>,
        label: String,
        make_loop: impl FnOnce(&mut CompileCtx<'_>, &InstrSeqBuilder),
    ) -> &Self {
        let builder = self.dangling_instr_seq(ctx);
        make_loop(ctx, &builder);
        let seq = builder.id;
        self.instr(ctx, Loop { label, seq })
    }

    /// Build a new `if <consequent> else <alternative> end` instruction
    /// sequence.
    pub fn if_else(
        &self,
        ctx: &mut CompileCtx<'_>,
        predicate: impl FnOnce(&mut CompileCtx<'_>, &InstrSeqBuilder),
        consequent: impl FnOnce(&mut CompileCtx<'_>, &InstrSeqBuilder),
        alternative: impl FnOnce(&mut CompileCtx<'_>, &InstrSeqBuilder),
    ) -> &Self {
        let predicate = {
            let mut builder = self.dangling_instr_seq(ctx);
            predicate(ctx, &mut builder);
            builder.id
        };

        let consequent = {
            let mut builder = self.dangling_instr_seq(ctx);
            consequent(ctx, &mut builder);
            builder.id
        };

        let alternative = {
            let mut builder = self.dangling_instr_seq(ctx);
            alternative(ctx, &mut builder);
            builder.id
        };

        self.instr(
            ctx,
            IfElse {
                predicate,
                consequent,
                alternative,
            },
        )
    }

    fn dangling_instr_seq(&self, ctx: &mut CompileCtx<'_>) -> InstrSeqBuilder {
        let id = ctx.module.instr_seq_arena.alloc(InstrSeq::new());
        InstrSeqBuilder { id }
    }
}

/// The identifier for a `InstrSeq` within some `Function`.
pub type InstrSeqId = Id<InstrSeq>;

/// A sequence of instructions.
#[derive(Debug, Default)]
pub struct InstrSeq(pub Vec<Instr>);

impl InstrSeq {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Deref for InstrSeq {
    type Target = Vec<Instr>;

    #[inline]
    fn deref(&self) -> &Vec<Instr> {
        &self.0
    }
}

impl DerefMut for InstrSeq {
    #[inline]
    fn deref_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.0
    }
}
