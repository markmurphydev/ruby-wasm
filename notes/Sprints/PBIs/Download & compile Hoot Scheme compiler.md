This is the simplest compiler I'm aware of that compiles to a Wasm 3.0 / WasmGC target.

I'd like to start by investigating the 0.1.0 release:
https://community.spritely.institute/t/guile-hoot-v0-1-0-released/298
It looks a little simpler, while still having a full Wasm toolchain.

Building from source assumes I have Guix, which is not available on MacOS:
https://files.spritely.institute/docs/guile-hoot/0.1.0/Installation.html
It also says it depends on `main` branch Guile.
I think I'll have to go look through the Guile repo to find a `main` branch commit that matches the date of the Hoot `0.1.0` tag.
Or maybe there's a Guix config file that lists the right Guile commit?