The game here is to deal with the "operator problem" in recursive descent.

```
Expr =
  Expr '+' Expr
| Expr '*' Expr
| '(' Expr ')'
| number
```

Two problems:
- Infinite left recursion
- What's the operator precedence?

# Solution

Define a precedence table for operators
Keep a `precedence_limit` in the parser state
- [x] Does this need to be a stack of precedence limits?
	Yeah, or make it an arg to every parse function
		(The poor man's stack)

When we get to `parse_expr()`:
- Parse a "prefix" with no operators
- Crawl forward in a loop, looking for operators
	- When found, compare its precedence to the current `precedence_limit`
		- If it binds tighter, parse the RHS
		- If not, return the previously-parsed expression as the RHS of our recursion-parent

There's some extensions for dealing with prefixes, postfixes.
Just consult the sources.
# Sources
Pratt Parsing Index
https://www.oilshell.org/blog/2017/03/31.html

Demystifying Pratt parsers
https://martin.janiczek.cz/2023/07/03/demystifying-pratt-parsers.html

Simple but Powerful Pratt Parsing
https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html