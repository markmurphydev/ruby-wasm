//! TODO -- I want `Type` to be `copy`, but I don't know if function types will fuck that up
//!     Probably interning fixes everything?

use crate::arena_set::ArenaSet;
use id_arena::Id;

/// Wasm-supertype of all Ruby values
/// ≡ `(ref eq)`
pub const UNITYPE: Type = Type::ReferenceType(ReferenceType {
    null: false,
    heap_type: HeapType::Eq,
});

/// Identifier for types.
/// Replaces `<TYPEIDX>` from Wasm WAT spec.
pub type TypeId = Id<NamedType>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedType {
    pub ty: Type,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Val(ValType),
    ReferenceType(ReferenceType),
    SubType(SubType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValType {
    NumberType(NumberType),
    // VecType,
    // RefType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberType {
    /// Ambiguously-signed 32-bit integer.
    /// TODO -- Currently we always treat as signed
    I32,
    /// Ambiguously-signed 32-bit integer
    /// TODO -- Currently we always treat as signed
    I64,
    F32,
    F64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub value_type: ValType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mutability {
    Const,
    Var,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReferenceType {
    pub null: bool,
    pub heap_type: HeapType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HeapType {
    Eq,
}

/// The type of an instruction sequence
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockType {
    Val(ValType),
    Id(TypeId), // Should be SubType<Function>
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
    pub supertypes: Option<Box<[TypeId]>>, // Should be SubType
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
    pub params: FuncParams,
    pub results: FuncResults,
}

pub type FuncParams = Box<[FuncParam]>;
pub type FuncResults = Box<[FuncResult]>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FuncParam {
    pub name: Option<String>,
    pub ty: ValType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FuncResult {
    name: Option<String>,
    ty: ValType,
}
