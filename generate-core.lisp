(ql:quickload :named-readtables)
(defpackage :generate-core
  (:use :cl))
(in-package :generate-core)

;; To get case preserved, we have to set the readtable to
;;   case-preserving, then write function calls in upper-case...
(named-readtables:in-readtable :modern)


;; Convert 
(DEFUN STRING-DEF (STR)
  (LET* ((CONSTS (MAPCAR (LAMBDA (C) (FORMAT NIL "(i32.const ~D)" (CHAR-INT C)))
                         (COERCE STR 'LIST)))
         (RES (FORMAT NIL "(global $const-str-~A (ref $str) (array.new_fixed $str ~D ~{~A~^ ~}))"
                      STR (LENGTH STR) CONSTS)))
    (READ-FROM-STRING RES)))
  
(DEFPARAMETER *MODULE*
  `(module
    (type $str (array i8))
    (type $obj (array i8))
    (type $method (func (param $self (ref $obj))
                        (param $args (ref $arr-unitype))
                        (result (ref eq))))
    ,(STRING-DEF "new"))))

(WITH-OPEN-FILE (F "./core_generated.wat"))
(PPRINT *MODULE* *STANDARD-OUTPUT*)
