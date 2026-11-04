// rule: RELEASE kind: violation
// reason: mem::forget
#![allow(dead_code)]
pub fn f(input: i32) { let data = vec![input]; std::mem::forget(data); }
