//! Ruby values lowered to a union of all possible values.
//! Converted into Wasm `(ref eq)` subtypes

use pretty::RcDoc;
use serde::Serialize;
use wasmtime::{AnyRef, Rooted};
use wat_defs::ty::RefType;
use wat_macro::wat;

/// `wasmtime`'s Rust-side representation of a Wasm `(ref eq)` value
pub type WasmtimeRefEq = Rooted<AnyRef>;

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub enum Unitype {
    True,
    False,
    Nil,
    // Internally, stored as the actual number, without the marker bit
    Fixnum(Fixnum),
    HeapNum(i64),
    String(String),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct Fixnum(i32);

impl Unitype {
    /// Wasm-supertype of all Ruby values
    /// ≡ `(ref eq)`
    pub fn unitype() -> RefType {
        wat![ (ref eq) ]
    }

    /// `(ref i31)`
    // pub const REF_I31: RefType = RefType::new_abstract(AbsHeapType::I31, Nullability::NonNullable);

    pub const STRING_TYPE_IDENTIFIER: &'static str = "str";
    // pub const STRING_TYPE: CompType = CompType::Array(ArrayType {
    //     field: FieldType {
    //         mutability: Mutability::Const,
    //         ty: PackType::I8.into_storage_type(),
    //     },
    // });
    // pub fn string_ref_type() -> RefType {
    //     RefType {
    //         nullable: Nullability::NonNullable,
    //         heap_type: HeapType::Identifier(Self::STRING_TYPE_IDENTIFIER.to_string()),
    //     }
    // }
    // 
    // /// Global<Unitype>
    // pub const GLOBAL_CONST_TYPE: GlobalType = GlobalType {
    //     mutable: Mutability::Const,
    //     val_type: Self::UNITYPE.into_val_type(),
    // };
    // 
    // /// mut Global<Unitype>
    // pub const GLOBAL_MUT_TYPE: GlobalType = GlobalType {
    //     mutable: Mutability::Mut,
    //     val_type: Self::UNITYPE.into_val_type(),
    // };

    /// We give fixnums half an i31, marking MSB 1
    /// (0b1xx_xxxx...): i31
    pub const FIXNUM_BIT_WIDTH: u32 = 30;

    /// Fixnums are identified with a 1 in the MSB of the i31
    pub const FIXNUM_MARKER: i32 = 1 << 30;

    pub const FIXNUM_MASK: u32 = u32::MAX >> (u32::BITS - Self::FIXNUM_BIT_WIDTH);
    pub const FIXNUM_TOP_BIT_MASK: u32 = 1 << (Self::FIXNUM_BIT_WIDTH - 1);

    pub const FALSE_BIT_PATTERN: i32 = 0b0001;
    pub const TRUE_BIT_PATTERN: i32 = 0b0011;
    pub const NIL_BIT_PATTERN: i32 = 0b0101;

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
            n if bit_width(n) <= Self::FIXNUM_BIT_WIDTH => {
                Unitype::Fixnum(Fixnum(i32::try_from(n).unwrap()))
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
    pub fn parse_ref_eq(ref_eq: WasmtimeRefEq, store: &mut wasmtime::Store<()>) -> Self {
        let is_i31 = ref_eq.is_i31(&store).unwrap();
        if is_i31 {
            let value = ref_eq.unwrap_i31(&store).unwrap().get_u32() as i32;
            match value {
                Self::FALSE_BIT_PATTERN => Self::False,
                Self::TRUE_BIT_PATTERN => Self::True,
                Self::NIL_BIT_PATTERN => Self::Nil,
                val if (val & Self::FIXNUM_MARKER) != 0 => Self::Fixnum(Fixnum(val & !Self::FIXNUM_MARKER)),
                _ => panic!("Invalid i31 bit pattern 0b{:b}", value),
            }
        } else {
            match ref_eq {
                arr if ref_eq.is_array(&store).unwrap() => {
                    // TODO -- assuming all arrays are string arrays
                    let arr = arr.as_array(&store).unwrap().unwrap();
                    let bytes: Vec<u8> = arr
                        .elems(store)
                        .unwrap()
                        .map(|byte| {
                            // `arr.elems` zero-extends `i8` and `i16` into `Val::I32`
                            let byte = byte.i32().unwrap();
                            byte as u8
                        })
                        .collect();
                    let string = String::from_utf8(bytes).unwrap();
                    Unitype::String(string)
                }
                ref_eq => {
                    panic!("Unknown type: {:?}", ref_eq.ty(&store))
                },
            }
        }
    }

    pub fn to_i31_bits(self) -> i32 {
        match self {
            Unitype::True => Self::TRUE_BIT_PATTERN,
            Unitype::False => Self::FALSE_BIT_PATTERN,
            Unitype::Nil => Self::NIL_BIT_PATTERN,
            Unitype::Fixnum(Fixnum(val)) => val | Self::FIXNUM_MARKER,
            Unitype::HeapNum(_) => panic!("Not an i31 value: {:?}", self),
            Unitype::String(_) => panic!("Not an i31 value: {:?}", self),
        }
    }

    pub fn to_pretty(self) -> String {
        let mut w = Vec::new();
        self.module_to_doc().render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    fn module_to_doc(self) -> RcDoc<'static> {
        let text = match self {
            Unitype::True => "true".to_owned(),
            Unitype::False => "false".to_owned(),
            Unitype::Nil => "nil".to_owned(),
            Unitype::Fixnum(Fixnum(n)) => format!("{}", n),
            Unitype::HeapNum(n) => format!("{}", n),
            Unitype::String(s) => format!("\"{}\"", s),
        };
        RcDoc::text(text)
    }
}
