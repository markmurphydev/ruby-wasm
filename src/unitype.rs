//! Ruby values lowered to a union of all possible values.
//! Converted into Wasm `(ref eq)` subtypes
use crate::wasm::types::Nullability::NonNullable;
use crate::wasm::types::{AbsHeapType, RefType};
use wasmtime::{AnyRef, Rooted};

/// We give fixnums half an i31, marking MSB 1
/// (0b1xx_xxxx...): i31
const FIXNUM_BIT_WIDTH: u32 = 30;

/// Fixnums are identified with a 1 in the MSB of the i31
pub const FIXNUM_MARKER: i32 = 1 << 30;

/// `wasmtime`'s Rust-side representation of a Wasm `(ref eq)` value
pub type WasmtimeRefEq = Rooted<AnyRef>;

pub enum Unitype {
    True,
    False,
    Nil,

    Fixnum(i32),
    HeapNum(i64),
}

impl Unitype {
    /// Wasm-supertype of all Ruby values
    /// ≡ `(ref eq)`
    pub const UNITYPE: RefType = RefType::new_abstract(AbsHeapType::Eq, NonNullable);

    // pub const I31_TYPE: RefType = RefType::new_abstract(AbsHeapType::I31, NonNullable);
    // pub const HEAP_NUM_TYPE: RefType = RefType

    /// Converts the given Ruby integer to a fixnum or heapnum.
    pub fn from_integer(n: i64) -> Unitype {
        // NB: Rust signed <-> unsigned casts (using `as`) reinterpret with 2's complement.
        // ```
        // assert_eq!(-1, u64::MAX as i64);
        // ```

        /// Minimum size required for 2's complement representation of the given number
        /// Strategy from:
        /// https://internals.rust-lang.org/t/add-methods-that-return-the-number-of-bits-necessary-to-represent-an-integer-in-binary-to-the-standard-library/21870/7
        fn bit_width(n: i64) -> u32 {
            i64::BITS - n.abs().leading_zeros() + 1
        }

        match n {
            n if bit_width(n) <= FIXNUM_BIT_WIDTH => {
                Unitype::Fixnum(FIXNUM_MARKER | i32::try_from(n).unwrap())
            }
            n if bit_width(n) <= i64::BITS => Unitype::HeapNum(n),
            _ => {
                todo!(
                    "Bignums not yet implemented.
                     [n={:x}] larger than W::I64",
                    22
                )
            }
        }
    }

    /// Parse a Wasm `(ref eq)` value into a `UnitypeValue`.
    /// Used only for displaying `wasmtime` output.
    pub fn parse_ref_eq(ref_eq: WasmtimeRefEq) -> Self {
        todo!()
    }
}
