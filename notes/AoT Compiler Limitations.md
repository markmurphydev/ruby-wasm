Problem: 
- The point of the AoT compiler is to avoid shipping the ruby interpreter on every call.
- But Ruby has extensive metaprogramming facilities
	- In the worst case, `eval` requires shipping a ruby compiler/interpreter, right?

# Prior Art
There are other projects with the goal of AoT compilation for scripting languages.

Python AoT
https://github.com/exaloop/codon
"There are some aspects of Python that are not suitable for static compilation"

Lisp
Many lisp implementations, including SBCL, have bytecode or native compilation.
I haven't investigated how they do it, but I think that will be a rich vein.

Javascript -> Wasm (for what? I kid.)
https://github.com/AssemblyScript/assemblyscript
## Ruby
There's some abortive / marginal Ruby AoT compilers.

Sorbet:
https://sorbet.org/blog/2021/07/30/open-sourcing-sorbet-compiler
- Stripe I think? Or Shopify? One of them.
- Compiler's cancelled, type checker lives on.
- Compiler source can be pulled from the git history

Dragon Ruby (game engine)'s compiler (Lightstorm):
https://github.com/DragonRuby/lightstorm
https://lowlevelbits.org/compiling-ruby-part-0/
https://blog.llvm.org/posts/2024-12-03-minimalistic-ruby-compiler/
