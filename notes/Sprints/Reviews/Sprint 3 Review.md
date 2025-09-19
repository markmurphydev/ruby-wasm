# Goal
Complete integer codegen, control structure (if/else/elsif, while/until loops) codegen.

# PBIs
 [[Lexer end-to-end test infrastructure]]
	4pt

[[Document how we'll use Ruby spec]]
	4pt

[[Document our value representation]]
	4pt

[[Codegen for integers]]
	6pt

[[Codegen for if_else, loops]]
	8pt

[[Research parsing algorithms – recursive descent & Pratt parsing (again...)]]
	6pt

# Incomplete PBIs
[[Lexer end-to-end test infrastructure]]
- I'm very confused about how/if you're supposed to run output binaries in `cargo-test`
	- I can avoid this by just running the internal library functions.
	 No longer "end-to-end", but whatever.
- My original idea was to test against the Prism parser's output, but I no longer think that's worth doing.
	Better just to do snapshot testing
	- That means I need concise, readable lexer output
		- [ ] Need to write a display function for `lexemes` (using default `Debug` right now)
		- [ ] Need `(line, col)` output

# Reflections
Sorry, I will get these in on Thursday in the future.

I'm officially falling behind the proposal's schedule.
	I'm deploying countermeasures.

I haven't actually been spending enough time on the "Sprint X" planning
	The sub-tasks are good guides. I should spend more time on them.
	My point estimates are badddd. I should do another pass on them.

These self-inflicted "documentation" tasks are killing me.
	I'm taking notes while reading anyway, and documenting my code.
	Writing it out in `notes/` is just doing everything in triplicate

I spent a lot of time on the Wasm "back-end"
	At one point I wanted to control the whole `ruby source -> .wasm binary` chain.
	I no longer care about that at all.
- [ ] Am I good to produce the wasm-level AST, then pass it off to Wasm toolchain librar(y・ies)?
	- Wouldn't change the level of lowering I'm doing, just the amount of serializing to `.wat` files...
- [ ] Am I good to use wasm-level AST definitions from a library?
	- It's all very mechanical conversion from the spec.
- [ ] Am I good to pass the output into `binaryen` Wasm -> Wasm optimizer?

Between that and copying AST repr from Prism, I'm really reducing the amount of code that's "mine", but:
- I'm probably incapable of inventing a correct Ruby AST from first principles
- The core of the project is basically "How to translate Ruby AST -> Wasm AST?", and all that work is still mine.

## Testing
I'm crumbling under the weight of writing tests, and also crumbling under the weight of not having tests.
I've already had to spend an hour re-writing unit tests when I refactored the lexer.

### Snapshot testing
I'd like to rely a lot more on snapshot testing:
- Write input file
- Run it through stage X of the compiler
- Test that it matches the expected output file
	- If not, I check the difference manually

Rust-analyzer uses this a lot, and I found it nice to work with.
The danger is that I fail to inspect correctly. But I could also fail to write a unit test correctly.
Requires me to have concise output for each step, which I don't always right now.

### Copying from Ruby spec
The "ruby spec" (scare quotes) is a bunch of executable tests.
The idea is to have the Ruby implementation run its test runner, but that's a long way out.

BUT I want to copy test input, and run "manual" comparisons between Wasm's output and a Ruby interpreter's output.
- This is blocked on getting a readable output from a Wasm runtime
	But that's going to be one of my first tasks next week

# Questions
- Do you have an opinion about which of the classical "test types" (unit, integration, e2e, ...) are most important?

- Intermediate reprs
	- I think they're all trees for me
	- But how many?
- Runtime support functions
	- Manipulating Ruby types, ...
	- Write in Wasm directly? Try to compile Ruby or Rust to Wasm?
		What do normal compilers do?