(rec
  (type $str (sub final (array i8)))
  (type $obj (sub (struct (field $parent (mut (ref null $class))))))
  (type $class
    (sub
      final
      $obj
      (struct
        (field $parent (mut (ref null $class)))
        (field $superclass (mut (ref null $class)))
        (field $name (ref $str))
        (field $instance-methods (ref $alist-str-method)))))
  (type $method
    (sub
      final
      (func
        (param $self (ref $obj))
        (param $args (ref $arr-unitype))
        (result (ref eq)))))
  (type $arr-unitype (sub final (array (ref eq))))
  (type $alist-str-unitype (sub final (array (ref $alist-str-unitype-pair))))
  (type $alist-str-unitype-pair
    (sub (struct (field $key (ref $str)) (field $val (ref eq)))))
  (type $alist-str-method (sub final (array (ref $alist-str-method-pair))))
  (type $alist-str-method-pair
    (sub (struct (field $key (ref $str)) (field $val (ref $method))))))
(global $str-Module
  (ref $str)
  (i32.const 77)
  (i32.const 111)
  (i32.const 100)
  (i32.const 117)
  (i32.const 108)
  (i32.const 101)
  (array.new_fixed $str 6))
(global $str-Class
  (ref $str)
  (i32.const 67)
  (i32.const 108)
  (i32.const 97)
  (i32.const 115)
  (i32.const 115)
  (array.new_fixed $str 5))
(global $str-BasicObject
  (ref $str)
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
  (i32.const 116)
  (array.new_fixed $str 11))
(global $str-Object
  (ref $str)
  (i32.const 79)
  (i32.const 98)
  (i32.const 106)
  (i32.const 101)
  (i32.const 99)
  (i32.const 116)
  (array.new_fixed $str 6))
(global $str-new
  (ref $str)
  (i32.const 110)
  (i32.const 101)
  (i32.const 119)
  (array.new_fixed $str 3))
(global $str-class
  (ref $str)
  (i32.const 99)
  (i32.const 108)
  (i32.const 97)
  (i32.const 115)
  (i32.const 115)
  (array.new_fixed $str 5))
(global $class-Module
  (ref $class)
  (ref.null $class)
  (ref.null $class)
  (global.get $str-Module)
  (array.new_fixed $alist-str-method 0)
  (struct.new $class))
(global $class-Class
  (ref $class)
  (ref.null $class)
  (ref.null $class)
  (global.get $str-Class)
  (array.new_fixed $alist-str-method 0)
  (struct.new $class))
(global $class-BasicObject
  (ref $class)
  (ref.null $class)
  (ref.null $class)
  (global.get $str-BasicObject)
  (array.new_fixed $alist-str-method 0)
  (struct.new $class))
(global $class-Object
  (ref $class)
  (ref.null $class)
  (ref.null $class)
  (global.get $str-Object)
  (array.new_fixed $alist-str-method 0)
  (struct.new $class))
(func
  $method-new
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr-unitype))
  (result (ref eq))
  (local.get $self)
  (ref.cast (ref $class))
  (struct.new $obj))
(func
  $method-class
  (type $method)
  (param $self (ref $obj)) (param $args (ref $arr-unitype))
  (result (ref eq))
  (local.get $self)
  (struct.get $obj $parent)
  (ref.cast (ref eq)))
(func
  $start
  
  (global.get $class-Module)
  (global.get $class-Class)
  (struct.set $class $parent)
  (global.get $class-Module)
  (global.get $class-Object)
  (struct.set $class $superclass)
  (global.get $class-Class)
  (global.get $class-Class)
  (struct.set $class $parent)
  (global.get $class-Class)
  (global.get $class-Module)
  (struct.set $class $superclass)
  (global.get $class-BasicObject)
  (global.get $class-Class)
  (struct.set $class $parent)
  (global.get $class-Object)
  (global.get $class-Class)
  (struct.set $class $parent)
  (global.get $class-Object)
  (global.get $class-BasicObject)
  (struct.set $class $superclass))
(func
  $str-eq
  (param $a (ref $str)) (param $b (ref $str))
  (result i32)
  (local $idx i32) (local $a_ch i32) (local $b_ch i32)
  (i32.const 0)
  (local.set $idx)
  (if
    (local.get $a)
    (array.len)
    (local.get $b)
    (array.len)
    (i32.eq)
    (then
      (i32.const 0)
      (return))
    (else
      ))
  (loop $for (result (ref eq))
    (if
      (local.get $idx)
      (local.get $a)
      (array.len)
      (i32.eq)
      (i32.eqz)
      (then
        (i32.const 1)
        (return))
      (else
        ))
    (local.get $a)
    (local.get $idx)
    (array.get_u $str)
    (local.set $a_ch)
    (local.get $b)
    (local.get $idx)
    (array.get_u $str)
    (local.set $b_ch)
    (if
      (local.get $a_ch)
      (local.get $b_ch)
      (i32.eq)
      (i32.eqz)
      (then
        (i32.const 0)
        (return))
      (else
        ))
    (local.get $idx)
    (i32.const 1)
    (i32.add)
    (local.set $idx)
    (br $for))
  (unreachable))
(func
  $__ruby_top_level_function
  (export "__ruby_top_level_function")
  (result (ref eq))
  (i32.const 5)
  (ref.i31))
(start $start)
