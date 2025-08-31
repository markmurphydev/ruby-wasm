Using wasm 3.0 draft
https://webassembly.github.io/spec/

From the 2.0 announcement:
https://webassembly.org/news/2025-03-20-wasm-2.0/#:~:text=Published%20on%20March%2020%2C%202025,rather%20old%20news%20to%20you.
> With the advent of 2.0, the Working Group is switching to a so-called “evergreen” model for future releases. That means that the Candidate Recommendation will be updated in place when we create new versions of the language, without ever technically moving it to the final Recommendation state. For all intents and purposes, the latest [Candidate Recommendation Draft](https://www.w3.org/TR/wasm-core-2/) is considered to be the current standard, representing the consensus of the Community Group and Working Group. (If this sounds odd, that’s mostly because the W3C’s document terminology does not quite match this more flexible process, which has recently been adopted by several working groups.)


WebAssembly: How Low Can a Bytecode Go?
https://queue.acm.org/detail.cfm?id=3746172

RustでWasm Runtimeを実装する
https://zenn.dev/skanehira/books/writing-wasm-runtime-in-rust/viewer/02_about_wasm

# WasmGC
https://developer.chrome.com/blog/wasmgc
https://v8.dev/blog/wasm-gc-porting
https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md
https://tanishiking.github.io/posts/wasm-gc/
Binaryen's GC lowering tips
https://github.com/WebAssembly/binaryen/wiki/GC-Implementation---Lowering-Tips


# Binutils
Hoot has some. Allegedly.

https://github.com/WebAssembly/wabt
- No WasmGC support
https://github.com/WebAssembly/wabt/issues/2348

https://github.com/bytecodealliance/wasm-tools

- `binaryen`'s `wasm-as` – it may modify instruction sequences to fit its IR
	- But `binaryen` is a Wasm optimizer so it should give accurate output...
# Misc
Oh, there's an ACM Queue edition on wasm!

WebAssembly: Yes, but for What?
https://queue.acm.org/detail.cfm?id=3746171