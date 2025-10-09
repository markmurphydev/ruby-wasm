
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
(func
  $__ruby_top_level_function
  (export "__ruby_top_level_function")
  (result (ref eq))
  (i32.const 5)
  (ref.i31))
