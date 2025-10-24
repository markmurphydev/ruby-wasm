(rec
  (type $str (sub final (array i8)))
  (type $boxnum (sub final (struct (field $val i64))))
  (type $obj (sub (struct (field $parent (mut (ref null $class))))))
  (type $method
    (sub
      final
      (func
        (param $self (ref $obj))
        (param $args (ref $arr_unitype))
        (result (ref eq)))))
  (type $class
    (sub
      final
      $obj
      (struct
        (field $parent (mut (ref null $class)))
        (field $superclass (mut (ref null $class)))
        (field $name (ref $str))
        (field $instance_methods (ref $alist_str_method)))))
  (type $arr_unitype (sub final (array (mut (ref eq)))))
  (type $alist_str_unitype (sub final (array (ref $alist_str_unitype_pair))))
  (type $alist_str_unitype_pair
    (sub final (struct (field $key (ref $str)) (field $val (ref eq)))))
  (type $alist_str_method (sub final (array (ref $alist_str_method_pair))))
  (type $alist_str_method_pair
    (sub final (struct (field $key (ref $str)) (field $val (ref $method))))))


(func
  $js_arr_new
  (import "arr" "new")
  (result (ref null extern))
  )

(func
  $js_arr_push
  (import "arr" "push")
  (param $arr (ref null extern)) (param $val (ref null extern))
  )

(func
  $js_i64_to_ref
  (import "i64" "toRef")
  (param $x i64)
  (result (ref null extern))
  )


(func
  $fixnum_to_i64
  (param $n (ref i31))
  (result i64)
  (local $n_i32 i32)
  (local $n_i32_no_fixnum_marker i32)
  (local $n_i32_sign_extend i32)
  (local.set $n_i32
    (i31.get_u
      (local.get $n)))
  (local.set $n_i32_no_fixnum_marker
    (i32.and
      (local.get $n_i32)
      (i32.const -1073741825)))
  (local.set $n_i32_sign_extend
    (call $sign_extend_fixnum
      (local.get $n_i32_no_fixnum_marker)))
  (i64.extend_i32_s
    (local.get $n_i32_sign_extend)))

(func
  $sign_extend
  (param $val i32) (param $bit_width i32)
  (result i32)
  (local $top_bit_mask i32) (local $missing_bits_mask i32)
  (local.set $top_bit_mask
    (i32.shl
      (i32.const 1)
      (i32.sub
        (local.get $bit_width)
        (i32.const 1))))
  (local.set $missing_bits_mask
    (i32.shr_s
      (i32.shl
        (i32.const 1)
        (i32.const 31))
      (i32.sub
        (i32.const 32)
        (local.get $bit_width))))
  (if
    (result i32)
    (i32.and
      (local.get $val)
      (local.get $top_bit_mask))
    (then
      (i32.or
        (local.get $val)
        (local.get $missing_bits_mask)))
    (else
      (local.get $val))))

(func
  $sign_extend_fixnum
  (param $n i32)
  (result i32)
  (call $sign_extend
    (local.get $n)
    (i32.const 30)))

(func
  $boxnum_to_i64
  (param $n (ref $boxnum))
  (result i64)
  (struct.get $boxnum $val
    (local.get $n)))

(func
  $integer_to_i64
  (param $n (ref eq))
  (result i64)
  (if
    (result i64)
    (call $is_fixnum
      (local.get $n))
    (then
      (call $fixnum_to_i64
        (ref.cast (ref i31)
          (local.get $n))))
    (else
      (call $boxnum_to_i64
        (ref.cast (ref $boxnum)
          (local.get $n))))))

(func
  $is_fixnum
  (param $n (ref eq))
  (result i32)
  (if
    (result i32)
    (ref.test (ref i31)
      (local.get $n))
    (then
      (i32.and
        (i32.const 1073741824)
        (i31.get_u
          (ref.cast (ref i31)
            (local.get $n)))))
    (else
      (i32.const 0))))

(func
  $arr_to_js
  (param $arr (ref $arr_unitype))
  (result (ref null extern))
  (local $arr_js (ref null extern))
  (local $idx i32)
  (local $val (ref eq))
  (local $val_js (ref null extern))
  (local.set $arr_js
    (call $js_arr_new))
  (local.set $idx
    (i32.const 0))
  (block $exit_for
    (loop $for
      (if
        (i32.eq
          (local.get $idx)
          (array.len
            (local.get $arr)))
        (then (br $exit_for))
        (else
          ))
      (local.set $val
        (array.get $arr_unitype
          (local.get $arr)
          (local.get $idx)))
      (block $body
        (local.set $val_js
          (call $unitype_to_js
            (local.get $val)))
        (call $js_arr_push
          (local.get $arr_js)
          (local.get $val_js)))
      (local.set $idx
        (i32.add
          (local.get $idx)
          (i32.const 1)))
      (br $for)))
  (local.get $arr_js))

(func
  $unitype_to_js
  (param $x (ref eq))
  (result (ref null extern))
  (if
    (result (ref null extern))
    (call $is_fixnum
      (local.get $x))
    (then
      (call $js_i64_to_ref
        (call $integer_to_i64
          (local.get $x))))
    (else
      (if
        (result (ref null extern))
        (ref.test (ref $boxnum)
          (local.get $x))
        (then
          (call $js_i64_to_ref
            (call $integer_to_i64
              (local.get $x))))
        (else
          (if
            (result (ref null extern))
            (ref.test (ref $arr_unitype)
              (local.get $x))
            (then
              (call $arr_to_js
                (ref.cast (ref $arr_unitype)
                  (local.get $x))))
            (else
              (unreachable))))))))

(func
  $is_boxnum
  (param $n (ref eq))
  (result i32)
  (if
    (result i32)
    (ref.test (ref $boxnum)
      (local.get $n))
    (then
      (i32.const 1))
    (else
      (i32.const 0))))

(func
  $i64_to_boxnum
  (param $n i64)
  (result (ref $boxnum))
  (struct.new $boxnum
    (local.get $n)))

(func
  $i32_to_fixnum
  (param $n i32)
  (result (ref i31))
  (ref.i31
    (i32.or
      (local.get $n)
      (i32.const 1073741824))))

(func $fn
  (export "fn")
  (param $n i64)
  (result (ref null extern))
  (call $unitype_to_js
      (array.new_fixed $arr_unitype 1
        (call $i32_to_fixnum (i32.const 22)))))
