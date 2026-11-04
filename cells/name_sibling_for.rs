// rule: NAME kind: control
#![allow(dead_code)]
pub fn f() -> i32 { let mut t = 0; for i in 0..2 { t += i; } for i in 0..2 { t += i; } t }
