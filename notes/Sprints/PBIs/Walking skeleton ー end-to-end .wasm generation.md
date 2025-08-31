The whole compiler pipeline. Take in a `.rb` file and output an executable (vm-able?) `.wasm` file.
The point is just to show that we can output a Wasm-compliant file.

This will require:
- Choosing _some_ AST representation for a tiny subset of the lang (Program, Statement, bools, restricted ints perhaps).
	- Extremely subject to revision
- Writing out data types mirroring the Wasm module structure
- Mapping the tiny AST subset to the Wasm types
	- Output structure _extremely_ subject to revision
- Serializing to `.wat` text format
	- It's correct to call this "serializing" right?

I don't want to worry about the Wasm binary format just yet.
I can output `.wat` and convert it with one of the [[Wasm#Binutils]] programs

Then should just test that the compiled programs give expected output when run.