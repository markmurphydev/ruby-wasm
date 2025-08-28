
# Ripper

User-visible parsing output:
https://web.archive.org/web/20180721084455/http://www.rubyinside.com/using-ripper-to-see-how-ruby-is-parsing-your-code-5270.html

https://docs.ruby-lang.org/en/3.4/Ripper.html

```ruby
require 'ripper'
require 'pp'

pp Ripper.lex('puts "Hi"')
pp Ripper.sexp('puts "Hi"')
```

# Tree Sitter
https://news.ycombinator.com/item?id=26226415

Has a big-ass external scanner:
https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners.html
`ruby_wasm/sources/treesitter/scanner.cc`

Needed for:
- "delimited literals"
	- Idk.
- whitespace-sensitive tokens

https://stackoverflow.com/questions/18703999/lexing-parsing-here-documents