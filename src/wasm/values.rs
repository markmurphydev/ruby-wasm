use std::fmt::{Display, Formatter};

pub struct Byte(pub u8);

// TODO -- ugggh
#[derive(Debug, Copy, Clone)]
pub struct Integer(pub u64);

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
