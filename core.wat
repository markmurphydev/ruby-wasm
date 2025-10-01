(type $str (array i8))
(func $str-eq
      (param $a (ref $str))
      (param $b (ref $str))
      (result i32)
      (local $idx i32)
      (local $a_ch i32)
      (local $b_ch i32)

      ;; if (a.len != b.len) { return false }
      ;; for (a_ch, b_ch) in (a, b) {
      ;;    if (a_ch != b_ch) { return false }
      ;; }
      ;; return true
      (if (result i32)
        (i32.eqz (i32.eq (array.len (local.get $a))
                     (array.len (local.get $b))))
        (then (i32.const 0))
        (else 
          (loop $for (result i32)
            (local.set $a_ch (array.get_u $str (local.get $a) (local.get $idx)))
            (local.set $b_ch (array.get_u $str (local.get $b) (local.get $idx)))
            (if (result i32)
              (i32.eqz (i32.eq (local.get $a_ch) (local.get $b_ch)))
              (then (return (i32.const 0)))
              (else
                ;; idx += 1;
                ;; if (idx == a.len) { return true }
                (local.set $idx (i32.add (local.get $idx)
                                         (i32.const 1)))
                (br_if $for (i32.eq (local.get $idx)
                                    (array.len (local.get $a)))
                       (i32.const 1))))))))

(type $unitype-alist-pair (struct (field $key (ref $str)) (field $val (ref eq))))
(type $unitype-alist (array (ref $unitype-alist-pair)))

(type $obj 
      (struct (field $class (ref null $obj))
              (field $fields (ref $unitype-alist))))

;; "22"
(global $test-string (ref $str)
    (array.new_fixed $str 2
                     (i32.const 50)
                     (i32.const 50)))

;; classes["22"] = Object.new(class: "22", {"22": "22"})
(global $class-BasicObject (ref $obj)
        (struct.new $obj
                    ;; $class
                    (ref.null $obj)
                    ;; $fields
                    (array.new_fixed $unitype-alist 1
                                     ;; ["22"] -> "22"
                                     (struct.new $unitype-alist-pair
                                                 ;; $key
                                                 (global.get $test-string)
                                                 ;; $val
                                                 (global.get $test-string)))))


(type $class-alist-pair (struct (field $key (ref $str)) (field $val (ref $obj))))
(type $class-alist (array (ref $class-alist-pair)))
;; AList<String, &Object>
(global $classes (ref $class-alist)
        (array.new_fixed $class-alist 1
                         (struct.new $class-alist-pair
                                     (global.get $test-string)
                                     (global.get $class-BasicObject))))
(func $classes-get
      (param $name (ref $str))
      (result (ref $obj))
      (local $idx i32)
      (local $pair (ref $class-alist-pair))
      (local $key (ref $str))
      (local $val (ref $obj))

      ;; for (key, class) in $classes {
      ;;    if (key == $name) { return class }
      ;; }
      ;; (error)
      (local.set $idx (i32.const 0))
      (loop $for
        (local.set $pair 
                   (array.get $class-alist
                              (global.get $classes)
                              (local.get $idx)))
        (local.set $key
                   (struct.get $class-alist-pair $key
                               (local.get $pair)))
        (local.set $val
                   (struct.get $class-alist-pair $val
                               (local.get $pair)))
        (if
          (call $str-eq (local.get $key) 
                        (local.get $name))
          (then (return (local.get $val)))
          (else
            (local.set $idx (i32.add (local.get $idx)
                                     (i32.const 1)))
            (br_if $for (i32.eq (local.get $idx) (array.len (global.get $classes)))))))
      (unreachable))

(global $false (ref i31)
        (ref.i31 (i32.const 1)))

(global $true (ref i31)
        (ref.i31 (i32.const 3)))


(func $call
      (param $receiver (ref eq))
      (param $message (ref $str))
      (result i32)
      (local $class-name (ref $str))
      (local $class (ref $obj))
      ;; If the receiver is a primitive, find its associated class
      (local.set $class-name
         (if (result (ref $str))
           (ref.test (ref i31) (local.get $receiver))
           (then (global.get $test-string))
           (else
             ;; It's an obj
             (struct.get $obj $class
                         (ref.cast (ref $obj) (local.get $receiver))))))
      (local.set $class (call $classes-get (local.get $class-name)))
      ;; TODO -- get the method from the class, call
      ;; TODO -- traverse class.ancestors()
      (ref.test (ref i31) (local.get $receiver)))



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
      (global.get $test-string))

