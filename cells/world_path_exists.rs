// rule: WORLD kind: violation
// reason: disallowed method `std::path::Path::exists`
#![allow(dead_code)]
pub fn found(path: &std::path::Path) -> bool { path.exists() }
