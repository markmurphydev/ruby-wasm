(func $arr_new (import "arr" "new") (result (ref null extern)))
(func $arr_push (import "arr" "push") (param (ref null extern)) (param i32))

(func $fn
  (export "fn")
  (param $n i32)
  (result (ref null extern))
  (local $arr (ref null extern))

  (local.set $arr (call $arr_new))
  (call $arr_push (local.get $arr) (i32.add (local.get $n) (i32.const 1)))
  (call $arr_push (local.get $arr) (i32.add (local.get $n) (i32.const 2)))
  (call $arr_push (local.get $arr) (i32.add (local.get $n) (i32.const 3)))
  (local.get $arr))
