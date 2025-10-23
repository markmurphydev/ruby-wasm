(global $x (mut i32) (i32.const 0))

(func $add (export "add") (param $a i32) (param $b i32) (result i32)
    (global.set $x (local.get $a))
    (global.get $x))