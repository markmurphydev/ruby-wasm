//! attribution: Adapted from Walrus crate `walrus/src/function_builder.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use std::ops::{Deref, DerefMut};
use id_arena::{Arena, Id};
use crate::CompileCtx;
use crate::wasm::{Block, IfElse, Instr, Loop, UnaryOp, Value};

/// A builder returned by instruction sequence-construction methods to build up
/// instructions within a block/loop/if-else over time.
pub struct InstrSeqBuilder {
    pub id: InstrSeqId,
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
    pub fn i64_const(&self, ctx: &mut CompileCtx<'_>, val: i64) -> &Self {
        self.const_(ctx, Value::I64(val))
    }

    /// Creates an `f32.const` instruction for the specified value
    #[inline]
    pub fn f32_const(&self, ctx: &mut CompileCtx<'_>, val: f32) -> &Self {
        self.const_(ctx, Value::F32(val))
    }

    /// Creates an `f64.const` instruction for the specified value
    #[inline]
    pub fn f64_const(&self, ctx: &mut CompileCtx<'_>, val: f64) -> &Self {
        self.const_(ctx, Value::F64(val))
    }

    /// Append a new, nested `block ... end` to this builder's sequence.
    pub fn block(&self, ctx: &mut CompileCtx<'_>, make_block: impl FnOnce(&Self)) -> &Self {
        let builder = self.dangling_instr_seq(ctx);
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

    pub fn i31_const(&self, ctx: &mut CompileCtx<'_>, val: i32) -> &Self {
        self.i32_const(ctx, val).unop(ctx, UnaryOp::RefI31)
    }

    // pub fn const_string(&self, _ctx: &mut CompileCtx<'_>, s: &str) -> &Self {
    //     todo!()
    // }

    fn dangling_instr_seq(&self, ctx: &mut CompileCtx<'_>) -> InstrSeqBuilder {
        let id = ctx.module.instr_seq_arena.alloc(InstrSeq::new());
        InstrSeqBuilder { id }
    }
}
