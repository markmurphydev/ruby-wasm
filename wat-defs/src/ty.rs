#[derive(Debug)]
pub enum NumType {
    I32,
}

#[derive(Debug)]
pub enum AbsHeapType {
    I31
}

#[derive(Debug)]
pub enum HeapType {
    Abs(AbsHeapType),
    TypeIdx(String),
}

#[derive(Debug)]
pub enum Nullable {
    NonNullable,
    Nullable,
}

#[derive(Debug)]
pub struct RefType {
    pub null: Nullable,
    pub heap_type: HeapType,
}

#[derive(Debug)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug)]
pub enum BlockType {
    Result(ValType),
    TypeUse(String),
}