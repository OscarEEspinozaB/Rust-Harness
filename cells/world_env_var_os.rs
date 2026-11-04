// rule: WORLD kind: violation
// reason: [WORLD] `std::env::var_os`
#![allow(dead_code)]
pub fn home() -> Option<std::ffi::OsString> { std::env::var_os("HOME") }
