(type $unitype_heap (struct))

(func $expect_ref_eq (param $x (ref eq)))
(func $call
      (call $expect_ref_eq (struct.new $unitype_heap)))
