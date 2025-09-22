# Separate test file
```rust
...
mod syntax_node;
#[cfg(test)]
mod tests; // In a `tests.rs` file
mod token_text;
...
```
source: `rust_analyzer/crates/syntax/src/lib.rs`

