// rule: WORLD kind: violation
// reason: disallowed method `std::env::args`
#![allow(dead_code)]
pub fn count() -> usize { std::env::args().count() }
