https://www.youtube.com/watch?v=fMGuQXNqlaE
Transcript:
[[rossberg_2023]]
# Wasm
- "Not a web technology"
- [ ] "deterministic (with asterisks)" ?
- WC3 group. Working group, subgroup meetings are open to the public.
- "Transparent, explicit cost model"
	- [ ] So, no built-in object model 
		- How do these relate? 
		- What would a built-in object model look like? JVM I assume.
- "Most interesting optimization should be on compiler side"
	- i.e., us.

## Example – Functions
- Functions are a primitive notion - funcs and their params, `call`, `ret`
- Doesn't expose the stack – calling convention left up to engine
	- This lets the engine optimize things like register allocation per-arch
	- If you do "shadow stack" tricks, does it kill this angle on security?
- [ ] Function tables model function pointers
	- Harvard arch, not von Neumann
- Functions are "just code" – not closures or methods

## Garbage collection
- Motivations from before
- [ ] "No safe exchange of refs with host"
	- Think about what a 2-GC model would look like, why does it break?

Non-goals:
- Automatic multi-language interop
	- 「define your own damn ABI」

Stripped-down GC:
- No complex type system
- Just describes memory layout, not the source types
	- High-level types must be lowered
	- [ ] Write very clearly why you still need wasm-level subtyping in this case
- "No hidden costs". So, avoid:
	- Language-specific constructs
	- Heavyweight objects/types
	- Reified generics
		- [ ] What does this look like?
	- Unmodular semantics
		- [ ] Allegedly, nominal typing at vm-level makes it impossible to lower structurally-typed languages modularly
			- I think cause you'd have to control type names for all modules used?

- GC heap completely separate from Wasm's linear memory
- Opaque refs – GC pointers aren't exposed to wasm programs
- GC refs can't be stored in linear memory
	- [ ] 「Would break the abstraction」- Why?
	- Can be stored in tables, though.

# Primitives
- Tuple refs – heterogenous, static indexing
- Array refs – homogenous, dynamic indexing
- Scalars – unboxed `i31` ints as tagged pointers
- Function refs – code pointers
- Externals – host pointers

## Why `i31`?
- "Have to be portable" (I assume he means to 32b platforms)
- Want predictable performance
	- So, don't want a "polyfill" for larger scalars for 32b platforms?

# Typing
- Refs are strongly-typed, to avoid runtime access checks
	- You need to know that eg. a tuple's 3rd field is there, what size it is, if it's a ref, ...

```
(type $point (field f64 f64))

(func $fst (param $p (ref $point))
		   (result f64)
	(local.get $p)
	(struct.get 0)) ;; Infalliable, validated by type system
```

# Subtyping
- Required to compile languages with subtyping
	- Don't want to require a checked casting instruction whenever subtyping is used in the source language.
- [ ] "Usual width & depth subtyping on structs & arrays"
	- "Invariant on mutable fields (soundness)" ???
	- "Contra/covariant subtyping on functions"
	- "In many cases, source-level subsumption commutes with lowering"
	- "Needs escape hatch" – checked runtime casts
	- "Can't subsume all languages' dynamic type systems"

# Example – Classes

```
class C {
	var a: Int
	method f(x: Int): Int { x + a }
	method g(x: Int, y: Int): Int { x + y }
}

class D <: C {
	var b: Int
	method h(x: Int): Int { x + a + b }
}
```

```
C    | Ref<VTable> | -----> VTable<C>    | f |
	 | a           |                     | g |
	 
D    | Ref<VTable> | -----> VTable<D>    | f |
	 | a           |                     | g |
	 | b           |                     | h |
```

- Want to have `D <: C` at wasm-level
	- So, need `VTable<D> <: VTable<C>`

## Lowering
```
type $C    = struct (ref $VT_C) i64
type $VT_C = struct (ref $C.f) (ref $C.g)
type $C.f  = func (ref $C) i64 -> i64
type $C.g  = func (ref $C) i64 i64 -> i64

(func $C.f (param $this (ref $C))
		   (param $x i64)
		   (result f64)
	(i32.add $x 
	         (struct.get 1 $this)))
			 
type $D    = struct (ref $VT_D) i64 i64
type $VT_D = struct (ref $C.f) (ref $C.g) (ref $D.h)
type $D.h  = func (ref $D) i64 -> i64

; calling example
; `d.f(7)`
(call_ref (struct.get 0 (struct.get 0 $d))    ; d.vtable.f
          $d
          (i32.const 7))
```

- [ ] Is it automatic that `D <: C` and `VT_D <: VT_C`?

### What if `D` overrides `f`?
```
class D <: C {
    var b: Int
    method h(x: Int): Int { x + a + b }
	override method f(x: Int): Int { x + b }
}
...
type $D.f = $C.f = func (ref $C) i64 -> i64
```
- [ ] "A well-known problem that the `this` arg is covariant in object encodings."
	- "But, that doesn't work with sound contravariant function subtyping"

How to implement `D.f`?
- Runtime cast
```
(func $D.f (param $this.C (ref $C))
		   (param $x i64)
		   (result f64)
	(local $this (ref.cast $D $this.C))
	(i32.add $x 
	         (struct.get 2 $this)))
```

- [ ] "There's a type system extension that would fix this case"

# Example – Closures
- [ ] "Degenerate case of a class with a single method"
	- Can flatten the vtable into the object

```
fun update (a: Array Int) (f: Int -> Int) =
	for i in 0 to a.size do
		a[i] := f(a[i])

fun shift (n: Int) (a: Array Int) =
	update a (fn x => x + n)
              ^^^^^^^^^^^^^
```

- [ ] So, each closure is a function with its own anonymous wasm-level type?
	- Or every closure with the same shape shares a type?

```
type $array-i64 = array i64                                ;; ???
;; The one that gets passed around:
type $clos-i64-i64 = struct (ref $code-i64-i64)
type $code-i64-i64 = func (ref $clos-i64-i64) i64 -> i64

(func $update (param $a (ref $array-i64))
		      (param $f (ref $clos-i64-i64))
		      (result nil)
	(local $i i64)
	(loop
		...
		(call-ref (struct.get 0 $f)
		          $f
		          (array.get $a $i))
		...)
```

- Same problem as classes

```
type $our-closure = subtype $clos-i64-i64 (ref $code-i64-i64) i64

(func $shift (param $n i64)
		     (param $a (ref $array-i64))
		     (result nil)
	(call $update
		  $a
		  (struct.new $our-closure
					  (ref.func $our-fn)
					  $n)))

(func $our-fn (param $clos (ref $clos-i64-i64))
		      (param $x i64)
		      (result nil)
	; x + n
	(i64.add $x
			 (struct.get 1
						 (ref.cast $our-closure $clos)))
```

# Uniform representation (dynamic typing)
- All values are repr'd as reference types subtype `any`
	- Cast only when using the type concretely

He talks a lot about highly-polymorphic languages like Ocaml, which apparently use this.
But we care about ultimately-polymorphic\[runtime-checked] languages like Ruby.

```
ML type        unboxed        uniform repr
-------        -------        ------------
bool           i8             i31
char           i32            i31
int            i32            i31/(struct i32)     Can be mixed repr, like Scheme
float          f64            (struct f64)
String         –              (array i8)
(t, u)         –              (struct (ref any) (ref any))
```

Limitations:
- Overhead of occasional casts
	- 2%-4% reported by early compiler implementors
- Doesn't cover special GC needs
	- Go has `interior pointers` into objects
	- "No story for weak refs, finalization"
- "Compiler backends need sufficient type info"
	- Problem for backends for legacy frontends, not my problem.