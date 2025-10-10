//! attribution: Adapted from Walrus crate `walrus/src/{function.rs, function_builder.rs}`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use crate::compiler::CompileCtx;
use crate::wasm::instr_seq::{InstrSeq, InstrSeqBuilder, InstrSeqId};
use crate::wasm::intern::InternedIdentifier;
use crate::wasm::module::ModuleFunctions;
use crate::wasm::types::{ParamsType, ResultsType, ValType};
use id_arena::Id;
use wasmtime::component::__internal::wasmtime_environ::FunctionType;

pub type FunctionId = Id<Function>;

#[derive(Debug)]
pub struct Function {
    pub name: InternedIdentifier,
    pub exported: ExportStatus,
    pub type_use: Option<String>,
    pub params: ParamsType,
    pub results: ResultsType,
    /// The entry-point into this function.
    pub entry_point: InstrSeqId,
    pub locals: Vec<Local>,
}

/// Is this item exported?
#[derive(Debug, Copy, Clone)]
pub enum ExportStatus {
    Exported,
    NotExported,
}

#[derive(Debug)]
pub struct Local {
    pub identifier: String,
    pub ty: ValType,
}

/// Build instances of `LocalFunction`.
#[derive(Debug)]
pub struct FunctionBuilder {
    name: InternedIdentifier,
    exported: ExportStatus,
    type_use: Option<String>,
    params: ParamsType,
    results: ResultsType,
    locals: Vec<Local>,
    /// The entry-point into this function.
    entry_point: InstrSeqId,
}

impl FunctionBuilder {
    /// Creates a new, empty function builder.
    pub fn new(
        ctx: &mut CompileCtx<'_>,
        name: &str,
        exported: ExportStatus,
        type_use: Option<&str>,
        params: ParamsType,
        results: ResultsType,
        locals: Vec<Local>,
    ) -> Self {
        let name = ctx.module.interner.intern(name);
        let entry_point = ctx.module.instr_seq_arena.alloc(InstrSeq::new());

        Self {
            name,
            exported,
            params,
            type_use: type_use.map(|t| t.to_string()),
            results,
            entry_point,
            locals,
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
            type_use,
            params,
            results,
            entry_point,
            locals,
        } = self;
        let func = Function {
            name,
            exported,
            type_use,
            params,
            results,
            entry_point,
            locals
        };
        funcs.add(func)
    }
}
