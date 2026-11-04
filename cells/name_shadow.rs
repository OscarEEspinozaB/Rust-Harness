// rule: NAME kind: violation
// reason: is shadowed
#![allow(dead_code)]
pub fn f() -> i32 { let v = 1; let v = v + 1; v }
