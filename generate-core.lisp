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
   (owner :initarg :owner :accessor owner)))

(defclass ruby-method-new (ruby-method) ())

(defclass ruby-class ()
  ((parent
    :initarg :parent
    :accessor parent)
   (superclass
    :initarg :superclass
    :accessor superclass)
   (child-superclass
    :initarg :child-superclass
    :accessor child-superclass)
   (name
    :initarg :name
    :accessor name)
   (instance-methods
    :initarg :instance-methods
    :accessor instance-methods)))

(defparameter *class-class*
  (make-instance 'ruby-class
                 :parent nil
                 :superclass nil
                 :child-superclass nil ; But should be Object
                 :name "Class"
                 :instance-methods (list *method-basic-object-new*)))

(defparameter *method-class-new* 
  (make-instance 'ruby-method-new :name "new" :owner *class-class*))

(defparameter *class-basic-object*
  (make-instance 'ruby-class
                 :parent *class-class*
                 :superclass nil ; But should be Module.
                 :child-superclass nil
                 :name "BasicObject" 
                 :instance-methods (list *method-class-new*)))

(defparameter *method-basic-object-new* 
  (make-instance 'ruby-method-new :name "new" :owner *class-basic-object*))

(defgeneric symbolic-name (item)
  (:method ((method ruby-method))
    (intern (format nil "$method-~a-~a" (name (owner method)) (name method))))
  (:method ((class ruby-class))
    (intern (format nil "$class-~a" (name class))))
  (:method ((str string))
    (intern (format nil "$str-~a" str)))
  (:method ((item null))
    '(ref null)))
(symbolic-name *method-class-new*)
(symbolic-name *class-class*)
(symbolic-name "==")
(symbolic-name nil)

(defgeneric compile-ruby-method (method)
  (:method ((method ruby-method-new))
    (let* ((symbol (symbolic-name method))
           (owner (owner method))
           (owner-symbol (symbolic-name owner))
           (child-superclass (child-superclass owner))
           (child-superclass-symbol
             (if child-superclass
                 (symbolic-name child-superclass)
                 '(ref.null $class))))
      `(func ,symbol (type $method)
             (param $self (ref $obj))
             (param $args (ref $arr-unitype))
             (result (ref eq))
             (struct.new $obj
                         (global.get ,owner-symbol)
                         ,child-superclass-symbol)))) )
(compile-ruby-method *method-class-new*)

(defgeneric compile-ruby-class (class)
  (:method ((class ruby-class))
    (let* ((class-name (symbolic-name class))
          (parent-expr (if (parent class)
                           `(global.get ,(symbolic-name (parent class)))
                           '(ref.null $class)))
          (superclass-expr (if (superclass class)
                               `(global.get ,(symbolic-name (superclass class)))
                               '(ref.null $class)))
          (name-expr `(global.get ,(symbolic-name (name class))))
          (methods (instance-methods class))
          (methods-arr-elems (mapcar (lambda (method)
                                       `(struct.new $alist-str-method-pair
                                                    (global.get ,(symbolic-name (name method)))
                                                    (ref.func ,(symbolic-name method))))
                                     methods))
          (instance-methods-expr
            `(array.new_fixed $alist-str-method 
                              ,(length (instance-methods class))
                              ,@methods-arr-elems)))
      `(global ,class-name (ref $class)
               (struct.new $class
                           ,parent-expr
                           ,superclass-expr
                           ,name-expr
                           ,instance-methods-expr)))))
(compile-ruby-class *class-class*)


(defun compile-string (str)
  (let* ((consts (mapcar (lambda (c) `(i32.const ,(char-int c)))
                         (coerce str 'list))))
    `(global ,(symbolic-name str) (ref $str) (array.new_fixed $str ,(length str) ,@consts))))

;; Have to get all the named objects together and make _one_ (global $str) definition for each.
(defparameter classes (list *class-class* *class-basic-object*))
(defparameter methods (mapcan (lambda (class) (instance-methods class)) classes))

(defparameter string-defs
  (let* ((strings-set (list))
         (class-names
           (mapcar (lambda (class) (name class))
                   classes))
         (method-names
           (mapcar (lambda (method) (name method)) methods))
         (names (append class-names method-names)))
    (dolist (name names)
      (pushnew name strings-set :test 'equal))
    (mapcar 'compile-string strings-set)))

(defparameter class-defs
  (mapcar 'compile-ruby-class classes))

(defparameter method-defs
  (mapcar 'compile-ruby-method methods))
  
(defparameter *module*
  `(module
    ;;;; Types
    (rec
     (type $str (array i8))
     (type $obj (sub (struct (field $parent (ref null $class))
                             (field $superclass (ref null $class)))))
     (type $class (sub $obj 
                       (struct (field $parent 
                                      (ref null $class))
                               (field $superclass 
                                      (ref null $class))
                               (field $name 
                                      (ref $str))
                               (field $instance-methods 
                                      (ref $alist-str-method)))))
     (type $method (func (param $self (ref $obj))
                         (param $args (ref $arr-unitype))
                         (result (ref eq))))
     (type $arr-unitype (array (ref eq)))

     (type $alist-str-unitype-pair (struct (field $key (ref $str)) (field $val (ref eq))))
     (type $alist-str-unitype (array (ref $alist-str-unitype-pair)))

     (type $alist-str-method-pair (struct (field $key (ref $str)) (field $val (ref $method))))
     (type $alist-str-method (array (ref $alist-str-method-pair))))

    ;;;; Globals
    ;;; Consts
    (global $false (ref i31)
            (ref.i31 (i32.const ,false-bit-pattern)))

    (global $true (ref i31)
            (ref.i31 (i32.const ,true-bit-pattern)))
    
    (global $nil (ref i31)
            (ref.i31 (i32.const ,nil-bit-pattern)))
    
    ;;; Strings
    ,@string-defs

    ;;; Class instances
    ,@class-defs
    
    ;;;; Functions
    ,@method-defs))
    

(with-open-file (f "./core_generated.wat"
                   :direction :output
                   :if-exists :overwrite
                   :if-does-not-exist :create)
  (pprint *module* f))

(uiop:run-program "wasmtime -W gc=y -W function-references=y core_generated.wat"
                  :output t
                  :error-output t)