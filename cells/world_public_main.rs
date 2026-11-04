// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
pub fn main() { let _ = std::env::var("HOME"); }
