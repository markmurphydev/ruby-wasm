//! attribution: Substantially copied from Walrus crate `walrus/src/module.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::arena_set::ArenaSet;
use crate::wasm::function::{FunctionId, ModuleFunctions};
use crate::wasm::types::{NamedType, TypeId};

/// A wasm module.
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct Module {
    // pub imports: ModuleImports,
    // pub tables: ModuleTables,
    pub types: ModuleTypes,
    pub funcs: ModuleFunctions,
    // pub globals: ModuleGlobals,
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

/// The set of de-duplicated types within a module.
/// TODO: Currently assuming all typedefs in _separate_ rectypes
#[derive(Debug, Default)]
pub struct ModuleTypes {
    arena: ArenaSet<NamedType>,
}

impl ModuleTypes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a type associated with an ID
    pub fn get(&self, id: TypeId) -> &NamedType {
        &self.arena[id]
    }

    /// Get a type associated with an ID
    pub fn get_mut(&mut self, id: TypeId) -> &mut NamedType {
        &mut self.arena[id]
    }

    // /// Get a type ID by its name.
    // ///
    // /// This is currently only intended for in-memory modifications, and by
    // /// default will always return `None` for a newly parsed module. A
    // /// hypothetical future WAT text format to `walrus::Module` parser could
    // /// preserve type names from the WAT.
    // pub fn by_name(&self, name: &str) -> Option<TypeId> {
    //     self.arena.iter().find_map(|(id, ty)| {
    //         if ty.name.as_deref() == Some(name) {
    //             Some(id)
    //         } else {
    //             None
    //         }
    //     })
    // }

    // /// Get a shared reference to this module's types.
    // pub fn iter(&self) -> impl Iterator<Item = &SubType> {
    //     self.arena.iter().map(|(_, f)| f)
    // }

    /// Add a new type to this module, and return its `Id`
    pub fn add(&mut self, ty: NamedType) -> TypeId {
        self.arena.insert(ty)
    }

    // /// Find the existing type for the given parameters and results.
    // pub fn find(&self, params: &[ValType], results: &[ValType]) -> Option<TypeId> {
    //     self.arena.iter().find_map(|(id, ty)| {
    //         if !ty.is_for_function_entry() && ty.params() == params && ty.results() == results {
    //             Some(id)
    //         } else {
    //             None
    //         }
    //     })
    // }
    //
    // pub(crate) fn find_for_function_entry(&self, results: &[ValType]) -> Option<TypeId> {
    //     self.arena.iter().find_map(|(id, ty)| {
    //         if ty.is_for_function_entry() && ty.params().is_empty() && ty.results() == results {
    //             Some(id)
    //         } else {
    //             None
    //         }
    //     })
    // }
}
