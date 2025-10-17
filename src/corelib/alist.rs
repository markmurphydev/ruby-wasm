use wat_defs::module::TypeDef;
use wat_defs::ty::{RefType, StorageType};
use wat_macro::wat;

// use crate::corelib::type_def::METHOD_TYPE_IDENTIFIER;
// use crate::unitype::Unitype;
// use crate::wasm::{Finality, TypeDef};
// use crate::wasm::types::{
//     AbsHeapType, ArrayType, CompType, FieldType, HeapType, Mutability, Nullability, RefType,
//     StorageType, StructType, SubType, ValType,
// };
// use crate::{CompileCtx, run_ruby_program};
//
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
            AListValTypeIdentifier::Unitype => wat![ (ref eq) ],
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
    ///     "alist-str-unitype".to_string()
    /// );
    /// ```
    pub fn alist_type_identifier(&self) -> String {
        format!(
            "alist-{}-{}",
            self.key_type_identifier,
            Self::val_name(&self.val_type)
        )
    }

    pub fn alist_pair_type_identifier(&self) -> String {
        format!(
            "alist-{}-{}-pair",
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

        let alist_pair_type_def = TypeDef::new(
            ctx,
            &self.alist_pair_type_identifier(),
            SubType {
                is_final: Finality::NotFinal,
                supertypes: vec![],
                comp_type: CompType::Struct(StructType {
                    fields: Box::new([
                        (
                            "key".to_string(),
                            FieldType::ref_identifier(self.key_type_identifier),
                        ),
                        (
                            "val".to_string(),
                            FieldType {
                                mutability: Mutability::Const,
                                ty: StorageType::Val(ValType::Ref(self.val_type)),
                            },
                        ),
                    ]),
                }),
            },
        );

        [alist_type_def, alist_pair_type_def]
    }

    /// The "name" of `val-type-identifier, to be interpolated into the AList type identifiers.
    /// It is not meaningful independent of the AList type identifier.
    fn val_name(val_type: &RefType) -> String {
        if let Nullability::Nullable = val_type.nullable {
            todo!("No val name for {:?}", val_type)
        }
        match &val_type.heap_type {
            HeapType::Abstract(abs) => match abs {
                AbsHeapType::Eq => "unitype",
                abs => todo!("No val name for {:?}", abs),
            },
            HeapType::Identifier(ident) => ident,
        }
        .to_string()
    }
}

/// Alist of pairs `(String, Unitype)`
pub fn alist_str_unitype() -> AListTypeDef {
    AListTypeDef {
        key_type_identifier: Unitype::STRING_TYPE_IDENTIFIER.to_string(),
        val_type: Unitype::UNITYPE,
    }
}

pub fn alist_str_method() -> AListTypeDef {
    AListTypeDef {
        key_type_identifier: Unitype::STRING_TYPE_IDENTIFIER.to_string(),
        val_type: RefType::new_identifier(METHOD_TYPE_IDENTIFIER.to_string()),
    }
}

pub fn alist_type_defs(ctx: &mut CompileCtx<'_>) -> Vec<TypeDef> {
    [alist_str_unitype(), alist_str_method()]
        .into_iter()
        .flat_map(|alist| alist.into_type_defs(ctx))
        .collect()
}
