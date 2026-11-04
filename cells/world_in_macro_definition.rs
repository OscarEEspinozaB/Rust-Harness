// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
macro_rules! home { () => { std::env::var("HOME") } }
pub fn found() -> bool { home!().is_ok() }
