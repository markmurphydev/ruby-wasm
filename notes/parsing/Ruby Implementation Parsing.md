Jesus christ I didn't realize that Ruby's parsing was such a disaster area.

So, CRuby 3.4 (current version) uses Prism as the default parser, with a fallback option to the old `parse.y` Yacc/Bison/LRama file (preprocessed through Ripper).
# Prism Parser
https://www.reddit.com/r/ruby/comments/18apelr/advent_of_prism_rubys_new_parser_part_1_literals/

Includes a great history of the Ruby parsers:
https://railsatscale.com/2024-04-16-prism-in-2024/
> "While [JRuby](https://github.com/jruby/jruby)’s parser has been by far the most comprehensive alternative Ruby parser over the years, getting to 100% parity with all of the various eccentricities is extremely difficult."

https://railsatscale.com/2023-06-12-rewriting-the-ruby-parser/

# Ripper
From https://railsatscale.com/2024-04-16-prism-in-2024/:
> This was an event-driven parser that allowed users to build their own syntax trees. It worked by copying the Ruby grammar file and modifying the actions to dispatch events that called out to user-defined methods.

> In order to marry these two requirements, [Ripper](https://github.com/ruby/ruby/blob/master/ext/ripper/README) was fashioned as a pre-processing step on the existing grammar file. Within the actions of the grammar file a special domain-specific language was used in C language comments to describe the actions that [Ripper](https://github.com/ruby/ruby/blob/master/ext/ripper/README) would take. A tool was created that would extract these comments and generate a clean grammar file that itself could then be passed into Bison. If this sounds complicated, that’s because it is. [Ripper](https://github.com/ruby/ruby/blob/master/ext/ripper/README)’s setup means that any changes to the grammar file might inadvertently change the semantics of [Ripper](https://github.com/ruby/ruby/blob/master/ext/ripper/README), a caveat that exists to this day.

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
Shaves off many of the more complex bits. Probably good for a start.

https://news.ycombinator.com/item?id=26226415

Has a big-ass external scanner:
https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners.html
`ruby_wasm/sources/treesitter/scanner.cc`

Needed for:
- "delimited literals"
	- Idk.
- whitespace-sensitive tokens

https://stackoverflow.com/questions/18703999/lexing-parsing-here-documents

# Exploring Ruby parsers
https://kddnewton.com/2023/11/30/advent-of-prism-part-0#exploring