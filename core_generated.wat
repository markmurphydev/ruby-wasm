
(REC (TYPE $STR (ARRAY I8))
 (TYPE $OBJ (SUB (STRUCT (FIELD $PARENT (MUT (REF NULL $CLASS))))))
 (TYPE $CLASS
  (SUB $OBJ
   (STRUCT (FIELD $PARENT (MUT (REF NULL $CLASS)))
    (FIELD $SUPERCLASS (MUT (REF NULL $CLASS))) (FIELD $NAME (REF $STR))
    (FIELD $INSTANCE-METHODS (REF |$alist-str-method|)))))
 (TYPE $METHOD
  (FUNC (PARAM $SELF (REF $OBJ)) (PARAM $ARGS (REF $ARR-UNITYPE))
   (RESULT (REF EQ))))
 (TYPE $ARR-UNITYPE (ARRAY (REF EQ)))
 (TYPE |$alist-str-unitype| (ARRAY (REF |$alist-pair-str-unitype|)))
 (TYPE |$alist-pair-str-unitype|
  (STRUCT (FIELD $KEY (REF |$str|)) (FIELD $VAL (REF EQ))))
 (TYPE |$alist-str-method| (ARRAY (REF |$alist-pair-str-method|)))
 (TYPE |$alist-pair-str-method|
  (STRUCT (FIELD $KEY (REF |$str|)) (FIELD $VAL (REF |$method|)))))
(GLOBAL $FALSE (REF I31) (REF.I31 (I32.CONST 1)))
(GLOBAL $TRUE (REF I31) (REF.I31 (I32.CONST 3)))
(GLOBAL $NIL (REF I31) (REF.I31 (I32.CONST 5)))
(GLOBAL $EMPTY-ARGS (REF $ARR-UNITYPE) (ARRAY.NEW_FIXED $ARR-UNITYPE 0))
(GLOBAL |$STR-class| (REF $STR)
 (ARRAY.NEW_FIXED $STR 5 (I32.CONST 99) (I32.CONST 108) (I32.CONST 97)
  (I32.CONST 115) (I32.CONST 115)))
(GLOBAL |$STR-new| (REF $STR)
 (ARRAY.NEW_FIXED $STR 3 (I32.CONST 110) (I32.CONST 101) (I32.CONST 119)))
(GLOBAL |$STR-Object| (REF $STR)
 (ARRAY.NEW_FIXED $STR 6 (I32.CONST 79) (I32.CONST 98) (I32.CONST 106)
  (I32.CONST 101) (I32.CONST 99) (I32.CONST 116)))
(GLOBAL |$STR-BasicObject| (REF $STR)
 (ARRAY.NEW_FIXED $STR 11 (I32.CONST 66) (I32.CONST 97) (I32.CONST 115)
  (I32.CONST 105) (I32.CONST 99) (I32.CONST 79) (I32.CONST 98) (I32.CONST 106)
  (I32.CONST 101) (I32.CONST 99) (I32.CONST 116)))
(GLOBAL |$STR-Module| (REF $STR)
 (ARRAY.NEW_FIXED $STR 6 (I32.CONST 77) (I32.CONST 111) (I32.CONST 100)
  (I32.CONST 117) (I32.CONST 108) (I32.CONST 101)))
(GLOBAL |$STR-Class| (REF $STR)
 (ARRAY.NEW_FIXED $STR 5 (I32.CONST 67) (I32.CONST 108) (I32.CONST 97)
  (I32.CONST 115) (I32.CONST 115)))
(GLOBAL |$class-Class| (REF $CLASS)
 (STRUCT.NEW $CLASS (REF.NULL $CLASS) (REF.NULL $CLASS)
  (GLOBAL.GET |$STR-Class|)
  (ARRAY.NEW_FIXED |$alist-str-method| 1
   (STRUCT.NEW |$alist-pair-str-method| (GLOBAL.GET |$STR-new|)
    (REF.FUNC |$method-Class-new|)))))
(GLOBAL |$class-Module| (REF $CLASS)
 (STRUCT.NEW $CLASS (REF.NULL $CLASS) (REF.NULL $CLASS)
  (GLOBAL.GET |$STR-Module|) (ARRAY.NEW_FIXED |$alist-str-method| 0)))
(GLOBAL |$class-BasicObject| (REF $CLASS)
 (STRUCT.NEW $CLASS (REF.NULL $CLASS) (REF.NULL $CLASS)
  (GLOBAL.GET |$STR-BasicObject|) (ARRAY.NEW_FIXED |$alist-str-method| 0)))
(GLOBAL |$class-Object| (REF $CLASS)
 (STRUCT.NEW $CLASS (REF.NULL $CLASS) (REF.NULL $CLASS)
  (GLOBAL.GET |$STR-Object|) (ARRAY.NEW_FIXED |$alist-str-method| 0)))
(FUNC |$method-Class-new| (TYPE $METHOD) (PARAM $SELF (REF $OBJ))
 (PARAM $ARGS (REF $ARR-UNITYPE)) (RESULT (REF EQ))
 (STRUCT.NEW $OBJ (REF.CAST (REF $CLASS) (LOCAL.GET $SELF))))
(FUNC $STR-EQ (PARAM $A (REF $STR)) (PARAM $B (REF $STR)) (RESULT I32)
 (LOCAL $IDX I32) (LOCAL $A_CH I32) (LOCAL $B_CH I32)
 (LOCAL.SET $IDX (I32.CONST 0))
 (IF (I32.EQZ (I32.EQ (ARRAY.LEN (LOCAL.GET $A)) (ARRAY.LEN (LOCAL.GET $B))))
     (THEN (RETURN (I32.CONST 0)))
     (ELSE
      (LOOP $FOR (IF (I32.EQ (LOCAL.GET $IDX) (ARRAY.LEN (LOCAL.GET $A)))
                     (THEN (RETURN (I32.CONST 1)))) (LOCAL.SET $A_CH
                                                     (ARRAY.GET_U $STR
                                                      (LOCAL.GET $A)
                                                      (LOCAL.GET
                                                       $IDX))) (LOCAL.SET $B_CH
                                                                (ARRAY.GET_U
                                                                 $STR
                                                                 (LOCAL.GET $B)
                                                                 (LOCAL.GET
                                                                  $IDX))) (IF (I32.EQZ
                                                                               (I32.EQ
                                                                                (LOCAL.GET
                                                                                 $A_CH)
                                                                                (LOCAL.GET
                                                                                 $B_CH)))
                                                                              (THEN
                                                                               (RETURN
                                                                                (I32.CONST
                                                                                 0)))) (LOCAL.SET
                                                                                        $IDX
                                                                                        (I32.ADD
                                                                                         (LOCAL.GET
                                                                                          $IDX)
                                                                                         (I32.CONST
                                                                                          1))) (BR
                                                                                                $FOR))))
 (UNREACHABLE))
(FUNC $ALIST-STR-METHOD-GET (PARAM $ALIST (REF |$alist-str-method|))
 (PARAM $NAME (REF $STR)) (RESULT (REF $METHOD)) (LOCAL $IDX I32)
 (LOCAL $PAIR (REF |$alist-pair-str-method|)) (LOCAL $KEY (REF $STR))
 (LOCAL $VAL (REF $METHOD)) (LOCAL.SET $IDX (I32.CONST 0))
 (LOCAL.SET $IDX (I32.CONST 0))
 (LOOP $FOR (IF (I32.EQ (LOCAL.GET $IDX) (ARRAY.LEN (LOCAL.GET $ALIST)))
                (THEN (UNREACHABLE))) (LOCAL.SET $PAIR
                                       (ARRAY.GET |$alist-str-method|
                                        (LOCAL.GET $ALIST)
                                        (LOCAL.GET $IDX))) (LOCAL.SET $KEY
                                                            (STRUCT.GET
                                                             |$alist-pair-str-method|
                                                             $KEY
                                                             (LOCAL.GET
                                                              $PAIR))) (LOCAL.SET
                                                                        $VAL
                                                                        (STRUCT.GET
                                                                         |$alist-pair-str-method|
                                                                         $VAL
                                                                         (LOCAL.GET
                                                                          $PAIR))) (IF (CALL
                                                                                        $STR-EQ
                                                                                        (LOCAL.GET
                                                                                         $KEY)
                                                                                        (LOCAL.GET
                                                                                         $NAME))
                                                                                       (THEN
                                                                                        (RETURN
                                                                                         (LOCAL.GET
                                                                                          $VAL)))) (LOCAL.SET
                                                                                                    $IDX
                                                                                                    (I32.ADD
                                                                                                     (LOCAL.GET
                                                                                                      $IDX)
                                                                                                     (I32.CONST
                                                                                                      1))) (BR
                                                                                                            $FOR))
 (UNREACHABLE))
(FUNC $CALL (PARAM $RECEIVER (REF $OBJ)) (PARAM $MESSAGE (REF $STR))
 (PARAM $ARGS (REF $ARR-UNITYPE)) (RESULT (REF EQ))
 (LOCAL $PARENT (REF $CLASS)) (LOCAL $METHOD (REF $METHOD))
 (LOCAL.SET $PARENT
  (REF.AS_NON_NULL (STRUCT.GET $OBJ $PARENT (LOCAL.GET $RECEIVER))))
 (LOCAL.SET $METHOD
  (REF.CAST (REF $METHOD)
   (CALL $ALIST-STR-METHOD-GET
    (STRUCT.GET $CLASS $INSTANCE-METHODS (LOCAL.GET $PARENT))
    (LOCAL.GET $MESSAGE))))
 (CALL_REF $METHOD (LOCAL.GET $RECEIVER) (LOCAL.GET $ARGS) (LOCAL.GET $METHOD)))
(FUNC $START
 (STRUCT.SET $CLASS $PARENT (GLOBAL.GET |$class-Class|)
  (GLOBAL.GET |$class-Class|))
 (STRUCT.SET $CLASS $PARENT (GLOBAL.GET |$class-Module|)
  (GLOBAL.GET |$class-Class|))
 (STRUCT.SET $CLASS $PARENT (GLOBAL.GET |$class-BasicObject|)
  (GLOBAL.GET |$class-Class|))
 (STRUCT.SET $CLASS $PARENT (GLOBAL.GET |$class-Object|)
  (GLOBAL.GET |$class-Class|))
 (STRUCT.SET $CLASS $SUPERCLASS (GLOBAL.GET |$class-Class|)
  (GLOBAL.GET |$class-Module|))
 (STRUCT.SET $CLASS $SUPERCLASS (GLOBAL.GET |$class-Module|)
  (GLOBAL.GET |$class-Object|))
 (STRUCT.SET $CLASS $SUPERCLASS (GLOBAL.GET |$class-Object|)
  (GLOBAL.GET |$class-BasicObject|)))
(START $START)