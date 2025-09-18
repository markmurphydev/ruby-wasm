//! TODO -- I want `Type` to be `copy`, but I don't know if function types will fuck that up
//!     Probably interning fixes everything?

/// Wasm-supertype of all Ruby values
/// ≡ `(ref eq)`
pub const UNITYPE: Type = Type::ReferenceType(ReferenceType {
    null: false,
    heap_type: HeapType::Eq,
});

#[derive(Debug, Clone)]
pub enum Type {
    Value(ValueType),
    ReferenceType(ReferenceType),
}

#[derive(Debug, Clone)]
pub enum ValueType {
    NumberType(NumberType),
    // VecType,
    // RefType,
}

#[derive(Debug, Clone)]
pub enum NumberType {
    /// Ambiguously-signed 32-bit integer.
    /// TODO -- Currently we always treat as signed
    I32,
    /// Ambiguously-signed 32-bit integer
    /// TODO -- Currently we always treat as signed
    I64,
    F32,
    F64
}

#[derive (Debug, Clone)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub value_type: ValueType
}

#[derive (Debug, Clone)]
pub enum Mutability {
    Const,
    Var
}

#[derive (Debug, Clone)]
pub struct ReferenceType {
    pub null: bool,
    pub heap_type: HeapType,
}

#[derive (Debug, Clone)]
pub enum HeapType {
    Eq,
}
