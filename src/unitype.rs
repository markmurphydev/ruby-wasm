//! Ruby values lowered to a union of all possible values.
//! Converted into Wasm `(ref eq)` subtypes

use pretty::RcDoc;
use serde::Serialize;
use wasmtime as WT;
use wasmtime::{AnyRef, AsContextMut, ExternRef, OwnedRooted, RootScope, Rooted, RootedGcRef};
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
    Array(Vec<Unitype>),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct Fixnum(i32);

impl Unitype {
    /// Wasm-supertype of all Ruby values
    /// ≡ `(ref eq)`
    pub fn unitype() -> RefType {
        wat![(ref eq)]
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
    pub const FIXNUM_MIN_VAL: i64 = -2i64.pow(Self::FIXNUM_BIT_WIDTH - 1);
    pub const FIXNUM_MAX_VAL: i64 = 2i64.pow(Self::FIXNUM_BIT_WIDTH - 1) - 1;

    /// Fixnums are identified with a 1 in the MSB of the i31
    pub const FIXNUM_MARKER: i32 = 1 << 30;

    pub const FIXNUM_MASK: i32 = i32::MAX >> (i32::BITS - Self::FIXNUM_BIT_WIDTH);
    pub const FIXNUM_TOP_BIT_MASK: i32 = 1 << (Self::FIXNUM_BIT_WIDTH - 1);

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

        match n {
            n if Self::FIXNUM_MIN_VAL <= n && n <= Self::FIXNUM_MAX_VAL => {
                Unitype::Fixnum(Fixnum(i32::try_from(n).unwrap()))
            }
            n if n <= i64::MAX => Unitype::HeapNum(n),
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
    pub fn parse_ref_eq(
        ref_eq: impl RootedGcRef<AnyRef>,
        mut store: &mut impl AsContextMut,
    ) -> Self {
        let is_i31 = ref_eq.is_i31(&store).unwrap();
        if is_i31 {
            let value = ref_eq.unwrap_i31(&store).unwrap().get_u32() as i32;
            Self::from_i31_bits(value)
        } else {
            match ref_eq {
                arr if ref_eq.is_array(&store).unwrap() => {
                    // TODO -- assuming all arrays are string arrays
                    let arr = arr.as_array(&store).unwrap().unwrap();
                    let is_string =
                        arr.ty(&store)
                            .unwrap()
                            .field_type()
                            .matches(&wasmtime::FieldType::new(
                                wasmtime::Mutability::Const,
                                wasmtime::StorageType::I8,
                            ));
                    if is_string {
                        let bytes: Vec<u8> = arr
                            .elems(&mut store)
                            .unwrap()
                            .map(|byte| {
                                // `arr.elems` zero-extends `i8` and `i16` into `Val::I32`
                                let byte = byte.i32().unwrap();
                                byte as u8
                            })
                            .collect();
                        let string = String::from_utf8(bytes).unwrap();
                        Unitype::String(string)
                    } else {
                        let mut unitype_elems = vec![];
                        let len = arr.len(&store).unwrap();
                        for idx in 0..len {
                            let val = arr.get(&mut store, idx).unwrap();
                            let val = val
                                .unwrap_any_ref()
                                .unwrap()
                                .to_owned_rooted(&mut store)
                                .unwrap();
                            let res = Self::parse_ref_eq(val, store);
                            unitype_elems.push(res);
                        }
                        Unitype::Array(unitype_elems)
                    }
                }
                strukt if strukt.is_struct(&store).unwrap() => {
                    let strukt = strukt.as_struct(&store).unwrap().unwrap();
                    if let Some(n) = strukt.field(store, 0).ok().and_then(|f| f.i64()) {
                        Unitype::HeapNum(n)
                    } else {
                        todo!("Unknown struct type {:?}", strukt)
                    }
                }
                other => {
                    panic!("Unknown type: {:?}", other.ty(&store))
                }
            }
        }
    }

    pub fn to_i31_bits(self) -> i32 {
        match self {
            Unitype::True => Self::TRUE_BIT_PATTERN,
            Unitype::False => Self::FALSE_BIT_PATTERN,
            Unitype::Nil => Self::NIL_BIT_PATTERN,
            Unitype::Fixnum(Fixnum(val)) => val | Self::FIXNUM_MARKER,
            Unitype::HeapNum(_) | Unitype::String(_) | Unitype::Array(_) => {
                panic!("Not an i31 value: {:?}", self)
            }
        }
    }

    pub fn from_i31_bits(value: i32) -> Self {
        match value {
            Self::FALSE_BIT_PATTERN => Self::False,
            Self::TRUE_BIT_PATTERN => Self::True,
            Self::NIL_BIT_PATTERN => Self::Nil,
            val if (val & Self::FIXNUM_MARKER) != 0 => {
                let val = val & !Self::FIXNUM_MARKER;
                // sign-extend
                let extend_width = i32::BITS - Self::FIXNUM_BIT_WIDTH;
                let val = (val << extend_width) >> extend_width;
                Self::Fixnum(Fixnum(val))
            }
            _ => panic!("Invalid i31 bit pattern 0b{:b}", value),
        }
    }

    pub fn to_pretty(self) -> String {
        let mut w = Vec::new();
        self.module_to_doc().render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    fn module_to_doc(self) -> RcDoc<'static> {
        match self {
            Unitype::True => RcDoc::text("true".to_owned()),
            Unitype::False => RcDoc::text("false".to_owned()),
            Unitype::Nil => RcDoc::text("nil".to_owned()),
            Unitype::Fixnum(Fixnum(n)) => RcDoc::text(format!("{}", n)),
            Unitype::HeapNum(n) => RcDoc::text(format!("{}", n)),
            Unitype::String(s) => RcDoc::text(format!("\"{}\"", s)),
            Unitype::Array(vals) => RcDoc::text("[")
                .append(RcDoc::intersperse(
                    vals.into_iter().map(Self::module_to_doc),
                    RcDoc::text(",").append(RcDoc::line()),
                ))
                .append(RcDoc::text("]"))
                .nest(2)
                .group(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unitype::{Fixnum, Unitype};

    #[test]
    fn negative_fixnum_round_trip() {
        let expected = Unitype::Fixnum(Fixnum(-22));
        let actual = Unitype::from_i31_bits(expected.clone().to_i31_bits());
        assert_eq!(expected, actual);
    }
}
