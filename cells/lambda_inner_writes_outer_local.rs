// rule: LAMBDA kind: violation
// reason: [LAMBDA] closure writes `count`
#![allow(dead_code)]
pub fn run() -> i32 { let outer = || { let mut count = 0; let mut inner = || count += 1; inner(); count }; outer() }
