
# Type
Ruby is a dynamically typed language. 
Every value the program deals with is, in theory, a member of the tagged-union type `Unitype`.

In Wasm terms, every Ruby value is a subtype of `(ref eq)` – reference (GC'd) types with reference equality.

Wasm references are either:
- References to heap values
- Pointer-tagged `i31` values
	We stuff small integers, booleans, etc. in here

# [ ] Unitype struct
This isn't specified yet. Right now I think I store boxed integers as just `(ref (struct i32))`, which is wrong.

# Fixnums
Fixnums are an implementation detail of Ruby `Integer`'s.
Small integers are stored as fixnums in `i31` values.

We dedicate the high half of `i31` to fixnums, so any signed integer that fits in 30 bits is represented with `1` in the MSB, and the value in the remaining 30 bits.
```rust
// (0b1xx_xxxx...): i31  
const FIXNUM_BIT_WIDTH: u32 = 30;
```

# Booleans, etc.
Right now, this is just `true, false, nil`.
Constants are stored in the low half of `i31`.

I think we'll have space to spare, so I use a sparse representation that makes it a little easier to do boolean operations.
Inspired by Hoot's representation. It might be identical currently.
```rust
// In Wasm, you produce a `(i32.const 0bxxxx)`,
//  then use the `ref.i31` instruction to convert.
pub const FALSE: I32 = I32(0b0001);  
//                             │└───is_bool
//                             └────is_true
pub const TRUE: I32  = I32(0b0011);  
pub const NIL: I32   = I32(0b0100);
//                            └─────is_nil
```
