//! attribution: Adapted from Walrus crate `walrus/src/{function.rs, function_builder.rs}`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::wasm::intern::{IdentifierInterner, InternedIdentifier};
use crate::wasm::module::ModuleFunctions;
use crate::wasm::types::{BlockType, ParamsType, ResultsType};
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
        Self {
            builder
        }
    }

    pub fn name(&self) -> &str {
        self.builder.ident_interner.get(self.builder.name)
    }

    pub fn params(&self) -> &ParamsType {
        &self.builder.params
    }

    pub fn results(&self) -> &ResultsType {
        &self.builder.results
    }

    fn xxx(&self) {
    }
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
    params: ParamsType,
    results: ResultsType,
    /// The entry-point into this function.
    pub(super) entry_point: InstrSeqId,
}

impl FunctionBuilder {
    /// Creates a new, empty function builder.
    pub fn new(
        name: &str,
        params: ParamsType,
        results: ResultsType,
    ) -> Self {
        let mut ident_interner = IdentifierInterner::new();
        let name = ident_interner.intern(name);

        let mut instr_seq_arena = Arena::new();
        let entry_point = instr_seq_arena.alloc(InstrSeq::new());

        Self {
            ident_interner,
            instr_seq_arena,
            name,
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
        InstrSeqBuilder { id, builder: self }
    }

    /// Create a new instruction sequence that is unreachable.
    ///
    /// It is your responsibility to
    ///
    /// * make a `Instr::Block`, `Instr::Loop`, or `Instr::IfElse` that uses
    ///   this instruction sequence, and
    ///
    /// * append that `Instr` into a parent instruction sequence via
    ///   `InstrSeqBuilder::instr` or `InstrSeqBuilder::instr_at`
    ///
    /// or else this built up instruction sequence will never be used.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use walrus::ir::*;
    ///
    /// let mut module = walrus::Module::default();
    /// let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    ///
    /// // Create an empty, dangling instruction sequemce.
    /// let mut seq = builder.dangling_instr_seq(None);
    /// let seq_id = seq.id();
    ///
    /// // Do stuff with the sequence...
    /// drop(seq);
    ///
    /// // Finally, make our instruction sequence reachable by adding an
    /// // block/loop/if-else instruction that uses it to a reachable instruction
    /// // sequence.
    /// builder
    ///     .func_body()
    ///     .instr(Block { seq: seq_id });
    /// ```
    pub fn dangling_instr_seq(&mut self) -> InstrSeqBuilder {
        let id = self.instr_seq_arena.alloc(InstrSeq::new());
        InstrSeqBuilder { id, builder: self }
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
pub struct InstrSeqBuilder<'fb> {
    pub id: InstrSeqId,
    pub builder: &'fb mut FunctionBuilder,
}

impl InstrSeqBuilder<'_> {
    /// Returns the id of the instruction sequence that we're building.
    // #[inline]
    // pub fn id(&self) -> InstrSeqId {
    //     self.id
    // }

    // /// Get this instruction sequence's instructions.
    // pub fn instrs(&self) -> &[(Instr, InstrLocId)] {
    //     &self.builder.arena[self.id]
    // }

    // /// Get this instruction sequence's instructions mutably.
    // pub fn instrs_mut(&mut self) -> &mut Vec<(Instr, InstrLocId)> {
    //     &mut self.builder.arena[self.id].instrs
    // }

    /// Pushes a new instruction onto this builder's sequence.
    #[inline]
    pub fn instr(&mut self, instr: impl Into<Instr>) -> &mut Self {
        self.builder.instr_seq_arena[self.id]
            .0
            .push(instr.into());
        self
    }

    // /// Splice a new instruction into this builder's sequence at the given index.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `position > self.instrs.len()`.
    // #[inline]
    // pub fn instr_at(&mut self, position: usize, instr: impl Into<Instr>) -> &mut Self {
    //     self.builder.arena[self.id]
    //         .instrs
    //         .insert(position, (instr.into(), Default::default()));
    //     self
    // }

    /// Creates an `i32.const` instruction for the specified value.
    #[inline]
    pub fn i32_const(&mut self, val: i32) -> &mut Self {
        self.const_(Value::I32(val))
    }

    /// Creates an `i64.const` instruction for the specified value.
    #[inline]
    pub fn i64_const(&mut self, val: i64) -> &mut Self {
        self.const_(Value::I64(val))
    }

    /// Creates an `f32.const` instruction for the specified value
    #[inline]
    pub fn f32_const(&mut self, val: f32) -> &mut Self {
        self.const_(Value::F32(val))
    }

    /// Creates an `f64.const` instruction for the specified value
    #[inline]
    pub fn f64_const(&mut self, val: f64) -> &mut Self {
        self.const_(Value::F64(val))
    }

    /// Append a new, nested `block ... end` to this builder's sequence.
    ///
    /// # Example:
    ///
    /// ```ignored
    /// let mut module = walrus::Module::default();
    /// let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    ///
    /// // Append the following WAT to the function:
    /// //
    /// //     block
    /// //       i32.const 1337
    /// //       drop
    /// //     end
    /// builder
    ///     .func_body()
    ///     .block(None, |block| {
    ///         block
    ///             .i32_const(1337)
    ///             .drop();
    ///     });
    /// ```
    pub fn block(
        &mut self,
        ty: BlockType,
        make_block: impl FnOnce(&mut InstrSeqBuilder),
    ) -> &mut Self {
        let mut builder = self.builder.dangling_instr_seq();
        make_block(&mut builder);
        let seq = builder.id;
        self.instr(Block { seq })
    }

    /// Create a new `loop ... end` instruction sequence.
    ///
    /// # Example
    ///
    /// ```ignored
    /// let mut module = walrus::Module::default();
    /// let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    ///
    /// // Append the following WAT to the function:
    /// //
    /// //     block
    /// //       i32.const 1337
    /// //       drop
    /// //     end
    /// builder
    ///     .func_body()
    ///     .loop_(None, |loop_| {
    ///         loop_
    ///             .i32_const(1337)
    ///             .drop();
    ///     });
    /// ```
    pub fn loop_(
        &mut self,
        make_loop: impl FnOnce(&mut InstrSeqBuilder),
    ) -> &mut Self {
        let mut builder = self.builder.dangling_instr_seq();
        make_loop(&mut builder);
        let seq = builder.id;
        self.instr(Loop { seq })
    }


    /// Build a new `if <consequent> else <alternative> end` instruction
    /// sequence.
    ///
    /// # Example
    ///
    /// ```ignored
    /// use walrus::ValType;
    ///
    /// let mut module = walrus::Module::default();
    ///
    /// let ty = module.types.add(&[], &[ValType::I32]);
    /// let (flip_coin, _) = module.add_import_func("flip", "coin", ty);
    ///
    /// let mut builder = walrus::FunctionBuilder::new(&mut module.types, &[], &[]);
    /// builder
    ///     .func_body()
    ///     // (if (call $flip_coin)
    ///     //   (then (i32.const 12))
    ///     //   (else (i32.const 34)))
    ///     .call(flip_coin)
    ///     .if_else(
    ///         ValType::I32,
    ///         |then| {
    ///             then.i32_const(12);
    ///         },
    ///         |else_| {
    ///             else_.i32_const(34);
    ///         },
    ///     );
    /// ```
    pub fn if_else(
        &mut self,
        predicate: impl FnOnce(&mut InstrSeqBuilder),
        consequent: impl FnOnce(&mut InstrSeqBuilder),
        alternative: impl FnOnce(&mut InstrSeqBuilder),
    ) -> &mut Self {

        let predicate = {
            let mut builder = self.builder.dangling_instr_seq();
            predicate(&mut builder);
            builder.id
        };

        let consequent = {
            let mut builder = self.builder.dangling_instr_seq();
            consequent(&mut builder);
            builder.id
        };

        let alternative = {
            let mut builder = self.builder.dangling_instr_seq();
            alternative(&mut builder);
            builder.id
        };

        self.instr(IfElse {
            predicate,
            consequent,
            alternative,
        })
    }
}

// impl Deref for InstrSeqBuilder<'_> {
//     type Target = FunctionBuilder;
//
//     fn deref(&self) -> &FunctionBuilder {
//         &*self.builder
//     }
// }
//
// impl DerefMut for InstrSeqBuilder<'_> {
//     fn deref_mut(&mut self) -> &mut FunctionBuilder {
//         &mut *self.builder
//     }
// }

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

