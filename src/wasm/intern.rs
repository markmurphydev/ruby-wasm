//! Interner for WAT identifiers.
//! attribution: Adapted from the "simplest possible interner" example here:
//! https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html

use std::collections::HashMap;
use id_arena::{Arena, Id};

/// A WAT identifier.
///
/// Identifiers of the form `$symbol_name` are
/// used as an alternative to indexed identifiers in WAT format.
///
/// https://webassembly.github.io/spec/core/text/modules.html#indices
///
/// Each _index space_ has a corresponding _identifier space_.
pub struct Identifier(String);

/// A handle to an interned identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InternedIdentifier(u32);

#[derive(Debug, Default)]
pub struct IdentifierInterner {
    // This is a specialization of a bijective map
    // e.g. https://docs.rs/bimap/latest/bimap/
    /// { idx -> identifier }
    vec: Vec<String>,
    /// { identifier -> idx }
    map: HashMap<String, u32>,
}

impl IdentifierInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, name: &str) -> InternedIdentifier {
        if let Some(&idx) = self.map.get(name) {
            return InternedIdentifier(idx);
        };
        let idx = self.vec.len() as u32;
        self.vec.push(name.to_owned());
        self.map.insert(name.to_owned(), idx);

        debug_assert_eq!(self.get(InternedIdentifier(idx)), name);
        debug_assert_eq!(self.intern(name).0, idx);

        InternedIdentifier(idx)
    }

    pub fn get(&self, interned_identifier: InternedIdentifier) -> &str {
        self.vec[interned_identifier.0 as usize].as_str()
    }
}


