
(module
 (rec (type $str (array i8))
  (type $obj
   (sub
    (struct (field $parent (ref null $class))
     (field $superclass (ref null $class)))))
  (type $class
   (sub $obj
    (struct (field $parent (ref null $class))
     (field $superclass (ref null $class)) (field $name (ref $str))
     (field $instance-methods (ref $alist-str-method)))))
  (type $method
   (func (param $self (ref $obj)) (param $args (ref $arr-unitype))
    (result (ref eq))))
  (type $arr-unitype (array (ref eq)))
  (type $alist-str-unitype-pair
   (struct (field $key (ref $str)) (field $val (ref eq))))
  (type $alist-str-unitype (array (ref $alist-str-unitype-pair)))
  (type $alist-str-method-pair
   (struct (field $key (ref $str)) (field $val (ref $method))))
  (type $alist-str-method (array (ref $alist-str-method-pair))))
 (global $false (ref i31) (ref.i31 (i32.const 1)))
 (global $true (ref i31) (ref.i31 (i32.const 3)))
 (global $nil (ref i31) (ref.i31 (i32.const 5)))
 (global $STR-NEW (ref $str)
  (array.new_fixed $str 3 (i32.const 110) (i32.const 101) (i32.const 119)))
 (global $str-BasicObject (ref $str)
  (array.new_fixed $str 11 (i32.const 66) (i32.const 97) (i32.const 115)
   (i32.const 105) (i32.const 99) (i32.const 79) (i32.const 98) (i32.const 106)
   (i32.const 101) (i32.const 99) (i32.const 116)))
 (global $str-Class (ref $str)
  (array.new_fixed $str 5 (i32.const 67) (i32.const 108) (i32.const 97)
   (i32.const 115) (i32.const 115)))
 (global $class-Class (ref $class)
  (struct.new $class (ref.null $class) (ref.null $class)
   (global.get $str-Class)
   (array.new_fixed $alist-str-method 2
    (struct.new $alist-str-method-pair (global.get $STR-NEW)
     (ref.func $method-BasicObject-new))
    (struct.new $alist-str-method-pair (global.get $STR-NEW)
     (ref.func $method-Class-new)))))
 (global $class-BasicObject (ref $class)
  (struct.new $class (global.get $class-Class) (ref.null $class)
   (global.get $str-BasicObject)
   (array.new_fixed $alist-str-method 1
    (struct.new $alist-str-method-pair (global.get $STR-NEW)
     (ref.func $method-Class-new)))))
 (func $method-BasicObject-new (type $method) (param $self (ref $obj))
  (param $args (ref $arr-unitype)) (result (ref eq))
  (struct.new $obj (global.get $class-BasicObject) (ref.null $class)))
 (func $method-Class-new (type $method) (param $self (ref $obj))
  (param $args (ref $arr-unitype)) (result (ref eq))
  (struct.new $obj (global.get $class-Class) (ref.null $class))))