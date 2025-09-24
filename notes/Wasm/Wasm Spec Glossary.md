# Function  

# block
Wasm instruction.
WAT:
```
(block <LABEL>? <BLOCKTYPE> <INSTR>*)
```
From within the block, you can branch _out_ of the block, like a `break` statement.

# Recursive Types
Mutually recursive composite types. Each can declare a list of type uses of supertypes it matches.
WAT:
```
RECTYPE = (rec <TYPEDEF>*)
TYPEDEF = (type <ID>? <SUBTYPE>)
SUBTYPE = (sub final? <TYPEIDX>* <COMPTYPE>)
```

# Type Use
A reference to a type definition. Think "type reference"
WAT:
```
TYPEUSE = (type <TYPEIDX>)
		| (type <TYPEIDX>) <PARAM>* <RESULT>*
```
The `<PARAM>*` and `<RESULT>*` are for binding symbols to the local indices of the parameters.
They don't add anything new to the type reference.
But, a type use can be replaced _entirely_ by inline `<PARAM>* <RESULT>*`
## Abbreviation
Type use can be replaced entirely with inline `<PARAM>, <RESULT>` declarations.
In that case, we auto-insert a type index.

# Type Index
In WAT, type index can be a `u32` index or a `$symbol`

# Control Instructions
NB: `<LABEL> ::= ... | ε`, so treat `<LABEL>` as `<LABEL>?`
## If
```
'(' if <LABEL> <BLOCKTYPE> predicate:<INSTR>+
	'(' then <INSTR>* ')'
	( '(' else <INSTR>* ')' )? ')'
```

## Loop
```
'(' loop <LABEL> <BLOCKTYPE>
	<INSTR>* ')'
```