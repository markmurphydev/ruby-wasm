use crate::wasm::Finality;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValType {
    Num(NumType),
    // VecType,
    Ref(RefType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumType {
    /// Ambiguously-signed 32-bit integer.
    I32,
    /// Ambiguously-signed 32-bit integer
    I64,
    F32,
    F64,
}

impl NumType {
    pub const fn into_val_type(self) -> ValType {
        ValType::Num(self)
    }

    pub const fn into_result_type(self) -> ResultType {
        ResultType(self.into_val_type())
    }

    pub const fn into_block_type_result(self) -> BlockType {
        BlockType::Result(self.into_result_type())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalType {
    pub mutable: Mutability,
    pub val_type: ValType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mutability {
    Const,
    Mut,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Nullability {
    NonNullable,
    Nullable,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefType {
    pub nullable: Nullability,
    pub heap_type: HeapType,
}

// attribution: Impl adapted from `wasm-encoder` crate `wasm-encoder/src/core/types.rs`
// https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder
// MIT licensed.
impl RefType {
    /// Alias for the `anyref` type in WebAssembly.
    pub const ANYREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Any),
    };

    /// Alias for the `eqref` type in WebAssembly.
    pub const EQREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Eq),
    };

    /// Alias for the `funcref` type in WebAssembly.
    pub const FUNCREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Func),
    };

    /// Alias for the `externref` type in WebAssembly.
    pub const EXTERNREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Extern),
    };

    /// Alias for the `i31ref` type in WebAssembly.
    pub const I31REF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::I31),
    };

    /// Alias for the `arrayref` type in WebAssembly.
    pub const ARRAYREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Array),
    };

    /// Alias for the `exnref` type in WebAssembly.
    pub const EXNREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::Exn),
    };

    pub const NULLREF: RefType = RefType {
        nullable: Nullability::Nullable,
        heap_type: HeapType::Abstract(AbsHeapType::None),
    };

    /// Create a new abstract reference type.
    pub const fn new_abstract(ty: AbsHeapType, nullable: Nullability) -> Self {
        Self {
            nullable,
            heap_type: HeapType::Abstract(ty),
        }
    }

    pub fn new_identifier(ident: String) -> Self {
        Self {
            nullable: Nullability::NonNullable,
            heap_type: HeapType::Identifier(ident)
        }
    }

    pub const fn into_val_type(self) -> ValType {
        ValType::Ref(self)
    }

    pub const fn into_result_type(self) -> ResultType {
        ResultType(self.into_val_type())
    }

    pub const fn into_global_type(self, mutable: Mutability) -> GlobalType {
        GlobalType {
            mutable,
            val_type: self.into_val_type(),
        }
    }

    pub const fn into_block_type_result(self) -> BlockType {
        BlockType::Result(self.into_result_type())
    }

    pub const fn into_storage_type(self) -> StorageType {
        StorageType::Val(self.into_val_type())
    }

    pub const fn into_field_type(self) -> FieldType {
        FieldType {
            mutability: Mutability::Const,
            ty: self.into_storage_type(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HeapType {
    Abstract(AbsHeapType),
    Identifier(String),
}

impl HeapType {
    pub fn into_ref_type(self) -> RefType {
        RefType {
            nullable: Nullability::NonNullable,
            heap_type: self,
        }
    }

    pub fn into_val_type(self) -> ValType {
        self.into_ref_type().into_val_type()
    }
}

/// An abstract heap type.
/// attribution: Copied from `wasm-encoder` crate `wasm-encoder/src/core/types.rs`
/// https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder
/// MIT licensed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AbsHeapType {
    /// The abstract `any` heap type.
    ///
    /// The common supertype (a.k.a. top) of all internal types.
    Any,

    /// The abstract `eq` heap type.
    ///
    /// The common supertype of all referenceable types on which comparison
    /// (ref.eq) is allowed.
    Eq,

    /// The unboxed `i31` heap type.
    I31,

    /// The abstract `struct` heap type.
    ///
    /// The common supertype of all struct types.
    Struct,

    /// The abstract `array` heap type.
    ///
    /// The common supertype of all array types.
    Array,

    /// The abstract `none` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all internal types.
    None,

    /// Untyped (any) function.
    Func,

    /// The abstract `nofunc` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all function types.
    NoFunc,

    /// The abstract `exception` heap type.
    Exn,

    /// The abstract `no-excepction` heap type.
    NoExn,

    /// The abstract external heap type.
    Extern,

    /// The abstract `noextern` heap type.
    ///
    /// The common subtype (a.k.a. bottom) of all external types.
    NoExtern,
}

/// The type of an instruction sequence
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockType {
    Result(ResultType),
    TypeUse(String),
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
    pub is_final: Finality,
    pub supertypes: Vec<String>,
    pub comp_type: CompType,
}

impl SubType {
    pub fn final_no_super(comp_type: CompType) -> Self {
        Self {
            is_final: Finality::Final,
            supertypes: vec![],
            comp_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompType {
    Struct(StructType),
    Array(ArrayType),
    Func(FuncType),
}

impl CompType {
    /// Wasm's "default" abbreviated subtype is:
    /// `(type <COMP_TYPE>) == (type (sub final ∅ <COMP_TYPE>))`
    pub fn into_sub_type(self) -> SubType {
        SubType::final_no_super(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructType {
    pub fields: FieldsType,
}

impl StructType {
    pub fn into_comp_type(self) -> CompType {
        CompType::Struct(self)
    }

    pub fn into_sub_type(self) -> SubType {
        self.into_comp_type().into_sub_type()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub field: FieldType,
}

impl ArrayType {
    pub fn into_comp_type(self) -> CompType {
        CompType::Array(self)
    }

    pub fn into_sub_type(self) -> SubType {
        self.into_comp_type().into_sub_type()
    }
}

/// Type of a function
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub params: ParamsType,
    pub results: ResultsType,
}

impl FuncType {
    pub fn into_comp_type(self) -> CompType {
        CompType::Func(self)
    }

    pub fn into_sub_type(self) -> SubType {
        self.into_comp_type().into_sub_type()
    }
}

pub type FieldsType = Box<[(String, FieldType)]>;
pub type ParamsType = Box<[ParamType]>;
pub type ResultsType = Box<[ResultType]>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldType {
    pub mutability: Mutability,
    pub ty: StorageType,
}

impl FieldType {
    /// `(ref $<IDENTIFIER>)`
    pub fn ref_identifier(identifier: String) -> FieldType {
        Self {
            mutability: Mutability::Const,
            ty: StorageType::Val(ValType::Ref(RefType {
                nullable: Nullability::NonNullable,
                heap_type: HeapType::Identifier(identifier),
            })),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamType {
    pub name: String,
    pub ty: ValType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StorageType {
    Val(ValType),
    Pack(PackType),
}

impl StorageType {
    pub const fn into_field_type(self) -> FieldType {
        FieldType {
            mutability: Mutability::Const,
            ty: self,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PackType {
    I8,
    I16,
}

impl PackType {
    pub const fn into_storage_type(self) -> StorageType {
        StorageType::Pack(self)
    }

    pub const fn into_field_type(self) -> FieldType {
        self.into_storage_type().into_field_type()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResultType(pub ValType);
