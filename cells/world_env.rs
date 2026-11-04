// rule: WORLD kind: violation
// reason: disallowed method
#![allow(dead_code)]
pub fn home() -> Option<String> { std::env::var("HOME").ok() }
