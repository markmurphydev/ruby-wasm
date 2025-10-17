(rec
  (type $str (sub final (array i8)))
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
  (type $arr_unitype (sub final (array (ref eq))))
  (type $alist_str_unitype (sub final (array (ref $alist_str_unitype_pair))))
  (type $alist_str_unitype_pair
    (sub final (struct (field $key (ref $str)) (field $val (ref eq)))))
  (type $alist_str_method (sub final (array (ref $alist_str_method_pair))))
  (type $alist_str_method_pair
    (sub final (struct (field $key (ref $str)) (field $val (ref $method))))))
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
    (array.new_fixed $alist_str_method 3
      (struct.new $alist_str_method_pair
        (global.get $str_class)
        (ref.func $method_class))
      (struct.new $alist_str_method_pair
        (global.get $str_new)
        (ref.func $method_new))
      (struct.new $alist_str_method_pair
        (global.get $str_name)
        (ref.func $method_name)))))
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
    (array.new_fixed $alist_str_method 0)))
(func
  $method_new
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (struct.new $obj
    (ref.cast(ref $class)
      (local.get $self))))
(func
  $method_class
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (local.get $self)
  (struct.get $obj $parent)
  (ref.cast(ref eq)))
(func
  $method_name
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr_unitype))
  (result (ref eq))
  (struct.get $class $name
    (ref.cast(ref $class)
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
    (global.get $class_BasicObject)))
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
    (ref.cast(ref $obj)
      (local.get $receiver)))
  (local.set $parent
    (ref.as_non_null
      (struct.get $obj $parent
        (local.get $receiver_obj))))
  (local.set $method
    (ref.cast(ref $method)
      (call $alist_str_method_get
        (struct.get $class $instance_methods
          (local.get $parent))
        (local.get $message))))
  (call_ref $method
    (local.get $receiver_obj)
    (local.get $args)
    (local.get $method)))
(func
  $__ruby_top_level_function
  (export "__ruby_top_level_function")
  (result (ref eq))
  (call $call
    (global.get $class_BasicObject)
    (global.get $str_name)
    (global.get $empty_args)))
(start $_start)
