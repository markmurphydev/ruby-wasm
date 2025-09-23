//! attribution: Substantially copied from Walrus crate `walrus/src/arena_set.rs`
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed

use id_arena::{Arena, Id};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops;

/// A set of unique `T`s that are backed by an arena.
#[derive(Debug)]
pub struct ArenaSet<T: Clone + Eq + Hash> {
    arena: Arena<T>,
    already_in_arena: HashMap<T, Id<T>>,
}

impl<T: Clone + Eq + Hash> ArenaSet<T> {
    /// Construct a new set.
    pub fn new() -> ArenaSet<T> {
        ArenaSet {
            arena: Arena::default(),
            already_in_arena: HashMap::new(),
        }
    }

    /// Insert a value into the arena and get its id.
    pub fn insert(&mut self, val: T) -> Id<T> {
        if let Some(id) = self.already_in_arena.get(&val) {
            return *id;
        }

        let id = self.arena.alloc(val.clone());
        self.already_in_arena.insert(val, id);
        id
    }

    /// Get the id that will be used for the next unique item added to this set.
    pub fn next_id(&self) -> Id<T> {
        self.arena.next_id()
    }

    /// Remove an item from this set
    pub fn remove(&mut self, id: Id<T>)
    {
        self.already_in_arena.remove(&self.arena[id]);
    }

    /// Iterate over the items in this arena and their ids.
    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.arena.iter()
    }
}

impl<T: Clone + Eq + Hash> ops::Index<Id<T>> for ArenaSet<T> {
    type Output = T;

    #[inline]
    fn index(&self, id: Id<T>) -> &T {
        &self.arena[id]
    }
}

impl<T: Clone + Eq + Hash> ops::IndexMut<Id<T>> for ArenaSet<T> {
    #[inline]
    fn index_mut(&mut self, id: Id<T>) -> &mut T {
        &mut self.arena[id]
    }
}

impl<T: Clone + Eq + Hash> Default for ArenaSet<T> {
    fn default() -> ArenaSet<T> {
        ArenaSet::new()
    }
}
