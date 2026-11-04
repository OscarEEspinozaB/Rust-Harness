// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
use std::env as outside;
pub fn home() -> bool { outside::var("HOME").is_ok() }
