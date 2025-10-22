//! Test that [ruby_wasm::corelib] (or `./generate-corelib.lisp` for now)
//! is producing Wasm that:
//! - Compiles and type-checks
//! - Runs correctly

use ruby_wasm::corelib::add_core_items;
use ruby_wasm::{run, CompileCtx};
use wat_defs::instr::Instr;
use wat_defs::module::Module;
use wat_defs::ty::{NumType, ValType};
use wat_macro::wat;

fn run_helper(body: Vec<Instr>, result_ty: ValType) -> String {
    let module = Module::new();
    let mut ctx = CompileCtx::new(module);
    add_core_items(&mut ctx);

    let main_fn = wat! {
        (func $__ruby_top_level_function
            (export ,("__ruby_top_level_function".to_string()))
            (result ,(result_ty))
            ,(body))
    };
    ctx.module.funcs.push(main_fn);

    run::run_wat(run::compile_ctx_to_wat(&ctx))
}

/// Wraps `body` in a function definition, includes the corelib definitions,
/// and runs the file.
/// `body` must be the body of a wasm function `() -> (ref eq)`
fn run_main_fn_ref_eq(body: Vec<Instr>) -> String {
    run_helper(body, ValType::Ref(wat![ (ref eq) ]))
}

fn run_main_fn_i64(body: Vec<Instr>) -> String {
    run_helper(body, ValType::Num(NumType::I64))
}

#[test]
pub fn run_without_panicking() {
    let main_fn = wat! {
        (ref_i31 (const_i32 3))
    };
    let res = run_main_fn_ref_eq(main_fn);
    println!("{}", res);
}

mod function {
    mod is_i64 {
        use crate::{run_main_fn_i64, run_main_fn_ref_eq};
        use expect_test::expect;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;

        #[test]
        pub fn true_22() {
            let input = wat! {
                (call $to_bool (call $is_fixnum (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect![["true"]];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn true_neg_1() {
            let input = wat! {
                (call $to_bool (call $is_fixnum (ref_i31 (const_i32 ,(Unitype::from_integer(-1).to_i31_bits() as i64)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["true"];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn true_max() {
            let fixnum_max = Unitype::FIXNUM_MAX_VAL;
            let input = wat! {
                (call $to_bool (call $is_fixnum (ref_i31 (const_i32 ,(Unitype::from_integer(fixnum_max).to_i31_bits() as i64)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["true"];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn true_min() {
            let fixnum_min = Unitype::FIXNUM_MIN_VAL;
            let input = wat! {
                (call $to_bool (call $is_fixnum (ref_i31 (const_i32 ,(Unitype::from_integer(fixnum_min).to_i31_bits() as i64)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["true"];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn false_max_plus_one() {
            let fixnum_max_plus_one = Unitype::FIXNUM_MAX_VAL + 1;
            let input = wat! {
                (call $to_bool (call $is_fixnum (struct_new $boxnum (const_i64 ,(fixnum_max_plus_one)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["false"];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn false_min_minus_one() {
            let fixnum_min_minus_one = Unitype::FIXNUM_MIN_VAL - 1;
            let input = wat! {
                (call $to_bool (call $is_fixnum (struct_new $boxnum (const_i64 ,(fixnum_min_minus_one)))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["false"];
            expected.assert_eq(&actual);
        }
    }

    mod integer_to_i64 {
        use crate::{run_main_fn_i64, run_main_fn_ref_eq};
        use expect_test::expect;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;

        #[test]
        pub fn true_22() {
            let input = wat! {
                (call $fixnum_to_i64 (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64))))
            };
            let actual = run_main_fn_i64(input);
            let expected = expect![["22"]];
            expected.assert_eq(&actual);
        }
    }

    mod i64_to_integer {
        use crate::run_main_fn_ref_eq;
        use expect_test::expect;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;

        #[test]
        pub fn test_22() {
            let input = wat! {
                (call $i64_to_integer (const_i64 22))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect![["22"]];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn test_max_plus_one() {
            let fixnum_max_plus_one = Unitype::FIXNUM_MAX_VAL + 1;
            let input = wat! {
                (call $i64_to_integer (const_i64 ,(fixnum_max_plus_one)))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["536870912"];
            expected.assert_eq(&actual);
        }
    }

    mod add {
        use expect_test::expect;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;
        use crate::run_main_fn_ref_eq;

        #[test]
        pub fn test_max_plus_one() {
            let fixnum_max = Unitype::FIXNUM_MAX_VAL;
            let input = wat! {
                (call $add (ref_i31 (const_i32 ,(Unitype::from_integer(fixnum_max).to_i31_bits() as i64)))
                           (ref_i31 (const_i32 ,(Unitype::from_integer(1).to_i31_bits() as i64))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect!["536870912"];
            expected.assert_eq(&actual);
        }
    }

    mod negate {
        use expect_test::expect;
        use ruby_wasm::corelib::helpers;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;
        use crate::{run_main_fn_i64, run_main_fn_ref_eq};

        #[test]
        pub fn i64_neg() {
            let input = wat! {
                ,(helpers::i64_neg(wat![ (const_i64 22) ]))
            };
            eprintln!("{:?}", input);
            let actual = run_main_fn_i64(vec![input]);
            let expected = expect![["-22"]];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn test_22() {
            let input = wat! {
                (call $negate (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect![["-22"]];
            expected.assert_eq(&actual);
        }
    }

    mod eq_eq {
        use expect_test::expect;
        use ruby_wasm::unitype::Unitype;
        use wat_macro::wat;
        use crate::run_main_fn_ref_eq;

        #[test]
        pub fn equal() {
            let input = wat! {
                (call $eq_eq (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64)))
                             (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect![["true"]];
            expected.assert_eq(&actual);
        }

        #[test]
        pub fn not_equal() {
            let input = wat! {
                (call $eq_eq (ref_i31 (const_i32 ,(Unitype::from_integer(22).to_i31_bits() as i64)))
                             (ref_i31 (const_i32 ,(Unitype::from_integer(44).to_i31_bits() as i64))))
            };
            let actual = run_main_fn_ref_eq(input);
            let expected = expect![["false"]];
            expected.assert_eq(&actual);
        }
    }
}
