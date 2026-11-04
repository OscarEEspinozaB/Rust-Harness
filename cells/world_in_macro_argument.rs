// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
pub fn text() -> String { format!("{:?}", std::env::var("HOME")) }
