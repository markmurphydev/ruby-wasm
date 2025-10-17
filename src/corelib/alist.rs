use crate::unitype::Unitype;
use wat_defs::module::TypeDef;
use wat_defs::ty::{AbsHeapType, HeapType, Nullable, RefType, StorageType, ValType};
use wat_macro::wat;

/// Most AList key, val types are written `(ref $<IDENTIFIER>)`.
/// However, some types can't be given an identifier in Wasm.
/// The main one of concern is [Unitype], which is represented as inline `(ref eq)` in .wat.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AListValTypeIdentifier {
    Unitype,
    Identifier(String),
}

impl AListValTypeIdentifier {
    pub fn ref_type(&self) -> RefType {
        match self {
            AListValTypeIdentifier::Unitype => wat![(ref eq)],
            AListValTypeIdentifier::Identifier(ident) => wat![ (ref ,(ident.to_string())) ],
        }
    }
}

/// An assoc-list of type `[ {key: <KEY>, val: <VAL>} ]`
/// Used as a temporary alternative to hash tables.
#[derive(Debug)]
pub struct AListTypeDef {
    /// Identifier of the `key` field of this AList's pairs
    pub key_type_identifier: String,
    /// Type of the `val` field of this AList's pairs
    pub val_type: StorageType,
}

impl AListTypeDef {
    /// The type identifier by which to refer to this AList in .wat code.
    ///
    /// ```
    /// assert_eq!(
    ///     alist_str_unitype().alist_type_identifier,
    ///     "alist_str_unitype".to_string()
    /// );
    /// ```
    pub fn alist_type_identifier(&self) -> String {
        format!(
            "alist_{}_{}",
            self.key_type_identifier,
            Self::val_name(&self.val_type)
        )
    }

    pub fn alist_pair_type_identifier(&self) -> String {
        format!(
            "alist_{}_{}_pair",
            self.key_type_identifier,
            Self::val_name(&self.val_type)
        )
    }

    /// An AList type definition consists of a pair:
    ///
    /// ```
    /// (type <ALIST_TYPE_NAME> (array (ref <ALIST_PAIR_TYPE_NAME>)))
    /// (type <ALIST_PAIR_TYPE_NAME>
    ///     (struct (field $key <KEY_TYPE_NAME>)
    ///             (field $val <VAL_TYPE_EXPR>)))
    /// ```
    ///
    /// where:
    ///
    /// ```
    /// VAL_TYPE_EXPR ::= (ref $<VAL_TYPE_NAME>)
    ///                 | (ref eq)
    /// ```
    ///
    /// Returns `[alist_def, alist_pair_def]`
    pub fn into_type_defs(self) -> [TypeDef; 2] {
        let alist_type_def = wat! {
            (type ,(self.alist_type_identifier())
                   (array (ref ,(self.alist_pair_type_identifier()))))
        };

        let alist_pair_type_def = wat! {
            (type ,(self.alist_pair_type_identifier())
                   (struct (field $key (ref ,(self.key_type_identifier)))
                           (field $val ,(self.val_type))))
        };

        [alist_type_def, alist_pair_type_def]
    }

    /// The "name" of `val-type-identifier, to be interpolated into the AList type identifiers.
    /// It is not meaningful independent of the AList type identifier.
    fn val_name(val_type: &StorageType) -> String {
        match val_type {
            StorageType::Val(ValType::Ref(RefType {
                null: Nullable::NonNullable,
                heap_type: HeapType::TypeIdx(name)
            })) => name,
            StorageType::Val(ValType::Ref(RefType {
                null: Nullable::NonNullable,
                heap_type: HeapType::Abs(AbsHeapType::Eq),
            })) => "unitype",
            _ => todo!("No val name for {:?}", val_type),
        }
        .to_string()
    }
}

/// Alist of pairs `(String, Unitype)`
pub fn alist_str_unitype() -> AListTypeDef {
    AListTypeDef {
        key_type_identifier: "str".to_string(),
        val_type: Unitype::unitype().into_storage_type(),
    }
}

pub fn alist_str_method() -> AListTypeDef {
    AListTypeDef {
        key_type_identifier: "str".to_string(),
        val_type: wat![ (ref $method) ].into_storage_type(),
    }
}

pub fn alist_type_defs() -> Vec<TypeDef> {
    [alist_str_unitype(), alist_str_method()]
        .into_iter()
        .flat_map(AListTypeDef::into_type_defs)
        .collect()
}
