// rule: NAME kind: control
#![allow(dead_code)]
pub fn f() -> i32 { let mut t = 0; { let item = 1; t += item; } { let item = 2; t += item; } t }
