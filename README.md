```
cargo run wat "nil" > nil.wat && wasmtime -W gc=y nil.wat --invoke "_start"
```