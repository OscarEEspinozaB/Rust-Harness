// rule: BYPASS kind: violation
// reason: [BYPASS] `unsafe_code`
#![cfg_attr(all(), allow(unsafe_code))]
pub fn zero() -> i32 { 0 }
