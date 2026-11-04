// rule: STATE kind: violation
// reason: [STATE] static item
#![allow(dead_code)]
pub fn limit() -> i32 { static LIMIT: i32 = 3; LIMIT }
