# Rust Standard Library Macros

**Total: 81 macros** (69 regular macros + 12 derive macros)

## Top-Level Macros (43)

### Stable
- `assert!`
- `assert_eq!`
- `assert_ne!`
- `cfg!`
- `column!`
- `compile_error!`
- `concat!`
- `dbg!`
- `debug_assert!`
- `debug_assert_eq!`
- `debug_assert_ne!`
- `env!`
- `eprint!`
- `eprintln!`
- `file!`
- `format!`
- `format_args!`
- `include!`
- `include_bytes!`
- `include_str!`
- `is_x86_feature_detected!`
- `line!`
- `matches!`
- `module_path!`
- `option_env!`
- `panic!`
- `print!`
- `println!`
- `stringify!`
- `thread_local!`
- `todo!`
- `try!` (deprecated)
- `unimplemented!`
- `unreachable!`
- `vec!`
- `write!`
- `writeln!`

### Unstable
- `cfg_select!`
- `concat_bytes!`
- `const_format_args!`
- `format_args_nl!`
- `log_syntax!`
- `trace_macros!`

## Architecture Macros (13)

### Assembly
- `asm!`
- `global_asm!`
- `naked_asm!`

### Feature Detection (Stable)
- `is_x86_feature_detected!`
- `is_aarch64_feature_detected!`
- `is_loongarch_feature_detected!`
- `is_riscv_feature_detected!`

### Feature Detection (Unstable)
- `is_arm_feature_detected!`
- `is_mips_feature_detected!`
- `is_mips64_feature_detected!`
- `is_powerpc_feature_detected!`
- `is_powerpc64_feature_detected!`
- `is_s390x_feature_detected!`

## Module-Specific Macros (13)

### std::assert_matches (Unstable)
- `assert_matches!`
- `debug_assert_matches!`

### std::future
- `join!`

### std::intrinsics::mir (Unstable)
- `mir!`
- `place!`

### std::io (Unstable)
- `const_error!`

### std::iter (Unstable)
- `iter!`

### std::mem
- `offset_of!`

### std::pat (Unstable)
- `pattern_type!`

### std::pin
- `pin!`

### std::ptr
- `addr_of!`
- `addr_of_mut!`

### std::prelude::v1 (Unstable)
- `deref!`
- `type_ascribe!`

### std::simd (Unstable)
- `simd_swizzle!`

### std::task
- `ready!`

## Derive Macros (12)

### Stable
- `Clone`
- `Copy`
- `Debug`
- `Default`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`

### Unstable
- `CoercePointee`
- `ConstParamTy`
- `UnsizedConstParamTy`
