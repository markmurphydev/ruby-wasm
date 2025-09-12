- All values are of the `SCM` unitype
	- Represented as `(ref eq)`
		- Non-nullable
			- We can use subtypes of that (so, heap types and `(ref i31)`)

- Numbers are either `fixnum`'s or `bignum`'s
	- No separate boxed i32

# Immediate values
- Partition of `(ref i31)`
- Half to signed fixnums in range `[-2^29, 2^29 - 1]`
- 1/4 to chars
- The rest to `#f, nil, '(), #t, unspecified value, EOF, undefined value`

# Heap values

All subtypes of `$heap-object`

```wat
(type $void-struct (struct))
(type $heap-object
  (sub $void-struct
    (struct
      (field $hash (mut i32)))))
```

- Idk why the hash
> For symbols and keywords, the `$hash` field is eagerly computed based on the string-hash of the underlying string.  For other data types, the hash is computed lazily. A hash of 0 indicates an uninitialized hash. Bit 0 is always set if the hash is initialized.  Otherwise for immediate values, there is a simple bit-mixing hash function.

## Heap Numbers

```wat
(type $heap-number
  (sub $heap-object
    (struct
      (field $hash (mut i32)))))
```

There is a supertype for heap numbers, in case we need to quickly check
that a non-fixnum is indeed a number.  Then there are the concrete heap
number types.

```wat
(type $bignum
  (sub $heap-number
    (struct
      (field $hash (mut i32))
      (field $val (ref extern)))))
(type $flonum
  (sub $heap-number
    (struct
      (field $hash (mut i32))
      (field $val f64))))
(type $complex
  (sub $heap-number
    (struct
      (field $hash (mut i32))
      (field $real f64)
      (field $imag f64))))
(type $fraction
  (sub $heap-number
    (struct
      (field $hash (mut i32))
      (field $num (ref eq))
      (field $denom (ref eq)))))
```