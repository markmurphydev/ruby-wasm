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
pub enum Null {
    NotNull,
    Null,
}

#[derive(Debug)]
pub struct RefType {
    null: Null,
    heap_type: HeapType,
}

#[derive(Debug)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug)]
pub enum BlockType {
    Result(ResultType),
    TypeUse(String),
}

#[derive(Debug)]
pub struct ResultType(ValType);