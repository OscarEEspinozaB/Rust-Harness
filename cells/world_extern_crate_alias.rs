// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
extern crate std as standard;
pub fn home() -> bool { standard::env::var("HOME").is_ok() }
