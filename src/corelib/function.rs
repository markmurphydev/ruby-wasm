use crate::corelib::alist::AListTypeDef;
use crate::corelib::class::Class;
use crate::corelib::helpers::i64_neg;
use crate::corelib::{class, method};
use crate::unitype::Unitype;
use crate::{CompileCtx, corelib};
use wat_defs::func::Func;
use wat_defs::instr::Instr;
use wat_macro::wat;

pub fn add_functions(ctx: &mut CompileCtx) {
    add_start(ctx);

    for func in funcs() {
        ctx.module.funcs.push(func);
    }
}

fn funcs() -> Vec<Func> {
    vec![
        str_eq(),
        alist_str_method_get(),
        call(),
        is_nil(),
        is_fixnum(),
        is_boxnum(),
        sign_extend(),
        sign_extend_fixnum(),
        fixnum_to_i64(),
        boxnum_to_i64(),
        integer_to_i64(),
        in_fixnum_range(),
        i32_to_fixnum(),
        i64_to_fixnum(),
        i64_to_boxnum(),
        i64_to_integer(),
        add(),
        to_bool(),
        from_bool(),
        negate(),
        and(),
        or(),
        lt(),
        gt(),
        eq_eq(),
        arr_to_js(),
        unitype_to_js(),
    ]
}

/// The `start` function runs when the module is loaded.
/// We use it to set up cyclic object references:
/// - `Class.parent`
/// - `Class.superclass`
fn add_start(ctx: &mut CompileCtx) {
    let mut instrs = vec![];
    let classes = class::corelib_classes();
    for class in classes {
        let class_identifier = class.identifier();
        let parent_identifier = Class::name_to_identifier(&class.parent_name);
        let superclass_identifier = class
            .superclass_name
            .map(|name| Class::name_to_identifier(&name));

        instrs.append(&mut wat! {
            (struct_set $class $parent
                (global_get ,(class_identifier.clone()))
                (global_get ,(parent_identifier)))
        });

        // set class superclass
        if let Some(superclass_identifier) = superclass_identifier {
            instrs.append(&mut wat! {
                (struct_set $class $superclass
                    (global_get ,(class_identifier))
                    (global_get ,(superclass_identifier)))
            })
        }
    }

    // Instantiate global $main object
    instrs.append(&mut wat! {
        (global_set $main
            (ref_cast (ref $obj)
                (call ,(method::class_new().identifier()) // ($self, $args) -> ...
                      (global_get ,(class::object().identifier()))
                      (global_get $empty_args))))
    });

    let start_fn = wat! {
        (func $_start
            ,(instrs))
    };

    ctx.module.funcs.push(start_fn);
    ctx.module.start_fn = Some("_start".to_string());
}

/// `str-eq : (ref $str) (ref $str) -> Bool`
fn str_eq() -> Func {
    wat! {
        (func $str_eq
            (param $a (ref $str))
            (param $b (ref $str))
            (result i32)
            (local $idx i32) (local $a_ch i32) (local $b_ch i32)
            // if (a.len != b.len) { return false }
            // for (a_ch, b_ch) in zip(a, b) {
            //   if (a_ch != b_ch) { return false }
            // }
            // return true
            (local_set $idx (const_i32 0))
            (if (i32_eqz (i32_eq (array_len (local_get $a))
                                 (array_len (local_get $b))))
                (then (return (const_i32 0))))
            (loop $for (result (ref eq))
                (if (i32_eq (local_get $idx)
                            (array_len (local_get $a)))
                    (then (return (const_i32 1))))
                (local_set $a_ch (array_get_u $str (local_get $a) (local_get $idx)))
                (local_set $b_ch (array_get_u $str (local_get $b) (local_get $idx)))
                (if (i32_eqz (i32_eq (local_get $a_ch)
                                     (local_get $b_ch)))
                    (then (return (const_i32 0))))
                (local_set $idx (i32_add (local_get $idx)
                                         (const_i32 1)))
                (br $for))
            (unreachable))
    }
}

const ALIST_STR_METHOD_GET_IDENTIFIER: &str = "alist-str-method-get";

/// TODO: This should be genericized for any type of alist we have.
fn alist_str_method_get() -> Func {
    wat! {
        (func $alist_str_method_get
            (param $alist (ref $alist_str_method))
            (param $name (ref $str))
            (result (ref $method))
            (local $idx i32)
            (local $pair (ref $alist_str_method_pair))
            (local $key (ref $str))
            (local $val (ref $method))

            (local.set $idx (const_i32 0))
            (loop $for (result (ref eq))
                (if (i32_eq (local_get $idx)
                            (array_len (local_get $alist)))
                    (then (unreachable)))
                (local_set $pair
                    (array_get $alist_str_method
                        (local_get $alist)
                        (local_get $idx)))
                (local_set $key
                    (struct_get $alist_str_method_pair $key
                        (local_get $pair)))
                (local_set $val
                    (struct_get $alist_str_method_pair $val
                        (local_get $pair)))
                (if (call $str_eq
                        (local_get $key)
                        (local_get $name))
                    (then (return (local_get $val))))
                (local_set $idx
                    (i32_add (local_get $idx) (const_i32 1)))
                (br $for))
            (unreachable))
    }
}

fn call() -> Func {
    wat! {
        (func $call
            (param $receiver (ref eq))
            (param $message (ref $str))
            (param $args (ref $arr_unitype))
            (result (ref eq))
            (local $receiver_obj (ref $obj))
            (local $parent (ref $class))
            (local $method (ref $method))

            (local_set $receiver_obj (ref_cast (ref $obj) (local_get $receiver)))
            (local_set $parent
                (ref_as_non_null
                    (struct_get $obj $parent
                        (local_get $receiver_obj))))
            (local_set $method
                (ref_cast (ref $method)
                    (call $alist_str_method_get
                        (struct_get $class $instance_methods
                            (local_get $parent))
                        (local_get $message))))
            (call_ref $method
                (local_get $receiver_obj)
                (local_get $args)
                (local_get $method))
        )
    }
}

fn is_nil() -> Func {
    // Cast to `i31`, then test for the Unitype::FIXNUM_MARKER
    wat! {
        (func $is_nil
            (param $n (ref eq))
            (result i32)

            // Wasm has no short-circuiting booleans.
            (if (result i32)
                (ref_test (ref i31) (local_get $n))
                (then (i32_eq (i31_get_u (ref_cast (ref i31) (local_get $n)))
                              (const_i32 ,(Unitype::NIL_BIT_PATTERN as i64))))
                (else (const_i32 0))))
    }
}

fn is_fixnum() -> Func {
    // Cast to `i31`, then test for the Unitype::FIXNUM_MARKER
    wat! {
        (func $is_fixnum
            (param $n (ref eq))
            (result i32)

            // Wasm has no short-circuiting booleans.
            (if (result i32)
                (ref_test (ref i31) (local_get $n))
                (then (i32_and (const_i32 ,(Unitype::FIXNUM_MARKER as i64))
                               (i31_get_u (ref_cast (ref i31) (local_get $n)))))
                (else (const_i32 0))))
    }
}

fn is_boxnum() -> Func {
    wat! {
        (func $is_boxnum
            (param $n (ref eq))
            (result i32)

            (if (result i32)
                (ref_test (ref $boxnum) (local_get $n))
                (then (const_i32 1))
                (else (const_i32 0))))
    }
}

/// `sign_extend(val: i32, bit_width: i32) -> i32`
/// Sign-extend an `i_(bit_width)` to `i32`.
fn sign_extend() -> Func {
    wat! {
        (func $sign_extend
            (param $val i32)
            (param $bit_width i32)
            (result i32)
            (local $top_bit_mask i32)
            (local $missing_bits_mask i32)

            (local_set $top_bit_mask
                (i32_shl (const_i32 1)
                         (i32_sub (local_get $bit_width) (const_i32 1))))
            (local_set $missing_bits_mask
                (i32_shr_s (i32_shl (const_i32 1) (const_i32 31))
                           (i32_sub (const_i32 32) (local_get $bit_width))))
            (if (result i32)
                (i32_and (local_get $val) (local_get $top_bit_mask))
                (then (i32_or (local_get $val) (local_get $missing_bits_mask)))
                (else (local_get $val))))
    }
}

fn sign_extend_fixnum() -> Func {
    wat! {
        (func $sign_extend_fixnum
            (param $n i32)
            (result i32)

            (call $sign_extend
                  (local_get $n)
                  (const_i32 ,(Unitype::FIXNUM_BIT_WIDTH as i64))))
    }
}

fn fixnum_to_i64() -> Func {
    // - Strip Unitype::FIXNUM_MARKER
    // - Sign-extend i_Unitype::FIXNUM_BIT_WIDTH -> i64

    wat! {
        (func $fixnum_to_i64
            (param $n (ref i31))
            (result i64)
            (local $n_i32 i32)
            (local $n_i32_no_fixnum_marker i32)
            (local $n_i32_sign_extend i32)

            (local_set $n_i32 (i31_get_u (local_get $n)))
            (local_set $n_i32_no_fixnum_marker
                (i32_and (local_get $n_i32)
                         (const_i32 ,(!Unitype::FIXNUM_MARKER as i64))))
            (local_set $n_i32_sign_extend (call $sign_extend_fixnum (local_get $n_i32_no_fixnum_marker)))
            (i64_extend_i32_s (local_get $n_i32_sign_extend))
        )
    }
}

fn boxnum_to_i64() -> Func {
    wat! {
        (func $boxnum_to_i64
            (param $n (ref $boxnum))
            (result i64)
            (struct_get $boxnum $val (local_get $n))
        )
    }
}

fn integer_to_i64() -> Func {
    wat! {
        (func $integer_to_i64
            (param $n (ref eq))
            (result i64)
            (if (result i64)
                (call $is_fixnum (local_get $n))
                (then (call $fixnum_to_i64
                            (ref_cast (ref i31) (local_get $n))))
                (else (call $boxnum_to_i64
                            (ref_cast (ref $boxnum) (local_get $n))))))
    }
}

fn in_fixnum_range() -> Func {
    let min = -(2i64.pow(Unitype::FIXNUM_BIT_WIDTH - 1));
    let max = 2i64.pow(Unitype::FIXNUM_BIT_WIDTH - 1) - 1;

    wat! {
        (func $in_fixnum_range
            (param $n i64)
            (result i32)
            (local $n_i32 i32)
            (local_set $n_i32 (i32_wrap_i64 (local_get $n)))
            (i32_and (i32_lt_s (const_i32 ,(min))
                               (local_get $n_i32))
                     (i32_lt_s (local_get $n_i32)
                               (const_i32 ,(max)))))
    }
}

/// Pre: $n has the bit pattern of a valid fixnum, sans marker.
fn i32_to_fixnum() -> Func {
    wat! {
        (func $i32_to_fixnum
            (param $n i32)
            (result (ref i31))
            (ref_i31 (i32_or (local_get $n)
                             (const_i32 ,(Unitype::FIXNUM_MARKER as i64)))))
    }
}

/// Pre: $n has the bit pattern of a valid fixnum, sans marker.
fn i64_to_fixnum() -> Func {
    wat! {
        (func $i64_to_fixnum
            (param $n i64)
            (result (ref i31))
            (call $i32_to_fixnum (i32_wrap_i64 (local_get $n))))
    }
}

fn i64_to_boxnum() -> Func {
    wat! {
        (func $i64_to_boxnum
            (param $n i64)
            (result (ref $boxnum))
            (struct_new $boxnum (local_get $n)))
    }
}

fn i64_to_integer() -> Func {
    wat! {
        (func $i64_to_integer
            (param $n i64)
            (result (ref eq))
            (if (result (ref eq))
                (call $in_fixnum_range (local_get $n))
                (then (call $i64_to_fixnum (local_get $n)))
                (else (call $i64_to_boxnum (local_get $n)))))
    }
}

fn add() -> Func {
    // TODO: Should do checked add at least.
    wat! {
        (func $add
            (param $lhs (ref eq))
            (param $rhs (ref eq))
            (result (ref eq))
            (local $lhs_val i64)
            (local $rhs_val i64)
            (local $res i64)

            (local_set $lhs_val (call $integer_to_i64 (local_get $lhs)))
            (local_set $rhs_val (call $integer_to_i64 (local_get $rhs)))
            (local_set $res (i64_add (local_get $lhs_val)
                                     (local_get $rhs_val)))
            (call $i64_to_integer (local_get $res))
        )
    }
}

fn to_bool() -> Func {
    wat! {
        (func $to_bool
            (param $b i32)
            (result (ref i31))
            (ref_i31
                (if (result i32)
                    (local_get $b)
                    (then (const_i32 ,(Unitype::TRUE_BIT_PATTERN as i64)))
                    (else (const_i32 ,(Unitype::FALSE_BIT_PATTERN as i64))))))
    }
}

fn from_bool() -> Func {
    wat! {
        (func $from_bool
            (param $b (ref eq))
            (result i32)
            (ref_eq (ref_cast (ref i31) (local_get $b))
                    (ref_i31 (const_i32 ,(Unitype::TRUE_BIT_PATTERN as i64)))))
    }
}

fn negate() -> Func {
    wat! {
        (func $negate
            (param $n (ref eq))
            (result (ref eq))
            (call $i64_to_integer ,(vec![i64_neg(wat![ (call $integer_to_i64 (local_get $n)) ])])))
    }
}

fn and() -> Func {
    wat! {
        (func $and
            (param $a (ref eq))
            (param $b (ref eq))
            (result (ref eq))

            (ref_i31
                (i32_and (i31_get_u (ref_cast (ref i31) (local_get $a)))
                         (i31_get_u (ref_cast (ref i31) (local_get $b))))))
    }
}

fn or() -> Func {
    wat! {
        (func $or
            (param $a (ref eq))
            (param $b (ref eq))
            (result (ref eq))

            (ref_i31
                (i32_or (i31_get_u (ref_cast (ref i31) (local_get $a)))
                        (i31_get_u (ref_cast (ref i31) (local_get $b))))))
    }
}

fn lt() -> Func {
    wat! {
        (func $lt
            (param $a (ref eq))
            (param $b (ref eq))
            (result (ref eq))

            (call $to_bool
                (i64_lt_s (call $integer_to_i64 (local_get $a))
                          (call $integer_to_i64 (local_get $b)))))
    }
}

fn gt() -> Func {
    wat! {
        (func $gt
            (param $a (ref eq))
            (param $b (ref eq))
            (result (ref eq))

            (call $to_bool
                (i64_gt_s (call $integer_to_i64 (local_get $a))
                          (call $integer_to_i64 (local_get $b)))))
    }
}

/// `==` for Integers only.
/// Assumes that all inputs are Integers.
fn eq_eq() -> Func {
    wat! {
        (func $eq_eq
            (param $a (ref eq))
            (param $b (ref eq))
            (result (ref eq))

            (call $to_bool
                (i64_eq (call $integer_to_i64 (local_get $a))
                        (call $integer_to_i64 (local_get $b)))))
    }
}

fn arr_to_js() -> Func {
    let for_loop = corelib::helpers::for_in_arr(
        "arr".to_string(),
        "arr_unitype".to_string(),
        "idx".to_string(),
        "val".to_string(),
        wat! {
            (local_set $val_js
                (call $unitype_to_js (local_get $val)))
            (call $js_arr_push (local_get $arr_js)
                               (local_get $val_js))
        },
    );
    let wat_instrs = [
        wat! { (local_set $idx (ref_i31 (const_i32 ,(Unitype::NIL_BIT_PATTERN as i64)))) },
        wat! { (local_set $val (ref_i31 (const_i32 ,(Unitype::NIL_BIT_PATTERN as i64)))) },
        wat! { (local_set $arr_js (call $js_arr_new)) },
        for_loop,
        wat! { (local_get $arr_js) },
    ]
    .concat();
    wat! {
        (func $arr_to_js
            (param $arr (ref $arr_unitype))
            (result (ref null extern))
            (local $arr_js (ref null extern))
            (local $idx (ref eq))
            (local $val (ref eq))
            (local $val_js (ref null extern))

            ,(wat_instrs)
        )
    }
}

fn unitype_to_js() -> Func {
    wat! {
        (func $unitype_to_js
            (param $x (ref eq))
            (result (ref null extern))
            (if (result (ref null extern))
                (call $is_fixnum (local_get $x))
                (then
                    (call $js_i64_to_ref
                        (call $integer_to_i64 (local_get $x))))
                (else (if (result (ref null extern))
                          (call $is_nil (local_get $x))
                          (then
                              (call $js_i64_to_ref
                                  (const_i64 666)))
                          (else
                              (if (result (ref null extern))
                                  (ref_test (ref $boxnum) (local_get $x))
                                  (then
                                      (call $js_i64_to_ref
                                          (call $integer_to_i64 (local_get $x))))
                                  (else
                                      (if (result (ref null extern))
                                          (ref_test (ref $arr_unitype) (local_get $x))
                                          (then
                                              (call $arr_to_js (ref_cast (ref $arr_unitype) (local_get $x))))
                                          (else (unreachable))))))))))
    }
}
