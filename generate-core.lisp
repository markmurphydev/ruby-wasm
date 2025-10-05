;; To get case preserved in the quasi-quotes
;; It ~mostly~ works. Sometimes it capitalizes things...
(setf (readtable-case *readtable*) :invert)
(ql:quickload '(:uiop))
(defpackage :generate-core
  (:use :cl))
(in-package :generate-core)

;;;; Consts
(defconstant false-bit-pattern #b0001)
(defconstant true-bit-pattern #b0011)
(defconstant nil-bit-pattern #b0101)
;; We give fixnums half an i31, marking MSB 1
;; (0b1xx_xxxx...): i31
(defconstant fixnum-bit-width 30)
(defconstant fixnum-mask (- (expt 2 31) 1))

(defclass ruby-method ()
  ((name :initarg :name :accessor name)
   (fn-compile :initarg :fn-compile :accessor fn-compile)))

(defclass ruby-class ()
  ((fn-parent
    :initarg :fn-parent
    :accessor fn-parent)
   (fn-superclass
    :initarg :fn-superclass
    :accessor fn-superclass)
   (child-superclass
    :initarg :child-superclass
    :accessor child-superclass)
   (name
    :initarg :name
    :accessor name)
   (fn-instance-methods
    :initarg :fn-instance-methods
    :accessor fn-instance-methods)))

;; Classes need to mutually reference each other.
;; But if they were all functions, they'd infinitely recurse generating new classes.
;; We use parameter definitions and a lambda for late binding.

(defparameter *class-class* nil)
(defparameter *class-module* nil)
(defparameter *class-basic-object* nil)
(defparameter *class-object* nil)
(defun classes () 
  (list *class-class*  *class-module* *class-basic-object* *class-object*))

(defun methods ()
  (list (method-new) (method-class)))

;;;; ruby-class definitions
(defparameter *class-class*
      (make-instance 'ruby-class
                     :fn-parent (lambda () *class-class*)
                     :fn-superclass (lambda () *class-module*)
                     :child-superclass nil
                     :name "Class"
                     :fn-instance-methods 
                     (lambda () (list (method-new)))))

(setf *class-module*
      (make-instance 'ruby-class
                     :fn-parent (lambda () *class-class*)
                     :fn-superclass (lambda () *class-object*)
                     :child-superclass nil
                     :name "Module"
                     :fn-instance-methods (lambda () (list))))

(setf *class-basic-object*
      (make-instance 'ruby-class
                     :fn-parent (lambda () *class-class*)
                     :fn-superclass (lambda () nil)
                     :child-superclass nil
                     :name "BasicObject"
                     ;; equal?, !, __send__, ==, __id__, instance_eval, instance_exec
                     :fn-instance-methods (lambda () (list))))

(setf *class-object* 
      (make-instance 'ruby-class
                     :fn-parent (lambda () *class-class*)
                     :fn-superclass (lambda () *class-basic-object*)
                     :child-superclass nil
                     :name "Object" 
                     :fn-instance-methods (lambda () (list))))

;;;; Ruby method definitions
(defun compile-method-name (method-name class-name)
  (intern (format nil "$method-~a-~a" class-name method-name)))

(defun compile-method (method-name class-name body)
  (let ((name (compile-method-name method-name class-name)))
    `(func ,name (type $method)
           (param $self (ref $obj))
           (param $args (ref $arr-unitype))
           (result (ref eq))
           ,body)))

(defun compile-method-new (class-name)
  (compile-method "new" class-name
                  `(struct.new $obj
                               ;; $parent
                               (ref.cast (ref $class) (local.get $self)))))

(defun method-new ()
  (make-instance 'ruby-method
                 :name "new"
                 :fn-compile 'compile-method-new))

(defun compile-method-class (class-name)
  (compile-method "class" class-name
                  `(struct.get $obj $parent (local.get $self))))

(defun method-class ()
  (make-instance 'ruby-method
                 :name "class"
                 :fn-compile 'compile-method-class))

;;;; Compilation functions

(defun compile-string-name (str)
  (intern (format nil "$str-~a" str)))

(defun compile-string (str)
  (let* ((consts (mapcar (lambda (c) `(i32.const ,(char-int c)))
                         (coerce str 'list))))
    `(global ,(compile-string-name str) (ref $str) (array.new_fixed $str ,(length str) ,@consts))))

(defun compile-class-name (name)
  (intern (format nil "$class-~a" name)))

(defun compile-ruby-class (class)
  ;; You can't define mutually-referential globals.
  ;; But you can initialize to nil then set everything in _start.
  (let* ((compiled-class-name (compile-class-name (name class)))
         (name-expr `(global.get ,(compile-string-name (name class))))
         (methods (funcall (fn-instance-methods class)))
         (methods-arr-elems (mapcar (lambda (method)
                                      (let ((compiled-method-name (compile-method-name (name method) (name class))))
                                        `(struct.new ,(alist-pair-type (alist-str-method))
                                                     (global.get ,(compile-string-name (name method)))
                                                     (ref.func ,compiled-method-name))))
                                    methods))
         (instance-methods-expr
           `(array.new_fixed ,(alist-type (alist-str-method)) 
                             ,(length methods)
                             ,@methods-arr-elems)))
    `(global ,compiled-class-name (ref $class)
             (struct.new $class
                         ;; $parent
                         (ref.null $class)
                         ;; $superclass
                         (ref.null $class)
                         ,name-expr
                         ,instance-methods-expr))))
(compile-ruby-class *class-class*)

(defun set-class-parents ()
  "The section of code in _start that initializes classes' parents"
  (flet ((set-parents (class)
           (let* ((parent (funcall (fn-parent class)))
                  (compiled-class-name (compile-class-name (name class)))
                  (compiled-parent-name (compile-class-name (name parent))))
             `(struct.set $class $parent
                          (global.get ,compiled-class-name)
                          (global.get ,compiled-parent-name)))))
    (mapcar #'set-parents (classes))))
(set-class-parents)

(defun set-class-superclasses ()
  ;; (superclass (funcall (fn-superclass class)))
  ;; (superclass-expr (if superclass
  ;; `(global.get ,(compile-class-name (name superclass)))
  ;; '(ref.null $class)))
  (flet ((set-superclass (class)
           (let ((superclass (funcall (fn-superclass class))))
             (when superclass 
               (let ((compiled-class-name (compile-class-name (name class)))
                     (compiled-superclass-name (compile-class-name (name superclass))))
                 `(struct.set $class $superclass
                              (global.get ,compiled-class-name)
                              (global.get ,compiled-superclass-name)))))))
    (remove-if-not (lambda (x) x)
                   (mapcar #'set-superclass (classes)))))
(set-class-superclasses)

;;;; Collect item definitions

(defun string-defs ()
  (let* ((strings-set (list))
         (class-names
           (mapcar (lambda (class) (name class))
                   (classes)))
         (method-names
           (mapcar (lambda (method) (name method)) (methods)))
         (names (append class-names method-names)))
    (dolist (name names)
      (pushnew name strings-set :test 'equal))
    (mapcar 'compile-string strings-set)))
(string-defs)

(defun class-defs ()
  (mapcar 'compile-ruby-class (classes)))
(class-defs)

(defun method-defs ()
  (mapcan 
   (lambda (class)
     (mapcar 
      (lambda (method) 
        (funcall (fn-compile method) (name class)))
      (funcall (fn-instance-methods class))))
   (classes)))
(method-defs)

;;;; Alists

(defstruct wasm-alist key val)

(defun alist-type (alist)
  (intern (format nil "$alist-~a-~a" (wasm-alist-key alist) (wasm-alist-val alist))))
(defun alist-pair-type (alist)
  (intern (format nil "$alist-pair-~a-~a" (wasm-alist-key alist) (wasm-alist-val alist))))
(defun alist-type-def (alist)
  `(type ,(alist-type alist) (array (ref ,(alist-pair-type alist)))))
(defun alist-pair-type-def (alist)
  (let ((val (if (string= (wasm-alist-val) "unitype")
  `(type ,(alist-pair-type alist) (struct (field $key (ref $str)) (field $val (ref eq)))))
    
(defun alist-str-unitype () (make-wasm-alist :key "str" :val "unitype"))
(defun alist-str-method () (make-wasm-alist :key "str" :val "method"))

(defun for-in (&key (idx '$idx)
                    (pair '$pair)
                    (key '$key)
                    (val '$val)
                    (alist '$alist)
                    alist-type
                    body)
  (let ((alist-type (alist-type alist-type))
        (alist-pair-type (alist-pair-type alist-type)))
    `((local.set ,idx (i32.const 0))
      (loop $for
               (if (i32.eq (local.get ,idx)
                           (array.len (local.get ,alist)))
                   (then (unreachable)))
               (local.set ,pair
                          (array.get ,alist-type
                                     (local.get ,alist)
                                     (local.get ,idx)))
               (local.set ,key
                          (struct.get ,alist-pair-type $key
                                      (local.get ,pair)))
               (local.set ,val
                          (struct.get ,alist-pair-type $val
                                      (local.get ,pair)))
               ,body
               (local.set $idx (i32.add (local.get $idx)
                                        (i32.const 1)))
               (br $for))
      (unreachable))))
    

(defun module ()
  `(;;;; Types
    (rec
     (type $str (array i8))
     (type $obj (sub (struct (field $parent (mut (ref null $class))))))
     (type $class (sub $obj 
                       (struct (field $parent 
                                      (mut (ref null $class)))
                               (field $superclass
                                      (mut (ref null $class)))
                               (field $name 
                                      (ref $str))
                               (field $instance-methods 
                                      (ref ,(alist-type (alist-str-method)))))))
     (type $method (func (param $self (ref $obj))
                         (param $args (ref $arr-unitype))
                         (result (ref eq))))
     (type $arr-unitype (array (ref eq)))

     ,(alist-type-def (alist-str-unitype))
     ,(alist-pair-type-def (alist-str-unitype))

     ,(alist-type-def (alist-str-method))
     ,(alist-pair-type-def (alist-str-method)))

    ;;;; Globals
    ;;; Consts
    (global $false (ref i31)
            (ref.i31 (i32.const ,false-bit-pattern)))

    (global $true (ref i31)
            (ref.i31 (i32.const ,true-bit-pattern)))
    
    (global $nil (ref i31)
            (ref.i31 (i32.const ,nil-bit-pattern)))

    (global $empty-args (ref $arr-unitype)
            (array.new_fixed $arr-unitype 0))
    
    
    ;;; Strings
    ,@(string-defs)

    ;;; Class instances
    ,@(class-defs)
    
    ;;;; Functions
    ,@(method-defs)
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
          (param $alist (ref ,(alist-type (alist-str-method))))
          (param $name (ref $str))
          (result (ref $method))

          (local $idx i32)
          (local $pair (ref ,(alist-pair-type (alist-str-method))))
          (local $key (ref $str))
          (local $val (ref $method))

          ;; for (key, method) in $alist {
          ;;    if (key == $name) { return method }
          ;; }
          ;; (error)
          (local.set $idx (i32.const 0))
          ,@(for-in :alist-type (alist-str-method)
                    :body
                    `(if
                      (call $str-eq (local.get $key)
                            (local.get $name))
                      (then (return (local.get $val))))))
    
    (func $call
          (param $receiver (ref $obj))
          (param $message (ref $str))
          (param $args (ref $arr-unitype))
          (result (ref eq))

          (local $parent (ref $class))
          (local $method (ref $method))

          (local.set $parent (ref.as_non_null (struct.get $obj $parent (local.get $receiver))))
          ;; TODO -- get the method from the class, call
          (local.set $method
                     (ref.cast (ref $method)
                               (call $alist-str-method-get
                                     (struct.get $class
                                                 $instance-methods
                                                 (local.get $parent))
                                     (local.get $message))))
          (call_ref $method
                    (local.get $receiver)
                    (local.get $args)
                    (local.get $method)))
    
    (func $start
          ,@(set-class-parents)
          ,@(set-class-superclasses))
    (start $start)
    ;; (func
     ;; $__ruby_top_level_function
     ;; (export "__ruby_top_level_function")
     ;; (result (ref eq))
     ;; (global.get $class-Object)
     ;; (global.get $STR-NEW)
     ;; (global.get $empty-args)
     ;; (call $call)
     ;; (global.get $STR-CLASS)
     ;; (globstartal.get $str-Class-name)
     ;; (global.get $empty-args)
     ;; (call $call)
     ;; (ref.cast (ref $class)))))
    ))

    

(with-open-file (f "./core_generated.wat"
                   :direction :output
                   :if-exists :supersede
                   :if-does-not-exist :create)
  (dolist (item (module))
    (pprint item f)))

(uiop:run-program "wasmtime -W gc=y -W function-references=y core_generated.wat"
                  :output t
                  :error-output t)