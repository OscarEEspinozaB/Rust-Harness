// rule: WORLD kind: violation
// reason: disallowed method
#![allow(dead_code)]
pub fn stamp() -> std::time::Instant { std::time::Instant::now() }
