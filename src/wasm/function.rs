//! attribution: Adapted from Walrus crate `walrus/src/{function.rs, function_builder.rs}`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::compiler::CompileCtx;
use crate::wasm::instr_seq::{InstrSeq, InstrSeqBuilder, InstrSeqId};
use crate::wasm::intern::InternedIdentifier;
use crate::wasm::module::ModuleFunctions;
use crate::wasm::types::{ParamsType, ResultsType};
use id_arena::Id;

pub type FunctionId = Id<Function>;

#[derive(Debug)]
pub struct Function {
    pub name: InternedIdentifier,
    pub exported: ExportStatus,
    pub params: ParamsType,
    pub results: ResultsType,
    /// The entry-point into this function.
    pub entry_point: InstrSeqId,
}

/// Is this item exported?
#[derive(Debug, Copy, Clone)]
pub enum ExportStatus {
    Exported,
    NotExported,
}

/// Build instances of `LocalFunction`.
#[derive(Debug)]
pub struct FunctionBuilder {
    name: InternedIdentifier,
    exported: ExportStatus,
    params: ParamsType,
    results: ResultsType,
    /// The entry-point into this function.
    entry_point: InstrSeqId,
}

impl FunctionBuilder {
    /// Creates a new, empty function builder.
    pub fn new(
        ctx: &mut CompileCtx<'_>,
        name: &str,
        exported: ExportStatus,
        params: ParamsType,
        results: ResultsType,
    ) -> Self {
        let name = ctx.module.interner.intern(name);
        let entry_point = ctx.module.instr_seq_arena.alloc(InstrSeq::new());

        Self {
            name,
            exported,
            params,
            results,
            entry_point,
        }
    }

    /// Get a `InstrSeqBuilder` for building and mutating this function's body.
    pub fn func_body(&self) -> InstrSeqBuilder {
        // `InstrSeqBuilder` just wraps a `InstrSeqId`.
        //  any two with the same ID write to the same instr_seq.
        InstrSeqBuilder {
            id: self.entry_point,
        }
    }
    /// Finishes this builder, wrapping it all up and inserting it into the
    /// specified `Module`.
    pub fn finish(self, funcs: &mut ModuleFunctions) -> FunctionId {
        let FunctionBuilder {
            name,
            exported,
            params,
            results,
            entry_point,
        } = self;
        let func = Function {
            name,
            exported,
            params,
            results,
            entry_point,
        };
        funcs.add(func)
    }
}

