https://docs.rs/id-arena/latest/id_arena/
Rust discord:
> I'm looking at the `id_arena` crate ([https://crates.io/crates/id-arena](https://crates.io/crates/id-arena "https://crates.io/crates/id-arena")).
> I don't quite understand how `Arena<T>` is different from just wrapping a `Vec<T>` and not exposing `remove()`?

> it is fact is just a wrapper around a vec

> Looks like this crate also ensures that IDs from two different arenas are value-wise distinct.
> but tbh it seems rather unnecessary and overengineered

> it seems like it adds typed IDs and does not have any special allocation/borrowing powers

> But most people use slotmap instead

https://docs.rs/slotmap/latest/slotmap/