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
(func
  $__ruby_top_level_function
  (export "__ruby_top_level_function")
  (result (ref eq))
  (i32.const 5)
  (ref.i31))
