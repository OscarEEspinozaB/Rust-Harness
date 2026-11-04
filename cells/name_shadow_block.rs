// rule: NAME kind: violation
// reason: shadows a previous
#![allow(dead_code)]
pub fn f() -> i32 { let v = 1; { let v = 2; if v != 2 { return 0; } } v }
