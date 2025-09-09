use crate::wasm::Instruction;
use std::fmt::{Display, Formatter};

pub struct Byte(pub u8);

// TODO -- ugggh
#[derive(Debug, Copy, Clone)]
pub struct Integer(pub u32);

impl Integer {
    // Constant data types are represented as `i31` tagged pointers.
    // TODO -- This is not the final resting place of these.
    pub const FALSE: Integer = Integer(0b0001);
    pub const TRUE: Integer = Integer(0b0011);
    pub const NIL: Integer = Integer(0b0111);
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
