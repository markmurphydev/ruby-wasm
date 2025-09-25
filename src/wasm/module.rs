//! attribution: Substantially copied from Walrus crate `walrus/src/module.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::wasm::function::{Function, FunctionId};
use id_arena::Arena;

/// A wasm module.
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct Module {
    // pub imports: ModuleImports,
    // pub tables: ModuleTables,
    // pub types: ModuleTypes,
    pub funcs: ModuleFunctions,
    pub globals: ModuleGlobals,
    // pub locals: ModuleLocals,
    // pub exports: ModuleExports,
    // pub memories: ModuleMemories,
    // /// Registration of passive data segments, if any
    // pub data: ModuleData,
    // /// Registration of passive element segments, if any
    // pub elements: ModuleElements,
    /// The `start` function, if any
    pub start: Option<FunctionId>,
    // /// Representation of the eventual custom section, `producers`
    // pub producers: ModuleProducers,
    // /// Custom sections found in this module.
    // pub customs: ModuleCustomSections,
    // /// Dwarf debug data.
    // pub debug: ModuleDebugData,
    /// The name of this module, used for debugging purposes in the `name`
    /// custom section.
    pub name: Option<String>,
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


/// The set of globals in each function in this module.
#[derive(Debug, Default)]
pub struct ModuleGlobals {
    /// The arena where the globals are stored.
    arena: Arena<Global>,
}

impl ModuleGlobals {
    /// Adds a new imported global to this list.
    pub fn add_import(
        &mut self,
        ty: ValType,
        mutable: bool,
        shared: bool,
        import_id: ImportId,
    ) -> GlobalId {
        self.arena.alloc_with_id(|id| Global {
            id,
            ty,
            mutable,
            shared,
            kind: GlobalKind::Import(import_id),
            name: None,
        })
    }

    /// Construct a new global, that does not originate from any of the input
    /// wasm globals.
    pub fn add_local(
        &mut self,
        ty: ValType,
        mutable: bool,
        shared: bool,
        init: ConstExpr,
    ) -> GlobalId {
        self.arena.alloc_with_id(|id| Global {
            id,
            ty,
            mutable,
            shared,
            kind: GlobalKind::Local(init),
            name: None,
        })
    }

    /// Gets a reference to a global given its id
    pub fn get(&self, id: GlobalId) -> &Global {
        &self.arena[id]
    }

    /// Gets a reference to a global given its id
    pub fn get_mut(&mut self, id: GlobalId) -> &mut Global {
        &mut self.arena[id]
    }

    /// Removes a global from this module.
    ///
    /// It is up to you to ensure that any potential references to the deleted
    /// global are also removed, eg `get_global` expressions.
    pub fn delete(&mut self, id: GlobalId) {
        self.arena.delete(id);
    }

    /// Get a shared reference to this module's globals.
    pub fn iter(&self) -> impl Iterator<Item = &Global> {
        self.arena.iter().map(|(_, f)| f)
    }
}
