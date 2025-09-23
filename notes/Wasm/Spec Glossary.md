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
A reference to a type definition.
WAT:
```
TYPEUSE = (type <TYPEIDX>)
		| (type <TYPEIDX>) <PARAM>* <RESULT>*
```

## Abbreviation
Type use can be replaced entirely with inline `<PARAM>, <RESULT>` declarations.
In that case, we auto-insert a type index.