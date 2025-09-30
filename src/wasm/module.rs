//! attribution: Adapted from Walrus crate `walrus/src/module.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed
//! TODO -- It's bad to have function-local symbol interning
//!     The way to do it would be to pass `&mut Interner` to each `____Builder` method
//!     (maybe as part of a `CompileCtx` object)

use crate::wasm::function::{Function, FunctionId};
use crate::wasm::instr_seq::InstrSeqBuilder;
use crate::wasm::instr_seq::{InstrSeq, InstrSeqId};
use crate::wasm::intern::{IdentifierInterner, InternedIdentifier};
use crate::wasm::types::GlobalType;
use crate::wasm::{Global, TypeDef};
use crate::CompileCtx;
use id_arena::{Arena, Id};

/// A wasm module.
/// Walrus allocates all the data for the module inside
///     module-owned arenas.
/// I do that where necessary to avoid owned `Box<_>`,
///     but using WAT identifiers solves some cases (e.g. locals)
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct Module {
    /// Interner containing all WAT `$symbols`.
    pub interner: IdentifierInterner,
    /// Arena containing all instruction sequences.
    pub instr_seq_arena: Arena<InstrSeq>,
    /// Arena containing all globals.
    pub global_arena: Arena<Global>,
    /// Arena containing all named type definitions.
    pub type_def_arena: Arena<TypeDef>,
    // pub imports: ModuleImports,
    pub funcs: ModuleFunctions,
    // pub exports: ModuleExports,
    /// The `start` function, if any
    pub start: Option<FunctionId>,
}

impl Module {
    /// Create a default, empty module that uses the given configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

/// The set of functions within a module.
#[derive(Debug, Default)]
pub struct ModuleFunctions {
    /// The arena containing this module's functions.
    arena: Arena<Function>,
}

impl ModuleFunctions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, func: Function) -> FunctionId {
        self.arena.alloc(func)
    }

    pub fn get(&self, id: FunctionId) -> &Function {
        self.arena.get(id).unwrap()
    }

    /// Get a shared reference to this module's functions.
    pub fn iter(&self) -> impl Iterator<Item = &Function> {
        self.arena.iter().map(|(_, f)| f)
    }
}

pub type GlobalId = Id<Global>;

pub struct GlobalBuilder {
    name: InternedIdentifier,
    ty: GlobalType,
    /// Id of the root of the instr-seq tree this global initializes
    instr_seq: InstrSeqId,
}

impl GlobalBuilder {
    pub fn new(module: &mut Module, ty: GlobalType, name: String) -> Self {
        let interner = &mut module.interner;
        let name = interner.intern(&name);

        let arena = &mut module.instr_seq_arena;
        let instr_seq_id = arena.alloc(InstrSeq::new());

        Self {
            name,
            ty,
            instr_seq: instr_seq_id,
        }
    }

    /// Get an `InstrSeqBuilder` for building this global's body.
    pub fn instr_seq(&self) -> InstrSeqBuilder {
        InstrSeqBuilder { id: self.instr_seq }
    }

    pub fn finish(self, ctx: &mut CompileCtx) {
        let GlobalBuilder {
            name,
            ty,
            instr_seq,
        } = self;
        let global = Global {
            name,
            ty,
            instr_seq,
        };
        ctx.module.global_arena.alloc(global);
    }
}
