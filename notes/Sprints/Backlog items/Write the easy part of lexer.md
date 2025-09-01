Points: 8

I'm fairly certain a complete Ruby lexer needs to be incremental, and driven by the parser with a `lexer_mode` argument.
For now, let's ignore that. We can do incremental `lexeme()` method, but shouldn't need multiple modes.
I think this will be fine to lex the first couple of week's features.
- If that turns out not to be the case, I'll move [[Document the hard parts of the lexer]] forward.
I'm going to directly copy the list of tokens from the Prism parser unless told not to.

Sources:
Engineering a Compiler's lexer section just talks about DFAs...
Crafting Interpreters had a good bit on it.
Prism a 23k-line c file, but there's a lexer in there somewhere. It has some useful internal docs.