So, traditionally, I think the difference is that a lexer recognizes regular grammars, but a parser recognizes context-free grammars, right?
See here:
https://stackoverflow.com/questions/2842809/lexers-vs-parsers
but also like every textbook.

But (from https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners.html):
> Many languages have some tokens whose structure is impossible or inconvenient to describe with a regular expression. Some examples:
> - [Indent and dedent](https://en.wikipedia.org/wiki/Off-side_rule) tokens in Python
> - [Heredocs](https://en.wikipedia.org/wiki/Here_document) in Bash and Ruby
> - [Percent strings](https://docs.ruby-lang.org/en/2.5.0/doc/syntax/literals_rdoc.html#label-Percent+Strings) in Ruby

For the ruby examples at least, it's not feasible to tokenize while ignoring them.
With heredocs, for example, the contents of the heredoc are syntactically-meaningless strings.
You need to at least be able to know when you're in the range of _some_ heredoc.
But that's impossible (I think) within a regular language.
So, for Ruby, does that mean that we just can't have a lexer (normally defined) separate from the parser?

