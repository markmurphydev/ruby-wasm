# Lexer
- We track char indexes, not byte indexes.
	- The reason: we have to track the length manually in several sub-functions, because if we hit EOF, we don't get byte information. Chars were easier to count manually.
		- Solution: We have a method on our iterator already that gets the highest index. So, use that.
	- If we use those indexes to get the text of a lexeme, it's `O(n)` on text size.
	  Which is fine if it's just for display.
- We use char indexes to get text of lexeme, to parse numbers etc.
Bad stuff! But we need to ship!
The lexer is a superfund site right now. If we change to byte idx's, I gotta do codegen or a macro for the nested-match section for operators and keywords first.

- We clone text into some lexemes
	- Like `IntegerLiteral`
	- This is for convenience, and not-duplicating-reads of the text
		- You could have a separate memoization table for this
- We also make our lexer peekable, so we clone the lexeme each time we return it!