Things I really want to do but I'm not going to waste time on.

- Start using Emacs again, re-write every text editing facility I use in elisp
	- (Ok I did a little of this on my own time. But it's UNRELATED to the project!)
	- Re-write `notes/` and also my personal notes folder as an org-mode wiki...
- Re-write my compiler in Racket (The Hoot source code is so pretty! The macros are so easy!)
	- Writing `.wat` is just pretty printing!
		- This would be true if Wasm had decided to use Rust `#[derive(Debug)]` as its .wat format, too...
			- But they wouldn't.

# .wat syntax highlighting
- There exist treesitter grammars, editor plugins for .wat format, but they're all outdated
	- Don't support the current Wasm 3.0 constructs
- The wasm project writes their standard semi-programmatically using SpecTec:
	- https://dl.acm.org/doi/10.1145/3656440
- It should be possible to:
	- Take the grammar AST SpecTec produces
	- Trim it significantly
	- Transform it into a Treesitter grammar `.js` file
- In theory, that would keep it up-to-date with future revisions, assuming no breaking SpecTec changes.
- I looked into this, it was more of a pain than I hoped.
# Optimization


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

