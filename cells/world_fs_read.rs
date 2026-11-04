// rule: WORLD kind: violation
// reason: [WORLD] `std::fs::read_to_string`
#![allow(dead_code)]
pub fn read() -> std::io::Result<String> { std::fs::read_to_string("x") }
