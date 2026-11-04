// rule: COMPILER kind: violation
// reason: moved value
#![allow(dead_code)]
pub fn f(input: i32) -> i32 { let data = vec![input]; drop(data); data[0] }
