# block
Wasm instruction.
```
(block <LABEL>? <BLOCKTYPE> <INSTR>*)
```
From within the block, you can branch _out_ of the block, like a `break` statement.