# Goal
Prepare for the project, gathering prerequisites.
Complete a large chunk of the lexer, and a walking skeleton.

# PBIs
[[Get a Ruby parser to produce reference ASTs]]
	1pt
[[Install wasm utilities]]
	1pt
[[Document Ruby parsing situation]]
	3pt
[[Install, run, document Hoot Scheme compiler]]
	4pt
[[Write the easy part of lexer]]
	8pt
[[Lexer end-to-end test infrastructure]]
	4pt
[[Walking skeleton ー end-to-end .wasm generation]]
	4pt
[[Browser .wasm runner page]]
	2pt
[[Research value representation in dynamic languages]]
	2pt

# Incomplete PBIs
[[Install, run, document Hoot Scheme compiler]]
- This was much harder to install and run than I expected.
- The normal installation instructions assume you're using Guix.
	- Not easily usable on Macos
- Next week I want to try installing it on a Linux VPS with Guix.

[[Lexer end-to-end test infrastructure]]
- The workflow is mostly complete, didn't get to formatting and documentation
- I plan to write the actual tests on a feature-to-feature basis.

[[Walking skeleton ー end-to-end .wasm generation]]
[[Browser .wasm runner page]]
- I didn't get to these. I still don't expect them to be very hard.

[[Research value representation in dynamic languages]]
- I didn't get to these, but I did write up some strategies from the talk [[Is wasm becoming garbage?]]
# Velocity
13 / 29
Not too good! I took a lot this sprint, and some PBIs are half-completed, but still.

# Demo
In the project directory:
```
$ cargo run lex "22"
...
Lexeme { kind: IntegerLiteral, start_line: 1, start_col: 0, end_line: 1, end_col: 2 }
Lexeme { kind: Eof, start_line: 1, start_col: 2, end_line: 1, end_col: 2 }
```

# Reflections
I got distracted by interesting articles and talks on PL, Wasm, and Ruby internals.
These will be useful later, but it's very important to produce usable output.
I should focus less on writing up the research PBIs, and more on producing artifacts.
I absolutely need to produce a walking skeleton in sprint 2.
## Testing
I wanted to get a lot written for the lexer this week, to make sure I understood
the structure of it, and that it would work.

I don't want to test it all at once. First because it will be longgg and demotivating to do it, and also because I might not use all the lexemes I'm attempting to lex right now.
As I add features, I want to add tests for them at each stage from lexer to code generation.
So, I'm removing the "unit testing" task from the lexer. I still need to finish [[Lexer end-to-end test infrastructure]], since that's test infrastructure I'll need to use later.

# Sprint 2 goal
Finish the walking skeleton.
Finish sprint 1 PBIs.
Build out the parser.
Produce a complete implementation, including Wasm output for:
- Integers
- If/else
- Loops
