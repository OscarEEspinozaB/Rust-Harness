// rule: COMPILER kind: violation
// reason: moved value
#![allow(dead_code)]
pub fn f(input: i32) { let a = vec![input]; let b = a; drop(a); drop(b); }
