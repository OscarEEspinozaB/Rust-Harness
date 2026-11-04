// rule: COMPILER kind: violation
// reason: cannot move out
#![allow(dead_code)]
pub fn f(input: i32) -> i32 { let data = vec![input]; let first = &data[0]; drop(data); *first }
