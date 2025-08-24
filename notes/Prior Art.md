# Ruby -> Wasm
Survey of Ruby in Wasm (circa March 2024):
https://developer.fermyon.com/wasm-languages/ruby
## Compiling the Ruby interpreter to Wasm
CRuby:
https://github.com/ruby/ruby.wasm/
Shopify's "wizer-optimized" (?) version of Ruby:
https://github.com/Shopify/ruvy
Rust implementation of the _interpreter_ that compiles to Wasm:
https://www.artichokeruby.org
- I don't know exactly how this one works. 
  It seems not to ship the entire interpreter, but it's definitely shipping at least the GC.

# WasmGC compilers

Scheme on Wasm:
https://spritely.institute/hoot/
Explanation:
https://spritely.institute/news/scheme-wireworld-in-browser.html

# 