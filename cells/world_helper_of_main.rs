// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var`
#![allow(dead_code)]
fn home() -> bool { std::env::var("HOME").is_ok() }
#[allow(clippy::disallowed_methods)]
fn main() { let _ = home(); }
