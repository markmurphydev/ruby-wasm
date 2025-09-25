//! attribution: Substantially copied from Walrus crate `walrus/src/module.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed
//! TODO -- It's bad to have function-local symbol interning
//!     The way to do it would be to pass `&mut Interner` to each `____Builder` method
//!     (maybe as part of a `CompileCtx` object)

use crate::wasm::function::{ArenaProvider, Function, FunctionId, InstrSeq};
use crate::wasm::types::GlobalType;
use crate::wasm::Global;
use crate::InstrSeqBuilder;
use id_arena::{Arena, Id};

/// A wasm module.
/// Walrus allocates all the data for the module inside
///     module-owned arenas.
/// I do that where necessary to avoid owned `Box<_>`,
///     but using WAT identifiers solves some cases (e.g. locals)
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct Module {
    // pub imports: ModuleImports,
    pub funcs: ModuleFunctions,
    pub globals: ModuleGlobals,
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

/// The set of globals in each function in this module.
#[derive(Debug, Default)]
pub struct ModuleGlobals {
    /// The arena where the globals are stored.
    global_arena: Arena<Global>,
    instr_seq_arena: Arena<InstrSeq>,
}

impl ModuleGlobals {
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new global, that does not originate from any of the input
    /// wasm globals.
    pub fn add(
        &mut self,
        name: String,
        ty: GlobalType,
        instr_seq_f: impl FnOnce(&mut InstrSeqBuilder<Self>),
    ) {
        let instr_seq_id = {
            let mut builder = InstrSeqBuilder::new(self);
            instr_seq_f(&mut builder);
            builder.id
        };

        self.global_arena.alloc(Global {
            name,
            ty,
            instr_seq: instr_seq_id,
        });
    }

    /// Gets a reference to a global given its id
    pub fn get(&self, id: GlobalId) -> &Global {
        &self.global_arena[id]
    }

    /// Gets a reference to a global given its id
    pub fn get_mut(&mut self, id: GlobalId) -> &mut Global {
        &mut self.global_arena[id]
    }

    /// Get a shared reference to this module's globals.
    pub fn iter(&self) -> impl Iterator<Item = &Global> {
        self.global_arena.iter().map(|(_, f)| f)
    }
}

impl ArenaProvider for ModuleGlobals {
    fn arena(&mut self) -> &mut Arena<InstrSeq> {
        &mut self.instr_seq_arena
    }
}
