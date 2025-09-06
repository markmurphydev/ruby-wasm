In `irb`, require the `"Prism"` module.
# Lexing

```
$ irb

irb(main):001> require "Prism"
=> false

irb(main):002> Prism.lex("144 / 22")
=>
#<Prism::LexResult:0x000000011f94faf0
 @comments=[],
 @data_loc=nil,
 @errors=[],
 @magic_comments=[],
 @source=#<Prism::ASCIISource:0x00000001201cbee0 @offsets=[0], @source="144 / 22", @start_line=1>,
 @value=[[INTEGER(1,0)-(1,3)("144"), 2], [SLASH(1,4)-(1,5)("/"), 1], [INTEGER(1,6)-(1,8)("22"), 2], [EOF(1,8)-(1,8)(""), 2]],
 @warnings=
  [#<Prism::ParseWarning @type=:void_statement @message="possibly useless use of / in void context" @location=#<Prism::Location @start_offset=0 @length=8 start_line=1> @level=:verbose>]>
```

# Parsing

```
$ irb

irb(main):001> require "Prism"
=> false

irb(main):003> Prism.parse("144 / 22")
=>
#<Prism::ParseResult:0x0000000125d39340
 @comments=[],
 @data_loc=nil,
 @errors=[],
 @magic_comments=[],
 @source=#<Prism::ASCIISource:0x000000011fc08d78 @offsets=[0], @source="144 / 22", @start_line=1>,
 @value=
  @ ProgramNode (location: (1,0)-(1,8))
  ├── flags: ∅
  ├── locals: []
  └── statements:
      @ StatementsNode (location: (1,0)-(1,8))
      ├── flags: ∅
      └── body: (length: 1)
          └── @ CallNode (location: (1,0)-(1,8))
              ├── flags: newline
              ├── receiver:
              │   @ IntegerNode (location: (1,0)-(1,3))
              │   ├── flags: static_literal, decimal
              │   └── value: 144
              ├── call_operator_loc: ∅
              ├── name: :/
              ├── message_loc: (1,4)-(1,5) = "/"
              ├── opening_loc: ∅
              ├── arguments:
              │   @ ArgumentsNode (location: (1,6)-(1,8))
              │   ├── flags: ∅
              │   └── arguments: (length: 1)
              │       └── @ IntegerNode (location: (1,6)-(1,8))
              │           ├── flags: static_literal, decimal
              │           └── value: 22
              ├── closing_loc: ∅
              └── block: ∅,
 @warnings=
  [#<Prism::ParseWarning @type=:void_statement @message="possibly useless use of / in void context" @location=#<Prism::Location @start_offset=0 @length=8 start_line=1> @level=:verbose>]>
```