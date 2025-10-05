In summary:
- Get mrb_int values
- Decode their C int values
- Do math, checking for overflow
	- No overflow -> store number
		- Fits in fixnum -> fixnum
		- Doesn't -> boxed Integer
	- Overflow -> calculate & store bignum result

# Codegen
- Checks method names for operator names, and always converts to `OP_ADD`, ... VM instrs.
- Doesn't let you re-define operators like CRuby
# Code eval

`vm.c`

```
CASE(OP_ADD, B) {
  OP_MATH(add);
}
```

```
#define OP_MATH(op_name)                                                    \
  /* need to check if op is overridden */                                   \
  switch (TYPES2(mrb_type(regs[a]),mrb_type(regs[a+1]))) {                  \
    OP_MATH_CASE_INTEGER(op_name);                                          \
    OP_MATH_CASE_FLOAT(op_name, integer, float);                            \
    OP_MATH_CASE_FLOAT(op_name, float,  integer);                           \
    OP_MATH_CASE_FLOAT(op_name, float,  float);                             \
    OP_MATH_CASE_STRING_##op_name();                                        \
    default:                                                                \
      mid = MRB_OPSYM(op_name);                                             \
      goto L_SEND_SYM;                                                      \
  }                                                                         \
  NEXT;
```

```
#define OP_MATH_CASE_INTEGER(op_name)                                       \
  case TYPES2(MRB_TT_INTEGER, MRB_TT_INTEGER):                              \
    {                                                                       \
      mrb_int x = mrb_integer(regs[a]), y = mrb_integer(regs[a+1]), z;      \
      if (mrb_int_##op_name##_overflow(x, y, &z)) {                         \
        OP_MATH_OVERFLOW_INT(op_name,x,y);                                  \
      }                                                                     \
      else                                                                  \
        SET_INT_VALUE(mrb,regs[a], z);                                      \
    }                                                                       \
    break
```

- Gets a C int out of a `mrb_value`
- Checks for overflow
	- [ ] What is `##op_name##`? 
		- Concatenates `op_name` into the function name
		- Here, if `op_name == "add"`, becomes `mrb_int_add_overflow(x, y, &z)`
- Does the math then repacks the value
- `mrb_integer(...)` defined in `boxing_<BOX_TYPE>.md`

For `boxing_word.h`:
```
mrb_integer_func(mrb_value o) {
  if (mrb_immediate_p(o)) return mrb_fixnum(o);
  return mrb_val_union(o).ip->i;
}
#define mrb_integer(o) mrb_integer_func(o)
```

- Checks if the value is an immediate

```
#define mrb_immediate_p(o) ((o).w & WORDBOX_IMMEDIATE_MASK || (o).w <= MRB_Qundef)
```

- C bullshit.

```
static inline union mrb_value_
mrb_val_union(mrb_value v)
{
  union mrb_value_ x;
  x.value = v;
  return x;
}
```

```
/*
 * mrb_value representation:
 *
 * 64-bit word with inline float:
 *   nil   : ...0000 0000 (all bits are 0)
 *   false : ...0000 0100 (mrb_fixnum(v) != 0)
 *   true  : ...0000 1100
 *   undef : ...0001 0100
 *   symbol: ...0001 1100 (use only upper 32-bit as symbol value with MRB_64BIT)
 *   fixnum: ...IIII III1
 *   float : ...FFFF FF10 (51 bit significands; require MRB_64BIT)
 *   object: ...PPPP P000
 *
 * 32-bit word with inline float:
 *   nil   : ...0000 0000 (all bits are 0)
 *   false : ...0000 0100 (mrb_fixnum(v) != 0)
 *   true  : ...0000 1100
 *   undef : ...0001 0100
 *   symbol: ...SSS1 0100 (symbol occupies 20bits)
 *   fixnum: ...IIII III1
 *   float : ...FFFF FF10 (22 bit significands; require MRB_64BIT)
 *   object: ...PPPP P000
 *
 * and word boxing without inline float (MRB_WORDBOX_NO_FLOAT_TRUNCATE):
 *   nil   : ...0000 0000 (all bits are 0)
 *   false : ...0000 0100 (mrb_fixnum(v) != 0)
 *   true  : ...0000 1100
 *   undef : ...0001 0100
 *   fixnum: ...IIII III1
 *   symbol: ...SSSS SS10
 *   object: ...PPPP PP00 (any bits are 1)
 */
typedef struct mrb_value {
  uintptr_t w;
} mrb_value;

union mrb_value_ {
  void *p;
  struct RBasic *bp;
#ifndef MRB_NO_FLOAT
#ifndef MRB_WORDBOX_NO_FLOAT_TRUNCATE
  mrb_float f;
#else
  struct RFloat *fp;
#endif
#endif
  struct RInteger *ip;
  struct RCptr *vp;
  uintptr_t w;
  mrb_value value;
};
```

- In 32-bit system word boxing, fixnums are `i31`'s, just like Wasm

```
struct RInteger {
  MRB_OBJECT_HEADER;
  mrb_int i;
};
```

# Set int value
`boxing_word.h`
```
#define SET_INT_VALUE(mrb,r,n) ((r) = mrb_boxing_int_value(mrb, n))
```

`etc.c`
```
#if defined(MRB_WORD_BOXING) || (defined(MRB_NAN_BOXING) && defined(MRB_INT64))
/*
 * Boxes an `mrb_int` into an `mrb_value`.
 * If the integer `n` can be represented as a fixnum (checked by `FIXABLE(n)`),
 * it returns a fixnum-tagged `mrb_value`. Otherwise, it allocates an
 * `RInteger` object on the heap, stores `n` in it, marks the object as
 * frozen, and returns an object-tagged `mrb_value`.
 * This function is used when word boxing is enabled or when NaN boxing is
 * enabled for 64-bit integers.
 */
MRB_API mrb_value
mrb_boxing_int_value(mrb_state *mrb, mrb_int n)
{
  if (FIXABLE(n)) return mrb_fixnum_value(n);
  else {
    mrb_value v;
    struct RInteger *p = (struct RInteger*)mrb_obj_alloc(mrb, MRB_TT_INTEGER, mrb->integer_class);
    p->i = n;
    p->frozen = 1;
    SET_OBJ_VALUE(v, p);
    return v;
  }
}
#endif
```

- Boxed integers and Bigint are _not_ the same
	- Boxed integers are used as soon as you exceed fixnum size
	- Bigint is only used if platform-sized integer arithmetic would _overflow_
# Overflow checking
`numeric.h`

```
#ifdef MRB_HAVE_TYPE_GENERIC_CHECKED_ARITHMETIC_BUILTINS

static inline mrb_bool
mrb_int_add_overflow(mrb_int augend, mrb_int addend, mrb_int *sum)
{
  return __builtin_add_overflow(augend, addend, sum);
}

...
#endif
```

- Checks for GCC integer overflow builtins
	- https://gcc.gnu.org/onlinedocs/gcc/Integer-Overflow-Builtins.html
	- I think clang has them too.
- Nothing weird here.

# Overflow Handling

```
#ifdef MRB_USE_BIGINT
#define OP_MATH_OVERFLOW_INT(op,x,y) regs[a] = mrb_bint_##op##_ii(mrb,x,y)
#else
#define OP_MATH_OVERFLOW_INT(op,x,y) goto L_INT_OVERFLOW
#endif
```

```
L_INT_OVERFLOW:
	RAISE_LIT(mrb, E_RANGE_ERROR, "integer overflow");
```

- If we're not using Bigints, raise an exception

## Bigint math
`bigint.c`

```
/* Barrett reduction for efficient modular arithmetic with repeated operations */

...

mrb_value
mrb_bint_add_ii(mrb_state *mrb, mrb_int x, mrb_int y)
{
  mpz_t a, b, z;
  MPZ_CTX_INIT(mrb, ctx, pool);

  mpz_init(ctx, &z);
  mpz_init_set_int(ctx, &a, x);
  mpz_init_set_int(ctx, &b, y);
  mpz_add(ctx, &z, &a, &b);
  mpz_clear(ctx, &a);
  mpz_clear(ctx, &b);
  return bint_norm(mrb, bint_new(ctx, &z));
}
```

- Euhhh...