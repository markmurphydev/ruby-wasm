use crate::corelib::alist::AListTypeDef;
use wat_defs::instr::Instr;
use wat_macro::wat;

pub fn helper_for_in(alist_type_def: AListTypeDef, body: Vec<Instr>) -> Vec<Instr> {
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
        let mut res = wat![(const_i32 ,(-1))];
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
    }.remove(0)
}

/// `(i32.not x) ≡ (i32.xor x -1)`
///     (because `-1 = 0b1111_...`)
pub fn i64_not(mut body: Vec<Instr>) -> Instr {
    let wat_args = {
        let mut res = wat![(const_i64 ,(-1))];
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
    }.remove(0)
}
