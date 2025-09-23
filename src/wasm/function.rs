//! Functions within a wasm module.

use std::cmp;
use std::collections::BTreeMap;
use std::ops::Range;
use id_arena::{Arena, Id};
use crate::FunctionBuilder;
use crate::wasm::{InstrSeq, InstrSeqId, LocalId};
use crate::wasm::module::Module;
use crate::wasm::types::{TypeId, ValType};

/// A function identifier.
pub type FunctionId = Id<Function>;

// /// Parameter(s) to a function
// pub type FuncParams = Box<[ValType]>;
//
// /// Result(s) of a given function
// pub type FuncResults = Vec<ValType>;

/// A wasm function.
///
/// Either defined locally or externally and then imported; see `FunctionKind`.
#[derive(Debug)]
pub struct Function {
    // NB: Not public so that it can't get out of sync with the arena that this
    // function lives within.
    id: FunctionId,

    /// The kind of function this is.
    pub kind: FunctionKind,

    /// An optional name associated with this function
    pub name: Option<String>,
}

impl Function {
    fn new_uninitialized(id: FunctionId, ty: TypeId) -> Function {
        Function {
            id,
            kind: FunctionKind::Uninitialized(ty),
            name: None,
        }
    }

    /// Get this function's identifier.
    pub fn id(&self) -> FunctionId {
        self.id
    }

    // /// Get this function's type's identifier.
    // pub fn ty(&self) -> TypeId {
    //     match &self.kind {
    //         FunctionKind::Local(l) => l.ty(),
    //         FunctionKind::Import(i) => i.ty,
    //         FunctionKind::Uninitialized(t) => *t,
    //     }
    // }
}

/// The local- or external-specific bits of a function.
#[derive(Debug)]
pub enum FunctionKind {
    /// An externally defined, imported wasm function.
    Import(ImportedFunction),

    /// A locally defined wasm function.
    Local(LocalFunction),

    /// A locally defined wasm function that we haven't parsed yet (but have
    /// reserved its id and associated it with its original input wasm module
    /// index). This should only exist within
    /// `ModuleFunctions::add_local_functions`.
    Uninitialized(TypeId),
}

impl FunctionKind {
    /// Get the underlying `FunctionKind::Import` or panic if this is not an
    /// import function
    pub fn unwrap_import(&self) -> &ImportedFunction {
        match self {
            FunctionKind::Import(import) => import,
            _ => panic!("not an import function"),
        }
    }

    /// Get the underlying `FunctionKind::Local` or panic if this is not a local
    /// function.
    pub fn unwrap_local(&self) -> &LocalFunction {
        match self {
            FunctionKind::Local(l) => l,
            _ => panic!("not a local function"),
        }
    }

    /// Get the underlying `FunctionKind::Import` or panic if this is not an
    /// import function
    pub fn unwrap_import_mut(&mut self) -> &mut ImportedFunction {
        match self {
            FunctionKind::Import(import) => import,
            _ => panic!("not an import function"),
        }
    }

    /// Get the underlying `FunctionKind::Local` or panic if this is not a local
    /// function.
    pub fn unwrap_local_mut(&mut self) -> &mut LocalFunction {
        match self {
            FunctionKind::Local(l) => l,
            _ => panic!("not a local function"),
        }
    }
}

/// An externally defined, imported function.
#[derive(Debug)]
pub struct ImportedFunction {
    // /// The import that brings this function into the module.
    // pub import: ImportId,
    // /// The type signature of this imported function.
    // pub ty: TypeId,
}

/// A function defined locally within the wasm module.
#[derive(Debug)]
pub struct LocalFunction {
    /// All of this function's instructions, contained in the arena.
    builder: FunctionBuilder,

    /// Arguments to this function, and the locals that they're assigned to.
    pub args: Vec<LocalId>,

    // /// InstrSeqId list mapping to original instruction
    // pub instruction_mapping: Vec<(usize, InstrLocId)>,

    /// Original function binary range.
    pub original_range: Option<Range<usize>>,
}

impl LocalFunction {
    /// Creates a new definition of a local function from its components.
    pub(crate) fn new(args: Vec<LocalId>, builder: FunctionBuilder) -> LocalFunction {
        LocalFunction {
            args,
            builder,
            // instruction_mapping: Vec::new(),
            original_range: None,
        }
    }

    /// Get this function's type.
    #[inline]
    pub fn ty(&self) -> TypeId {
        self.builder.ty
    }

    pub(crate) fn add_block(
        &mut self,
        make_block: impl FnOnce(InstrSeqId) -> InstrSeq,
    ) -> InstrSeqId {
        self.builder.arena.alloc_with_id(make_block)
    }

    /// Get the id of this function's entry block.
    pub fn entry_block(&self) -> InstrSeqId {
        self.builder.entry.unwrap()
    }

    /// Get the block associated with the given id.
    pub fn block(&self, id: InstrSeqId) -> &InstrSeq {
        &self.builder.arena[id]
    }

    /// Get the block associated with the given id.
    pub fn block_mut(&mut self, id: InstrSeqId) -> &mut InstrSeq {
        &mut self.builder.arena[id]
    }

    /// Get access to a `FunctionBuilder` to continue adding instructions to
    /// this function.
    pub fn builder(&self) -> &FunctionBuilder {
        &self.builder
    }

    // /// Get access to a `FunctionBuilder` to continue adding instructions to
    // /// this function.
    // pub fn builder_mut(&mut self) -> &mut FunctionBuilder {
    //     &mut self.builder
    // }
    //
    // /// Collect the set of data segments that are used in this function via
    // /// `memory.init` or `data.drop` instructions.
    // pub fn used_data_segments(&self) -> IdHashSet<Data> {
    //     let mut visitor = DataSegmentsVisitor::default();
    //     dfs_in_order(&mut visitor, self, self.entry_block());
    //     return visitor.segments;
    //
    //     #[derive(Default)]
    //     struct DataSegmentsVisitor {
    //         segments: IdHashSet<Data>,
    //     }
    //
    //     impl<'a> Visitor<'a> for DataSegmentsVisitor {
    //         fn visit_data_id(&mut self, id: &DataId) {
    //             self.segments.insert(*id);
    //         }
    //     }
    // }
    //
    // fn used_locals(&self) -> IdHashSet<Local> {
    //     let mut locals = Used::default();
    //     dfs_in_order(&mut locals, self, self.entry_block());
    //     return locals.locals;
    //
    //     #[derive(Default)]
    //     struct Used {
    //         locals: IdHashSet<Local>,
    //     }
    //
    //     impl<'a> Visitor<'a> for Used {
    //         fn visit_local_id(&mut self, id: &LocalId) {
    //             self.locals.insert(*id);
    //         }
    //     }
    // }
}

/// The set of functions within a module.
#[derive(Debug, Default)]
pub struct ModuleFunctions {
    /// The arena containing this module's functions.
    arena: Arena<Function>,
}

impl ModuleFunctions {
    /// Construct a new, empty set of functions for a module.
    pub fn new() -> ModuleFunctions {
        Default::default()
    }

    // /// Create a new externally defined, imported function.
    // pub fn add_import(&mut self, ty: TypeId, import: ImportId) -> FunctionId {
    //     self.arena.alloc_with_id(|id| Function {
    //         id,
    //         kind: FunctionKind::Import(ImportedFunction { import, ty }),
    //         name: None,
    //     })
    // }

    /// Create a new internally defined function
    pub fn add_local(&mut self, func: LocalFunction) -> FunctionId {
        let func_name = func.builder().name.clone();
        self.arena.alloc_with_id(|id| Function {
            id,
            kind: FunctionKind::Local(func),
            name: func_name,
        })
    }

    /// Gets a reference to a function given its id
    pub fn get(&self, id: FunctionId) -> &Function {
        &self.arena[id]
    }

    /// Gets a reference to a function given its id
    pub fn get_mut(&mut self, id: FunctionId) -> &mut Function {
        &mut self.arena[id]
    }

    /// Get a function ID by its name.
    ///
    /// The name used is the "name" custom section name and *not* the export
    /// name, if a function happens to be exported.
    ///
    /// Note that function names are *not* guaranteed to be unique. This will
    /// return the first function in the module with the given name.
    pub fn by_name(&self, name: &str) -> Option<FunctionId> {
        self.arena.iter().find_map(|(id, f)| {
            if f.name.as_deref() == Some(name) {
                Some(id)
            } else {
                None
            }
        })
    }


    /// Get a shared reference to this module's functions.
    pub fn iter(&self) -> impl Iterator<Item = &Function> {
        self.arena.iter().map(|(_, f)| f)
    }

    /// Get an iterator of this module's local functions
    pub fn iter_local(&self) -> impl Iterator<Item = (FunctionId, &LocalFunction)> {
        self.iter().filter_map(|f| match &f.kind {
            FunctionKind::Local(local) => Some((f.id(), local)),
            _ => None,
        })
    }

    /// Get a mutable reference to this module's functions.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Function> {
        self.arena.iter_mut().map(|(_, f)| f)
    }

    /// Get an iterator of this module's local functions
    pub fn iter_local_mut(&mut self) -> impl Iterator<Item = (FunctionId, &mut LocalFunction)> {
        self.iter_mut().filter_map(|f| {
            let id = f.id();
            match &mut f.kind {
                FunctionKind::Local(local) => Some((id, local)),
                _ => None,
            }
        })
    }
}
