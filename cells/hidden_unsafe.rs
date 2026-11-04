// rule: HIDDEN kind: violation
// reason: unsafe
#![allow(dead_code)]
pub fn f(p: *const i32) -> i32 { unsafe { *p } }
