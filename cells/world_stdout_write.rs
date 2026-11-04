// rule: WORLD kind: violation
// reason: [WORLD] `std::io::stdout`
#![allow(dead_code)]
use std::io::Write;
pub fn hello() { let _ = writeln!(std::io::stdout(), "hello"); }
