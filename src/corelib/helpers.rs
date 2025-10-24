use crate::corelib::alist::AListTypeDef;
use wat_defs::instr::Instr;
use wat_defs::ty::ArrayType;
use wat_macro::wat;

/// Requires locals $arr, $idx, $val.
/// Binds arr values to $val.
pub fn for_in_arr(arr_name: String, arr_type_name: String, body: Vec<Instr>) -> Vec<Instr> {
    wat! {
        (local_set $idx (const_i32 0))
        (block $exit_for
            (loop $for
                (if (i32_eq (local_get $idx)
                            (array_len (local_get ,(arr_name.clone()))))
                    (then (br $exit_for)))
                (local_set $val
                    (array_get ,(arr_type_name)
                        (local_get ,(arr_name))
                        (local_get $idx)))
                (block $body ,(body))
                (local_set $idx (i32_add (local_get $idx)
                                       (const_i32 1)))
                (br $for)))
    }
}

pub fn for_in_alist(alist_type_def: AListTypeDef, body: Vec<Instr>) -> Vec<Instr> {
    let alist_identifier = alist_type_def.alist_type_identifier();
    let pair_identifier = alist_type_def.alist_pair_type_identifier();

    wat! {
        (local_set $idx (const_i32 0))
        (loop $for
            (if (i32_eq (local_get $idx)
                        (array_len (local_get $alist)))
                (then (unreachable)))
            (local_set $pair
                (array_get ,(alist_identifier)
                    (local_get $alist)
                    (local_get $idx)))
            (local_set $key
                (struct_get ,(pair_identifier.clone()) $key
                    (local_get $pair)))
            (local_set $val
                (struct_get ,(pair_identifier.clone()) $val
                    (local_get $pair)))
            (block $body ,(body))
            (local_set $idx (i32_add (local_get $idx)
                                   (const_i32 1)))
            (br $for))
    }
}

/// `(i32.not x) ≡ (i32.xor x -1)`
///     (because `-1 = 0b1111_...`)
pub fn i32_not(mut body: Vec<Instr>) -> Instr {
    let wat_args = {
        let mut res = wat![(const_i32, (-1))];
        res.append(&mut body);
        res
    };
    wat! {
        (i32_xor ,(wat_args))
    }
    .remove(0)
}

/// 2's comp negation of `i32`:
/// `(+ 1 (not n))`
pub fn i32_neg(body: Vec<Instr>) -> Instr {
    wat! {
        (i32_add (const_i32 1) ,(i32_not(body)))
    }
    .remove(0)
}

/// `(i32.not x) ≡ (i32.xor x -1)`
///     (because `-1 = 0b1111_...`)
pub fn i64_not(mut body: Vec<Instr>) -> Instr {
    let wat_args = {
        let mut res = wat![(const_i64, (-1))];
        res.append(&mut body);
        res
    };
    wat! {
        (i64_xor ,(wat_args))
    }
    .remove(0)
}

/// 2's comp negation of `i32`:
/// `(+ 1 (not n))`
pub fn i64_neg(body: Vec<Instr>) -> Instr {
    wat! {
        (i64_add (const_i64 1) ,(i64_not(body)))
    }
    .remove(0)
}
