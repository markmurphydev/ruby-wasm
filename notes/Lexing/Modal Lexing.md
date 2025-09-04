You write a lexer, but with a `mode` enum parameter. Simple enough.
- Good for handling string interpolation, because you can have nested string tokens:
	- Alternatively, just put a flag in lexer's state?
```ruby
irb(main):008> print "#{ "#{ "asdf" }" }"
asdf=> nil
```

Prism does this with internal lexer state

https://denisdefreyne.com/articles/2022-modal-lexer/
https://www.oilshell.org/blog/2017/12/17.html