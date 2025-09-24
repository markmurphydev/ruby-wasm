# Can you do bare type definitions in a module?
```
(module
	(type $a (struct)))
```

Yes! You can drop the outer `(rec)` and `(sub)`.
https://webassembly.github.io/spec/core/text/types.html#id2

# Can you refer to types in inline function typedefs?
```
(module
	(type $unitype (ref eq))
	(function $a (result (ref $unitype))))
```

# How to convert `i31 <-> i31`?
## `i32 -> i31`
```lisp
(i32.const 22)
(ref.i31)
```

## `i31 -> i32`
```lisp
; unsigned i31 on stack:
(i31.get_u)
; signed i31 on stack:
(i31.get_s)
```
