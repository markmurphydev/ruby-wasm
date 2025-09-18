NB: `<LABEL> ::= ... | ε`, so treat `<LABEL>` as `<LABEL>?`
# If
```
'(' if <LABEL> <BLOCKTYPE> predicate:<INSTR>+
	'(' then <INSTR>* ')'
	( '(' else <INSTR>* ')' )? ')'
```

# Loop
```
'(' loop <LABEL> <BLOCKTYPE>
	<INSTR>* ')'
```