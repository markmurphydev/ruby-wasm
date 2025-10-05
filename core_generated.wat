
(rec (type $str (array i8))
 (type $obj (sub (struct (field $parent (mut (ref null $class))))))
 (type $class
  (sub $obj
   (struct (field $parent (mut (ref null $class)))
    (field $superclass (mut (ref null $class))) (field $name (ref $str))
    (field $instance-methods (ref $ALIST-STR-METHOD)))))
 (type $method
  (func (param $self (ref $obj)) (param $args (ref $arr-unitype))
   (result (ref eq))))
 (type $arr-unitype (array (ref eq)))
 (type $ALIST-STR-UNITYPE (array (ref $ALIST-PAIR-STR-UNITYPE)))
 (type $ALIST-PAIR-STR-UNITYPE
  (struct (field $key (ref $str)) (field $val (ref eq))))
 (type $ALIST-STR-METHOD (array (ref $ALIST-PAIR-STR-METHOD)))
 (type $ALIST-PAIR-STR-METHOD
  (struct (field $key (ref $str)) (field $val (ref eq)))))
(global $false (ref i31) (ref.i31 (i32.const 1)))
(global $true (ref i31) (ref.i31 (i32.const 3)))
(global $nil (ref i31) (ref.i31 (i32.const 5)))
(global $empty-args (ref $arr-unitype) (array.new_fixed $arr-unitype 0))
(global $STR-CLASS (ref $str)
 (array.new_fixed $str 5 (i32.const 99) (i32.const 108) (i32.const 97)
  (i32.const 115) (i32.const 115)))
(global $STR-NEW (ref $str)
 (array.new_fixed $str 3 (i32.const 110) (i32.const 101) (i32.const 119)))
(global $str-Object (ref $str)
 (array.new_fixed $str 6 (i32.const 79) (i32.const 98) (i32.const 106)
  (i32.const 101) (i32.const 99) (i32.const 116)))
(global $str-BasicObject (ref $str)
 (array.new_fixed $str 11 (i32.const 66) (i32.const 97) (i32.const 115)
  (i32.const 105) (i32.const 99) (i32.const 79) (i32.const 98) (i32.const 106)
  (i32.const 101) (i32.const 99) (i32.const 116)))
(global $str-Module (ref $str)
 (array.new_fixed $str 6 (i32.const 77) (i32.const 111) (i32.const 100)
  (i32.const 117) (i32.const 108) (i32.const 101)))
(global $str-Class (ref $str)
 (array.new_fixed $str 5 (i32.const 67) (i32.const 108) (i32.const 97)
  (i32.const 115) (i32.const 115)))
(global $class-Class (ref $class)
 (struct.new $class (ref.null $class) (ref.null $class) (global.get $str-Class)
  (array.new_fixed $ALIST-STR-METHOD 1
   (struct.new $ALIST-PAIR-STR-METHOD (global.get $STR-NEW)
    (ref.func $method-Class-new)))))
(global $class-Module (ref $class)
 (struct.new $class (ref.null $class) (ref.null $class)
  (global.get $str-Module) (array.new_fixed $ALIST-STR-METHOD 0)))
(global $class-BasicObject (ref $class)
 (struct.new $class (ref.null $class) (ref.null $class)
  (global.get $str-BasicObject) (array.new_fixed $ALIST-STR-METHOD 0)))
(global $class-Object (ref $class)
 (struct.new $class (ref.null $class) (ref.null $class)
  (global.get $str-Object) (array.new_fixed $ALIST-STR-METHOD 0)))
(func $method-Class-new (type $method) (param $self (ref $obj))
 (param $args (ref $arr-unitype)) (result (ref eq))
 (struct.new $obj (ref.cast (ref $class) (local.get $self))))
(func $str-eq (param $a (ref $str)) (param $b (ref $str)) (result i32)
 (local $idx i32) (local $a_ch i32) (local $b_ch i32)
 (local.set $idx (i32.const 0))
 (if (i32.eqz (i32.eq (array.len (local.get $a)) (array.len (local.get $b))))
     (then (return (i32.const 0)))
     (else
      (loop $for (if (i32.eq (local.get $idx) (array.len (local.get $a)))
                     (then (return (i32.const 1)))) (local.set $a_ch
                                                     (array.get_u $str
                                                      (local.get $a)
                                                      (local.get
                                                       $idx))) (local.set $b_ch
                                                                (array.get_u
                                                                 $str
                                                                 (local.get $b)
                                                                 (local.get
                                                                  $idx))) (if (i32.eqz
                                                                               (i32.eq
                                                                                (local.get
                                                                                 $a_ch)
                                                                                (local.get
                                                                                 $b_ch)))
                                                                              (then
                                                                               (return
                                                                                (i32.const
                                                                                 0)))) (local.set
                                                                                        $idx
                                                                                        (i32.add
                                                                                         (local.get
                                                                                          $idx)
                                                                                         (i32.const
                                                                                          1))) (br
                                                                                                $for))))
 (unreachable))
(func $alist-str-method-get (param $alist (ref $ALIST-STR-METHOD))
 (param $name (ref $str)) (result (ref $method)) (local $idx i32)
 (local $pair (ref $ALIST-PAIR-STR-METHOD)) (local $key (ref $str))
 (local $val (ref $method)) (local.set $idx (i32.const 0))
 (local.set $idx (i32.const 0))
 (loop $for (if (i32.eq (local.get $idx) (array.len (local.get $alist)))
                (then (unreachable))) (local.set $pair
                                       (array.get $ALIST-STR-METHOD
                                        (local.get $alist)
                                        (local.get $idx))) (local.set $key
                                                            (struct.get
                                                             $ALIST-PAIR-STR-METHOD
                                                             $key
                                                             (local.get
                                                              $pair))) (local.set
                                                                        $val
                                                                        (struct.get
                                                                         $ALIST-PAIR-STR-METHOD
                                                                         $val
                                                                         (local.get
                                                                          $pair))) (if (call
                                                                                        $str-eq
                                                                                        (local.get
                                                                                         $key)
                                                                                        (local.get
                                                                                         $name))
                                                                                       (then
                                                                                        (return
                                                                                         (local.get
                                                                                          $val)))) (local.set
                                                                                                    $idx
                                                                                                    (i32.add
                                                                                                     (local.get
                                                                                                      $idx)
                                                                                                     (i32.const
                                                                                                      1))) (br
                                                                                                            $for))
 (unreachable))
(func $call (param $receiver (ref $obj)) (param $message (ref $str))
 (param $args (ref $arr-unitype)) (result (ref eq))
 (local $parent (ref $class)) (local $method (ref $method))
 (local.set $parent
  (ref.as_non_null (struct.get $obj $parent (local.get $receiver))))
 (local.set $method
  (ref.cast (ref $method)
   (call $alist-str-method-get
    (struct.get $class $instance-methods (local.get $parent))
    (local.get $message))))
 (call_ref $method (local.get $receiver) (local.get $args) (local.get $method)))
(func $start
 (struct.set $class $parent (global.get $class-Class)
  (global.get $class-Class))
 (struct.set $class $parent (global.get $class-Module)
  (global.get $class-Class))
 (struct.set $class $parent (global.get $class-BasicObject)
  (global.get $class-Class))
 (struct.set $class $parent (global.get $class-Object)
  (global.get $class-Class))
 (struct.set $class $superclass (global.get $class-Class)
  (global.get $class-Module))
 (struct.set $class $superclass (global.get $class-Module)
  (global.get $class-Object))
 (struct.set $class $superclass (global.get $class-Object)
  (global.get $class-BasicObject)))
(start $start)