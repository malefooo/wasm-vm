(module
  (type $t1 (func (param i32 i32) (result i32)))
  (table funcref (elem $f1 $f2 $f3))
  (func $f1 (type $t1) (i32.add (local.get 0) (local.get 1)))
  (func $f2 (type $t1) (i32.sub (local.get 0) (local.get 1)))
  (func $f3 (type $t1) (i32.mul (local.get 0) (local.get 1)))
  (func $test (type $t1)
    (local.get 0) (local.get 1)
    (call_indirect (type $t1) (i32.const 1))
  )
)