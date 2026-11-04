// rule: WORLD kind: violation
// reason: [WORLD] `std::env`
#![allow(dead_code)]
use std::env::*;
pub fn home() -> bool { var("HOME").is_ok() }
