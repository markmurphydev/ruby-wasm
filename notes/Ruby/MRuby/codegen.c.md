# Entrypoint: `generate_code()`

```
static struct RProc*
generate_code(mrb_state *mrb, parser_state *p, int val)
{ ... }
```
Strategy:
- Create a new scope
- call `codegen`...
# RProc
- [ ] What's an `RProc`?
```
struct RProc {
  MRB_OBJECT_HEADER;
  union {
    const mrb_irep *irep;
    mrb_func_t func;
    mrb_sym mid;
  } body;
  const struct RProc *upper;
  union {
    struct RClass *target_class;
    struct REnv *env;
  } e;
};
```
- "represents the script or code"
# MRB_TRY(...)
- Macro to implement try/catch in MRB source code.

# `codegen()`
This is where to look for MRuby AST -> bytecode lowering

- Switch on `nint(tree->car)`
	- I assume node type

## NODE_CONST
```
case NODE_CONST:
	{
	  int sym = new_sym(s, nsym(tree));
	
	  genop_2(s, OP_GETCONST, cursp(), sym);
	  if (val) push();
	}
	break;
```
- `nsym(...)` casts to symbol
- `new_sym(...)` returns symbol idx
- Produces `OP_GETCONST(s->sp, symbol_idx)`
	- Where `s->sp` is `R(a)`
	- [ ] What does `R(a)` mean?

- [ ] `if (val) push()` ?
- [x] Why only `sym`?
	This means "accessing a constant variable"
	You refer to constant variables by their symbol name.

## NODE_INT
```
case NODE_INT:
if (val) {
  char *p = (char*)tree->car;
  int base = nint(tree->cdr->car);
  mrb_int i;
  mrb_bool overflow;

  i = readint(s, p, base, FALSE, &overflow);
  if (overflow) {
	int off = new_litbint(s, p, base);
	genop_2(s, OP_LOADL, cursp(), off);
  }
  else {
	gen_int(s, cursp(), i);
  }
  push();
}
break;
```

- `gen_int` just chooses a convenient opcode to load the correct integer
	- [x] Why doesn't it load it in a packed representation?
		No it does. In the vm, calls `SET_FIXNUM_VALUE`, which dispatches on boxing method.
