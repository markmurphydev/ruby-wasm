;; ==== types ====
(type $str (array i8))

;; We say that if ($class == null), then it's a self-parented class, like BasicObject
(rec
  (type $obj 
        (struct (field $class (ref null $obj))
                ;; The methods of _children_ of this object. Only applies to classes?
                (field $instance_methods (ref $alist-str-method))))

  (type $method (func (param $self (ref $obj))
                      (param $args (ref $alist-str-unitype))
                      (result (ref eq))))

  (type $alist-str-unitype-pair (struct (field $key (ref $str)) (field $val (ref eq))))
  (type $alist-str-unitype (array (ref $alist-str-unitype-pair)))

  (type $alist-str-method-pair (struct (field $key (ref $str)) (field $val (ref $method))))
  (type $alist-str-method (array (ref $alist-str-method-pair))))


;; ==== globals ====

(global $false (ref i31)
        (ref.i31 (i32.const 1)))

(global $true (ref i31)
        (ref.i31 (i32.const 3)))

;; "!"
(global $const-str-! (ref $str)
        (array.new_fixed $str 1 (i32.const 33)))

;; class BasicObject < BasicObject {
;;     def !:
;;        False
;;     end
;; }
(global $class-BasicObject (ref $obj)
        (struct.new $obj
                    ;; $class
                    (ref.null $obj)
                    ;; $instance_methods
                    (array.new_fixed $alist-str-method 1
                                     (struct.new $alist-str-method-pair
                                                 (global.get $const-str-!)
                                                 (ref.func $method-BasicObject-!)))))

;; test instance of BasicObject
(global $instance-BasicObject (ref $obj)
        (struct.new $obj
                    ;; $class
                    (global.get $class-BasicObject)
                    ;; $instance_methods
                    (array.new_fixed $alist-str-method 0)))


;; ==== funcs ====

(func $method-BasicObject-! (type $method)
      (global.get $false))

(func $str-eq
      (param $a (ref $str))
      (param $b (ref $str))
      (result i32)
      (local $idx i32)
      (local $a_ch i32)
      (local $b_ch i32)

      ;; if (a.len != b.len) { return false }
      ;; for (a_ch, b_ch) in zip(a, b) {
      ;;    if (a_ch != b_ch) { return false }
      ;; }
      ;; return true
      (local.set $idx (i32.const 0))
      (if
        (i32.eqz (i32.eq (array.len (local.get $a))
                         (array.len (local.get $b))))
        (then (return (i32.const 0)))
        (else 
          (loop $for
            (if (i32.eq (local.get $idx) 
                        (array.len (local.get $a)))
              (then (return (i32.const 1))))
            (local.set $a_ch (array.get_u $str (local.get $a) (local.get $idx)))
            (local.set $b_ch (array.get_u $str (local.get $b) (local.get $idx)))
            (if (i32.eqz (i32.eq (local.get $a_ch) 
                                 (local.get $b_ch)))
              (then (return (i32.const 0))))
            ;; idx += 1;
            ;; if (idx == a.len) { return true }
            (local.set $idx (i32.add (local.get $idx)
                                     (i32.const 1)))
            (br $for))))
      (unreachable))

(func $alist-str-method-get
      (param $alist (ref $alist-str-method))
      (param $name (ref $str))
      (result (ref $method))

      (local $idx i32)
      (local $pair (ref $alist-str-method-pair))
      (local $key (ref $str))
      (local $val (ref $method))

      ;; for (key, method) in $alist {
      ;;    if (key == $name) { return method }
      ;; }
      ;; (error)
      (local.set $idx (i32.const 0))
      (loop $for
        (if (i32.eq (local.get $idx)
                    (array.len (local.get $alist)))
          (then (unreachable)))
        (local.set $pair
                   (array.get $alist-str-method
                              (local.get $alist)
                              (local.get $idx)))
        (local.set $key
                   (struct.get $alist-str-method-pair $key
                               (local.get $pair)))
        (local.set $val
                   (struct.get $alist-str-method-pair $val
                               (local.get $pair)))
        (if
          (call $str-eq (local.get $key)
                        (local.get $name))
          (then (return (local.get $val)))
          (else
            (local.set $idx (i32.add (local.get $idx)
                                     (i32.const 1)))
            (br $for))))
      (unreachable))



;; If receiver is:
;; - $obj -> get obj.class
;; - primitive -> TODO: Get associated class (Integer, String, ...)
;; Then, find method on class whose name matches $message
;;  TODO: Should traverse all ancestors
(func $call
      (param $receiver (ref $obj))
      (param $message (ref $str))
      (param $args (ref $alist-str-unitype))
      (result (ref eq))

      (local $class (ref $obj))
      (local $method (ref $method))

      (local.set $class (ref.as_non_null (struct.get $obj $class (local.get $receiver))))
      ;; TODO -- get the method from the class, call
      (local.set $method
                 (ref.cast (ref $method)
                           (call $alist-str-method-get
                                 (struct.get $obj
                                             $instance_methods
                                             (local.get $class))
                                 (local.get $message))))
      (call_ref $method
                (local.get $receiver)
                (local.get $args)
                (local.get $method)))


(func $to_ruby_bool
      (param $b i32)
      (result (ref eq))
      (if (result (ref eq))
        (local.get $b)
        (then (ref.i31 (i32.const 3)))
        (else (ref.i31 (i32.const 1)))))

(func $is_str 
      (param $maybe_str (ref eq))
      (result (ref eq))
      (call $to_ruby_bool
            (ref.test (ref $str)
                      (local.get $maybe_str))))

(func $is_false
      (param $b (ref eq))
      (result i32)
      (if (result i32)
        (ref.test (ref i31) (local.get $b))
        (then (i32.eq (i31.get_u (ref.cast (ref i31) (local.get $b)))
                      (i31.get_u (global.get $false))))
        (else (i32.const 0))))

(func $top (export "__ruby_top_level_function")
      (result (ref eq))
      (call $call (global.get $instance-BasicObject)
            (global.get $const-str-!)
            (array.new_fixed $alist-str-unitype 0)))

