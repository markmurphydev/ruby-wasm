One motivation for this project is that existing Ruby -> Wasm solutions require shipping a large binary on (each? Does it get cached by the browser?) page load.

The main `ruby.wasm` project (compiling CRuby interpreter to wasm) has a [cheat sheet](https://github.com/ruby/ruby.wasm/blob/main/docs/cheat_sheet.md),
which includes instructions for running ruby wasm from the browser.
The `ruby+stdlib.wasm` file from that is about 30MB.

There also exists Artichoke, which produces wasm binaries for Ruby.
I don't understand what it's doing, but I'm pretty certain it's shipping a large chunk of the Ruby VM.
It's not using the WasmGC proposal, so it's definitely shipping a garbage collector, if nothing else.
[This post](https://rubytalk.org/t/anyone-writing-building-a-ruby-2-wasm-compiler/75450/7) from 2021 says it's "2.68MB uncompressed, 789kiB compressed, plus a similarly sized boilerplate code (Emscripten)"
Going to [Artichoke's playground](https://artichoke.run/) on 2025-08-24 downloaded a 7.3MB playground.wasm file.
