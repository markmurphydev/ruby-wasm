//! TODO -- I want `Type` to be `copy`, but I don't know if function types will fuck that up
//!     Probably interning fixes everything?

use crate::arena_set::ArenaSet;
use id_arena::Id;
use crate::wasm::intern::InternedIdentifier;

/// Wasm-supertype of all Ruby values
/// ≡ `(ref eq)`
pub const UNITYPE: RefType = RefType::new_abstract(AbsHeapType::Eq, false);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValType {
    NumberType(NumberType),
    // VecType,
    Ref(RefType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberType {
    /// Ambiguously-signed 32-bit integer.
    I32,
    /// Ambiguously-signed 32-bit integer
    I64,
    F32,
    F64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub value_type: ValType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefType {
    pub nullable: bool,
    pub heap_type: HeapType,
}

// attribution: Impl adapted from `wasm-encoder` crate `wasm-encoder/src/core/types.rs`
// https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder
// MIT licensed.
impl RefType {
    /// Alias for the `anyref` type in WebAssembly.
    pub const ANYREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Any),
    };

    /// Alias for the `eqref` type in WebAssembly.
    pub const EQREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Eq),
    };

    /// Alias for the `funcref` type in WebAssembly.
    pub const FUNCREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Func),
    };

    /// Alias for the `externref` type in WebAssembly.
    pub const EXTERNREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Extern),
    };

    /// Alias for the `i31ref` type in WebAssembly.
    pub const I31REF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::I31),
    };

    /// Alias for the `arrayref` type in WebAssembly.
    pub const ARRAYREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Array),
    };

    /// Alias for the `exnref` type in WebAssembly.
    pub const EXNREF: RefType = RefType {
        nullable: true,
        heap_type: HeapType::Abstract(AbsHeapType::Exn),
    };

    /// Create a new abstract reference type.
    pub const fn new_abstract(ty: AbsHeapType, nullable: bool) -> Self {
        Self {
            nullable,
            heap_type: HeapType::Abstract(ty)
        }
    }

    /// Set the nullability of this reference type.
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    pub fn into_val_type(self) -> ValType {
        ValType::Ref(self)
    }

    pub fn into_result_type(self) -> ResultType {
        ResultType(self.into_val_type())
    }

    pub fn into_block_type_result(self) -> BlockType {
        BlockType::Result(self.into_result_type())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HeapType {
    Abstract(AbsHeapType),
    Identifier(String),
}

/// An abstract heap type.
/// attribution: Copied from `wasm-encoder` crate `wasm-encoder/src/core/types.rs`
/// https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder
/// MIT licensed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AbsHeapType {
    /// Untyped (any) function.
    Func,

    /// The abstract external heap type.
    Extern,

    /// The abstract `any` heap type.
    ///
    /// The common supertype (a.k.a. top) of all internal types.
    Any,

    /// The abstract `none` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all internal types.
    None,

    /// The abstract `noextern` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all external types.
    NoExtern,

    /// The abstract `nofunc` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all function types.
    NoFunc,

    /// The abstract `eq` heap type.
    ///
    /// The common supertype of all referenceable types on which comparison
    /// (ref.eq) is allowed.
    Eq,

    /// The abstract `struct` heap type.
    ///
    /// The common supertype of all struct types.
    Struct,

    /// The abstract `array` heap type.
    ///
    /// The common supertype of all array types.
    Array,

    /// The unboxed `i31` heap type.
    I31,

    /// The abstract `exception` heap type.
    Exn,

    /// The abstract `noexn` heap type.
    NoExn,
}

/// The type of an instruction sequence
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockType {
    Result(ResultType),
    TypeUse(String),
}

/// A composite type with an optional list of supertypes it matches.
///     Can be marked `final` to prevent further subtyping.
///
/// ```ebnf
/// <SUBTYPE> ::= (sub final? supertypes:<TYPEIDX>* <COMPTYPE>)
///             | (sub final <COMPTYPE>)                // Final subtype with no supertype
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubType {
    pub is_final: bool,
    pub supertypes: Option<Vec<Box<SubType>>>,
    pub comp_type: CompType,
}

impl SubType {
    pub fn final_no_super(comp_type: CompType) -> Self {
        Self {
            is_final: true,
            supertypes: None,
            comp_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompType {
    FuncType(FuncType),
}

/// Type of a function
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub params: ParamsType,
    pub results: ResultsType,
}

pub type ParamsType = Box<[ParamType]>;
pub type ResultsType = Box<[ResultType]>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamType {
    pub name: String,
    pub ty: ValType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResultType(pub ValType);
