Points: 2

Needs to load the given .wasm file, run its start function, then `console.log` the result.
I'm certain MDN or someone has a guide on this.

I think this could be just one html page with a file picker button.
Afaik all Wasm loading/running is done "dynamically" in JS, so it should just work with the loaded file.

This should give out-of-the-box `.wasm` debugging support in Chrome.
- At the Wasm bytecode level, not the source-program level.
- Pretty sure gdb-style debugging requires DWARF info here. No thanks.
