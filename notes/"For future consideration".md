Things I really want to do but I'm not going to waste time on.

- Start using Emacs again, re-write every text editing facility I use in elisp
	- Ok I did a little of this on my own time. But it's UNRELATED to the project!
- Re-write my compiler in Racket (The Hoot source code is so pretty! The macros are so easy!)
	- Writing `.wat` is just pretty printing!
		- This would be true if Wasm had decided to use Rust `#[derive(Debug)]` as its .wat format, too...
			- But they wouldn't.

# Rust feature requests
- Nested use aliases
```
use crate::wasm as W;
use W::types as W::T;
```

- Deref individual tuple/struct args in match pattern
```rust
enum Foo {
	Bar,
	Baz(u32, String),
}

match &Foo::Baz(1, "2".to_string()) {
	Bar => { ... }
	Baz(&a, b) => { 
		// a: u32
		// b: &String 
		... 
	}
}
```
- Right now you could deref the Baz match, but it would be a move cause `String` isn't copy.
