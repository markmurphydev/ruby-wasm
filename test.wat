(type $arr (array (mut i32)))
(global $arr (mut (ref $arr)) (array.new_fixed $arr 2 (i32.const 0) (i32.const 1)))

(func $_start
    (export "__ruby_top_level_function")
    (local $new (ref $arr))
    (local.set $new (array.new $arr (i32.const 0) (i32.const 3)))
    (array.copy $arr $arr
        (local.get $new)
        (i32.const 0)
        (global.get $arr)
        (i32.const 0)
        (array.len (global.get $arr)))
    (array.set $arr
        (local.get $new)
        (i32.const 2)
        (i32.const 2))
    (global.set $arr (local.get $new)))