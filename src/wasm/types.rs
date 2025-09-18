// pub type Unitype = ReferenceType;

#[derive(Debug, Clone)]
pub enum Type {
    Value(ValueType)
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
