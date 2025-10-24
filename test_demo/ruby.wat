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
  $js_i64_to_ref
  (import "i64" "toRef")
  (param $x i64)
  (result (ref null extern))
  )
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
  $get_cells_export
  (export "get_cells")
  (result (ref null extern))
  (call $unitype_to_js
    (call $method_Object_get_cells
      (global.get $main)
      (array.new_fixed $arr_unitype 0))))
(func
  $toggle_cell_export
  (export "toggle_cell")
  (param $row i32) (param $col i32)
  (result (ref null extern))
  (call $unitype_to_js
    (call $method_Object_toggle_cell
      (global.get $main)
      (array.new_fixed $arr_unitype 2
        (call $i32_to_fixnum
          (local.get $row))
        (call $i32_to_fixnum
          (local.get $col))))))
(func
  $count_neighbors_export
  (export "count_neighbors")
  (param $row i32) (param $col i32)
  (result (ref null extern))
  (call $unitype_to_js
    (call $method_Object_count_neighbors
      (global.get $main)
      (array.new_fixed $arr_unitype 2
        (call $i32_to_fixnum
          (local.get $row))
        (call $i32_to_fixnum
          (local.get $col))))))
(func
  $step_export
  (export "step")
  (result (ref null extern))
  (call $unitype_to_js
    (call $method_Object_step
      (global.get $main)
      (array.new_fixed $arr_unitype 0))))
(func
  $__ruby_top_level_function
  (export "__ruby_top_level_function")
  (result (ref eq))
  (global.set $cells
    (array.new_fixed $arr_unitype 10
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))))
  (ref.i31
    (i32.const 5))
  (drop)
  (ref.i31
    (i32.const 5))
  (drop)
  (ref.i31
    (i32.const 5))
  (drop)
  (ref.i31
    (i32.const 5))
  (drop)
  (ref.i31
    (i32.const 5)))
(func
  $method_Object_get_cells
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (global.get $cells))
(func
  $method_Object_toggle_cell
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (local $row (ref eq)) (local $col (ref eq)) (local $res (ref eq))
  (local.set $row
    (array.get $arr_unitype
      (local.get $args)
      (i32.const 0)))
  (local.set $col
    (array.get $arr_unitype
      (local.get $args)
      (i32.const 1)))
  (local.set $res
    (ref.i31
      (i32.const 5)))
  (if
    (result (ref eq))
    (call $from_bool
      (call $eq_eq
        (array.get $arr_unitype
          (ref.cast (ref $arr_unitype)
            (array.get $arr_unitype
              (ref.cast (ref $arr_unitype)
                (global.get $cells))
              (i32.wrap_i64
                (call $integer_to_i64
                  (local.get $row)))))
          (i32.wrap_i64
            (call $integer_to_i64
              (local.get $col))))
        (ref.i31
          (i32.const 1073741825))))
    (then
      (local.set $res
        (ref.i31
          (i32.const 1073741824)))
      (ref.i31
        (i32.const 5)))
    (else
      (local.set $res
        (ref.i31
          (i32.const 1073741825)))
      (ref.i31
        (i32.const 5))))
  (drop)
  (array.set $arr_unitype
    (ref.cast (ref $arr_unitype)
      (array.get $arr_unitype
        (ref.cast (ref $arr_unitype)
          (global.get $cells))
        (i32.wrap_i64
          (call $integer_to_i64
            (local.get $row)))))
    (i32.wrap_i64
      (call $integer_to_i64
        (local.get $col)))
    (local.get $res))
  (ref.i31
    (i32.const 5)))
(func
  $method_Object_count_neighbors
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (local $row (ref eq)) (local $col (ref eq))
  (local.set $row
    (array.get $arr_unitype
      (local.get $args)
      (i32.const 0)))
  (local.set $col
    (array.get $arr_unitype
      (local.get $args)
      (i32.const 1)))
  (ref.i31
    (i32.const 1073741827)))
(func
  $method_Object_step
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (local $res (ref eq))
  (local $row (ref eq))
  (local $d0eaa337-7bfc-4824-95ee-1b8f60d751d5 (ref eq))
  (local $f204f64a-38b2-4518-9efe-07b8356106a8 (ref eq))
  (local $col (ref eq))
  (local $390d9825-537e-4d2b-8f50-ad30221c8d54 (ref eq))
  (local $06f0def0-034d-4fa5-95ad-dcd8d583994a (ref eq))
  (local $alive (ref eq))
  (local $living_neighbors (ref eq))
  (local $res_alive (ref eq))
  (local.set $res
    (ref.i31
      (i32.const 5)))
  (local.set $row
    (ref.i31
      (i32.const 5)))
  (local.set $d0eaa337-7bfc-4824-95ee-1b8f60d751d5
    (ref.i31
      (i32.const 5)))
  (local.set $f204f64a-38b2-4518-9efe-07b8356106a8
    (ref.i31
      (i32.const 5)))
  (local.set $col
    (ref.i31
      (i32.const 5)))
  (local.set $390d9825-537e-4d2b-8f50-ad30221c8d54
    (ref.i31
      (i32.const 5)))
  (local.set $06f0def0-034d-4fa5-95ad-dcd8d583994a
    (ref.i31
      (i32.const 5)))
  (local.set $alive
    (ref.i31
      (i32.const 5)))
  (local.set $living_neighbors
    (ref.i31
      (i32.const 5)))
  (local.set $res_alive
    (ref.i31
      (i32.const 5)))
  (local.set $res
    (array.new_fixed $arr_unitype 10
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))
      (array.new_fixed $arr_unitype 10
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824))
        (ref.i31
          (i32.const 1073741824)))))
  (ref.i31
    (i32.const 5))
  (drop)
  (local.set $d0eaa337-7bfc-4824-95ee-1b8f60d751d5
    (array.new_fixed $arr_unitype 10
      (ref.i31
        (i32.const 1073741824))
      (ref.i31
        (i32.const 1073741825))
      (ref.i31
        (i32.const 1073741826))
      (ref.i31
        (i32.const 1073741827))
      (ref.i31
        (i32.const 1073741828))
      (ref.i31
        (i32.const 1073741829))
      (ref.i31
        (i32.const 1073741830))
      (ref.i31
        (i32.const 1073741831))
      (ref.i31
        (i32.const 1073741832))
      (ref.i31
        (i32.const 1073741833))))
  (local.set $f204f64a-38b2-4518-9efe-07b8356106a8
    (call $i64_to_integer
      (i64.const 0)))
  (block $exit_for
    (loop $for
      (if
        (i32.eq
          (i32.wrap_i64
            (call $integer_to_i64
              (local.get $f204f64a-38b2-4518-9efe-07b8356106a8)))
          (array.len
            (ref.cast (ref $arr_unitype)
              (local.get $d0eaa337-7bfc-4824-95ee-1b8f60d751d5))))
        (then
          (br $exit_for))
        (else
          ))
      (local.set $row
        (array.get $arr_unitype
          (ref.cast (ref $arr_unitype)
            (local.get $d0eaa337-7bfc-4824-95ee-1b8f60d751d5))
          (i32.wrap_i64
            (call $integer_to_i64
              (local.get $f204f64a-38b2-4518-9efe-07b8356106a8)))))
      (local.set $390d9825-537e-4d2b-8f50-ad30221c8d54
        (array.new_fixed $arr_unitype 10
          (ref.i31
            (i32.const 1073741824))
          (ref.i31
            (i32.const 1073741825))
          (ref.i31
            (i32.const 1073741826))
          (ref.i31
            (i32.const 1073741827))
          (ref.i31
            (i32.const 1073741828))
          (ref.i31
            (i32.const 1073741829))
          (ref.i31
            (i32.const 1073741830))
          (ref.i31
            (i32.const 1073741831))
          (ref.i31
            (i32.const 1073741832))
          (ref.i31
            (i32.const 1073741833))))
      (local.set $06f0def0-034d-4fa5-95ad-dcd8d583994a
        (call $i64_to_integer
          (i64.const 0)))
      (block $exit_for
        (loop $for
          (if
            (i32.eq
              (i32.wrap_i64
                (call $integer_to_i64
                  (local.get $06f0def0-034d-4fa5-95ad-dcd8d583994a)))
              (array.len
                (ref.cast (ref $arr_unitype)
                  (local.get $390d9825-537e-4d2b-8f50-ad30221c8d54))))
            (then
              (br $exit_for))
            (else
              ))
          (local.set $col
            (array.get $arr_unitype
              (ref.cast (ref $arr_unitype)
                (local.get $390d9825-537e-4d2b-8f50-ad30221c8d54))
              (i32.wrap_i64
                (call $integer_to_i64
                  (local.get $06f0def0-034d-4fa5-95ad-dcd8d583994a)))))
          (local.set $alive
            (call $eq_eq
              (array.get $arr_unitype
                (ref.cast (ref $arr_unitype)
                  (array.get $arr_unitype
                    (ref.cast (ref $arr_unitype)
                      (global.get $cells))
                    (i32.wrap_i64
                      (call $integer_to_i64
                        (local.get $row)))))
                (i32.wrap_i64
                  (call $integer_to_i64
                    (local.get $col))))
              (ref.i31
                (i32.const 1073741825))))
          (ref.i31
            (i32.const 5))
          (drop)
          (local.set $living_neighbors
            (call $call
              (global.get $main)
              (global.get $str_count_neighbors)
              (array.new_fixed $arr_unitype 2
                (local.get $row)
                (local.get $col))))
          (ref.i31
            (i32.const 5))
          (drop)
          (local.set $res_alive
            (call $or
              (call $eq_eq
                (local.get $living_neighbors)
                (ref.i31
                  (i32.const 1073741827)))
              (call $and
                (local.get $alive)
                (call $eq_eq
                  (local.get $living_neighbors)
                  (ref.i31
                    (i32.const 1073741826))))))
          (ref.i31
            (i32.const 5))
          (drop)
          (if
            (result (ref eq))
            (call $from_bool
              (local.get $res_alive))
            (then
              (array.set $arr_unitype
                (ref.cast (ref $arr_unitype)
                  (array.get $arr_unitype
                    (ref.cast (ref $arr_unitype)
                      (local.get $res))
                    (i32.wrap_i64
                      (call $integer_to_i64
                        (local.get $row)))))
                (i32.wrap_i64
                  (call $integer_to_i64
                    (local.get $col)))
                (ref.i31
                  (i32.const 1073741825)))
              (ref.i31
                (i32.const 5)))
            (else
              (array.set $arr_unitype
                (ref.cast (ref $arr_unitype)
                  (array.get $arr_unitype
                    (ref.cast (ref $arr_unitype)
                      (local.get $res))
                    (i32.wrap_i64
                      (call $integer_to_i64
                        (local.get $row)))))
                (i32.wrap_i64
                  (call $integer_to_i64
                    (local.get $col)))
                (ref.i31
                  (i32.const 1073741824)))
              (ref.i31
                (i32.const 5))))
          (local.set $06f0def0-034d-4fa5-95ad-dcd8d583994a
            (call $i64_to_integer
              (i64.add
                (call $integer_to_i64
                  (local.get $06f0def0-034d-4fa5-95ad-dcd8d583994a))
                (i64.const 1))))
          (br $for)))
      (ref.i31
        (i32.const 5))
      (local.set $f204f64a-38b2-4518-9efe-07b8356106a8
        (call $i64_to_integer
          (i64.add
            (call $integer_to_i64
              (local.get $f204f64a-38b2-4518-9efe-07b8356106a8))
            (i64.const 1))))
      (br $for)))
  (ref.i31
    (i32.const 5))
  (drop)
  (global.set $cells
    (local.get $res))
  (ref.i31
    (i32.const 5)))
(func
  $method_Class_new
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (struct.new $obj
    (ref.cast (ref $class)
      (local.get $self))))
(func
  $method_Object_class
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (ref.cast (ref eq)
    (struct.get $obj $parent
      (local.get $self))))
(func
  $method_Class_name
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (struct.get $class $name
    (ref.cast (ref $class)
      (local.get $self))))
(func
  $_start
  
  (struct.set $class $parent
    (global.get $class_Module)
    (global.get $class_Class))
  (struct.set $class $superclass
    (global.get $class_Module)
    (global.get $class_Object))
  (struct.set $class $parent
    (global.get $class_Class)
    (global.get $class_Class))
  (struct.set $class $superclass
    (global.get $class_Class)
    (global.get $class_Module))
  (struct.set $class $parent
    (global.get $class_BasicObject)
    (global.get $class_Class))
  (struct.set $class $parent
    (global.get $class_Object)
    (global.get $class_Class))
  (struct.set $class $superclass
    (global.get $class_Object)
    (global.get $class_BasicObject))
  (global.set $main
    (ref.cast (ref $obj)
      (call $method_Class_new
        (global.get $class_Object)
        (global.get $empty_args)))))
(func
  $str_eq
  (param $a (ref $str)) (param $b (ref $str))
  (result i32)
  (local $idx i32) (local $a_ch i32) (local $b_ch i32)
  (local.set $idx
    (i32.const 0))
  (if
    (i32.eqz
      (i32.eq
        (array.len
          (local.get $a))
        (array.len
          (local.get $b))))
    (then
      (return
        (i32.const 0)))
    (else
      ))
  (loop $for(result (ref eq))
    (if
      (i32.eq
        (local.get $idx)
        (array.len
          (local.get $a)))
      (then
        (return
          (i32.const 1)))
      (else
        ))
    (local.set $a_ch
      (array.get_u $str
        (local.get $a)
        (local.get $idx)))
    (local.set $b_ch
      (array.get_u $str
        (local.get $b)
        (local.get $idx)))
    (if
      (i32.eqz
        (i32.eq
          (local.get $a_ch)
          (local.get $b_ch)))
      (then
        (return
          (i32.const 0)))
      (else
        ))
    (local.set $idx
      (i32.add
        (local.get $idx)
        (i32.const 1)))
    (br $for))
  (unreachable))
(func
  $alist_str_method_get
  (param $alist (ref $alist_str_method)) (param $name (ref $str))
  (result (ref $method))
  (local $idx i32)
  (local $pair (ref $alist_str_method_pair))
  (local $key (ref $str))
  (local $val (ref $method))
  (loop $for(result (ref eq))
    (if
      (i32.eq
        (local.get $idx)
        (array.len
          (local.get $alist)))
      (then
        (unreachable))
      (else
        ))
    (local.set $pair
      (array.get $alist_str_method
        (local.get $alist)
        (local.get $idx)))
    (local.set $key
      (struct.get $alist_str_method_pair $key
        (local.get $pair)))
    (local.set $val
      (struct.get $alist_str_method_pair $val
        (local.get $pair)))
    (if
      (call $str_eq
        (local.get $key)
        (local.get $name))
      (then
        (return
          (local.get $val)))
      (else
        ))
    (local.set $idx
      (i32.add
        (local.get $idx)
        (i32.const 1)))
    (br $for))
  (unreachable))
(func
  $call
  (param $receiver (ref eq))
  (param $message (ref $str))
  (param $args (ref $arr_unitype))
  (result (ref eq))
  (local $receiver_obj (ref $obj))
  (local $parent (ref $class))
  (local $method (ref $method))
  (local.set $receiver_obj
    (ref.cast (ref $obj)
      (local.get $receiver)))
  (local.set $parent
    (ref.as_non_null
      (struct.get $obj $parent
        (local.get $receiver_obj))))
  (local.set $method
    (ref.cast (ref $method)
      (call $alist_str_method_get
        (struct.get $class $instance_methods
          (local.get $parent))
        (local.get $message))))
  (call_ref $method
    (local.get $receiver_obj)
    (local.get $args)
    (local.get $method)))
(func
  $is_nil
  (param $n (ref eq))
  (result i32)
  (if
    (result i32)
    (ref.test (ref i31)
      (local.get $n))
    (then
      (i32.eq
        (i31.get_u
          (ref.cast (ref i31)
            (local.get $n)))
        (i32.const 5)))
    (else
      (i32.const 0))))
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
  $in_fixnum_range
  (param $n i64)
  (result i32)
  (local $n_i32 i32)
  (local.set $n_i32
    (i32.wrap_i64
      (local.get $n)))
  (i32.and
    (i32.lt_s
      (i32.const -536870912)
      (local.get $n_i32))
    (i32.lt_s
      (local.get $n_i32)
      (i32.const 536870911))))
(func
  $i32_to_fixnum
  (param $n i32)
  (result (ref i31))
  (ref.i31
    (i32.or
      (local.get $n)
      (i32.const 1073741824))))
(func
  $i64_to_fixnum
  (param $n i64)
  (result (ref i31))
  (call $i32_to_fixnum
    (i32.wrap_i64
      (local.get $n))))
(func
  $i64_to_boxnum
  (param $n i64)
  (result (ref $boxnum))
  (struct.new $boxnum
    (local.get $n)))
(func
  $i64_to_integer
  (param $n i64)
  (result (ref eq))
  (if
    (result (ref eq))
    (call $in_fixnum_range
      (local.get $n))
    (then
      (call $i64_to_fixnum
        (local.get $n)))
    (else
      (call $i64_to_boxnum
        (local.get $n)))))
(func
  $add
  (param $lhs (ref eq)) (param $rhs (ref eq))
  (result (ref eq))
  (local $lhs_val i64) (local $rhs_val i64) (local $res i64)
  (local.set $lhs_val
    (call $integer_to_i64
      (local.get $lhs)))
  (local.set $rhs_val
    (call $integer_to_i64
      (local.get $rhs)))
  (local.set $res
    (i64.add
      (local.get $lhs_val)
      (local.get $rhs_val)))
  (call $i64_to_integer
    (local.get $res)))
(func
  $to_bool
  (param $b i32)
  (result (ref i31))
  (ref.i31
    (if
      (result i32)
      (local.get $b)
      (then
        (i32.const 3))
      (else
        (i32.const 1)))))
(func
  $from_bool
  (param $b (ref eq))
  (result i32)
  (ref.eq
    (ref.cast (ref i31)
      (local.get $b))
    (ref.i31
      (i32.const 3))))
(func
  $negate
  (param $n (ref eq))
  (result (ref eq))
  (call $i64_to_integer
    (i64.add
      (i64.const 1)
      (i64.xor
        (i64.const -1)
        (call $integer_to_i64
          (local.get $n))))))
(func
  $and
  (param $a (ref eq)) (param $b (ref eq))
  (result (ref eq))
  (ref.i31
    (i32.and
      (i31.get_u
        (ref.cast (ref i31)
          (local.get $a)))
      (i31.get_u
        (ref.cast (ref i31)
          (local.get $b))))))
(func
  $or
  (param $a (ref eq)) (param $b (ref eq))
  (result (ref eq))
  (ref.i31
    (i32.or
      (i31.get_u
        (ref.cast (ref i31)
          (local.get $a)))
      (i31.get_u
        (ref.cast (ref i31)
          (local.get $b))))))
(func
  $lt
  (param $a (ref eq)) (param $b (ref eq))
  (result (ref eq))
  (call $to_bool
    (i64.lt_s
      (call $integer_to_i64
        (local.get $a))
      (call $integer_to_i64
        (local.get $b)))))
(func
  $gt
  (param $a (ref eq)) (param $b (ref eq))
  (result (ref eq))
  (call $to_bool
    (i64.gt_s
      (call $integer_to_i64
        (local.get $a))
      (call $integer_to_i64
        (local.get $b)))))
(func
  $eq_eq
  (param $a (ref eq)) (param $b (ref eq))
  (result (ref eq))
  (call $to_bool
    (i64.eq
      (call $integer_to_i64
        (local.get $a))
      (call $integer_to_i64
        (local.get $b)))))
(func
  $arr_to_js
  (param $arr (ref $arr_unitype))
  (result (ref null extern))
  (local $arr_js (ref null extern))
  (local $idx (ref eq))
  (local $val (ref eq))
  (local $val_js (ref null extern))
  (local.set $idx
    (ref.i31
      (i32.const 5)))
  (local.set $val
    (ref.i31
      (i32.const 5)))
  (local.set $arr_js
    (call $js_arr_new))
  (local.set $idx
    (call $i64_to_integer
      (i64.const 0)))
  (block $exit_for
    (loop $for
      (if
        (i32.eq
          (i32.wrap_i64
            (call $integer_to_i64
              (local.get $idx)))
          (array.len
            (ref.cast (ref $arr_unitype)
              (local.get $arr))))
        (then
          (br $exit_for))
        (else
          ))
      (local.set $val
        (array.get $arr_unitype
          (ref.cast (ref $arr_unitype)
            (local.get $arr))
          (i32.wrap_i64
            (call $integer_to_i64
              (local.get $idx)))))
      (local.set $val_js
        (call $unitype_to_js
          (local.get $val)))
      (call $js_arr_push
        (local.get $arr_js)
        (local.get $val_js))
      (local.set $idx
        (call $i64_to_integer
          (i64.add
            (call $integer_to_i64
              (local.get $idx))
            (i64.const 1))))
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
        (call $is_nil
          (local.get $x))
        (then
          (call $js_i64_to_ref
            (i64.const 666)))
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
                  (unreachable))))))))))
(global $cells
  (mut (ref eq))
  (ref.i31
    (i32.const 5)))
(global $main
  (mut (ref $obj))
  (struct.new $obj
    (ref.null $class)))
(global $empty_args
  (ref $arr_unitype)
  (array.new_fixed $arr_unitype 0))
(global $str_Module
  (ref $str)
  (array.new_fixed $str 6
    (i32.const 77)
    (i32.const 111)
    (i32.const 100)
    (i32.const 117)
    (i32.const 108)
    (i32.const 101)))
(global $str_Class
  (ref $str)
  (array.new_fixed $str 5
    (i32.const 67)
    (i32.const 108)
    (i32.const 97)
    (i32.const 115)
    (i32.const 115)))
(global $str_BasicObject
  (ref $str)
  (array.new_fixed $str 11
    (i32.const 66)
    (i32.const 97)
    (i32.const 115)
    (i32.const 105)
    (i32.const 99)
    (i32.const 79)
    (i32.const 98)
    (i32.const 106)
    (i32.const 101)
    (i32.const 99)
    (i32.const 116)))
(global $str_Object
  (ref $str)
  (array.new_fixed $str 6
    (i32.const 79)
    (i32.const 98)
    (i32.const 106)
    (i32.const 101)
    (i32.const 99)
    (i32.const 116)))
(global $str_get_cells
  (ref $str)
  (array.new_fixed $str 9
    (i32.const 103)
    (i32.const 101)
    (i32.const 116)
    (i32.const 95)
    (i32.const 99)
    (i32.const 101)
    (i32.const 108)
    (i32.const 108)
    (i32.const 115)))
(global $str_toggle_cell
  (ref $str)
  (array.new_fixed $str 11
    (i32.const 116)
    (i32.const 111)
    (i32.const 103)
    (i32.const 103)
    (i32.const 108)
    (i32.const 101)
    (i32.const 95)
    (i32.const 99)
    (i32.const 101)
    (i32.const 108)
    (i32.const 108)))
(global $str_count_neighbors
  (ref $str)
  (array.new_fixed $str 15
    (i32.const 99)
    (i32.const 111)
    (i32.const 117)
    (i32.const 110)
    (i32.const 116)
    (i32.const 95)
    (i32.const 110)
    (i32.const 101)
    (i32.const 105)
    (i32.const 103)
    (i32.const 104)
    (i32.const 98)
    (i32.const 111)
    (i32.const 114)
    (i32.const 115)))
(global $str_step
  (ref $str)
  (array.new_fixed $str 4
    (i32.const 115)
    (i32.const 116)
    (i32.const 101)
    (i32.const 112)))
(global $str_new
  (ref $str)
  (array.new_fixed $str 3
    (i32.const 110)
    (i32.const 101)
    (i32.const 119)))
(global $str_class
  (ref $str)
  (array.new_fixed $str 5
    (i32.const 99)
    (i32.const 108)
    (i32.const 97)
    (i32.const 115)
    (i32.const 115)))
(global $str_name
  (ref $str)
  (array.new_fixed $str 4
    (i32.const 110)
    (i32.const 97)
    (i32.const 109)
    (i32.const 101)))
(global $class_Module
  (ref $class)
  (struct.new $class
    (ref.null $class)
    (ref.null $class)
    (global.get $str_Module)
    (array.new_fixed $alist_str_method 0)))
(global $class_Class
  (ref $class)
  (struct.new $class
    (ref.null $class)
    (ref.null $class)
    (global.get $str_Class)
    (array.new_fixed $alist_str_method 2
      (struct.new $alist_str_method_pair
        (global.get $str_new)
        (ref.func $method_Class_new))
      (struct.new $alist_str_method_pair
        (global.get $str_name)
        (ref.func $method_Class_name)))))
(global $class_BasicObject
  (ref $class)
  (struct.new $class
    (ref.null $class)
    (ref.null $class)
    (global.get $str_BasicObject)
    (array.new_fixed $alist_str_method 0)))
(global $class_Object
  (ref $class)
  (struct.new $class
    (ref.null $class)
    (ref.null $class)
    (global.get $str_Object)
    (array.new_fixed $alist_str_method 5
      (struct.new $alist_str_method_pair
        (global.get $str_get_cells)
        (ref.func $method_Object_get_cells))
      (struct.new $alist_str_method_pair
        (global.get $str_toggle_cell)
        (ref.func $method_Object_toggle_cell))
      (struct.new $alist_str_method_pair
        (global.get $str_count_neighbors)
        (ref.func $method_Object_count_neighbors))
      (struct.new $alist_str_method_pair
        (global.get $str_step)
        (ref.func $method_Object_step))
      (struct.new $alist_str_method_pair
        (global.get $str_class)
        (ref.func $method_Object_class)))))
(start $_start)
