Points: 4

The final lexer probably requires parser-only information, and must thus be driven by the parser.
Even so, I should add the option to get a flat token stream from the compiler binary.
I should test that the output of my lexer matches the output of Prism.
- It's the default CRuby parser now, so.

I'll need to make the compiler binary and add a flag for generating lexer output.

Prism can output to a token stream.
I can copy and output that format and just textually compare the two.
That format includes token locations and text. Now would be a good time to add that functionality to the lexer.
It also includes "lexer state" when the token is parsed (I'm pretty sure that's what it is). I'll have to strip that.

I do _not_ want to add any non-Rust testing dependencies right now.
I'll just write the input files, manually lex them with Prism, and write the expected output files.