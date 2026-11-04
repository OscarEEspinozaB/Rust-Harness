// rule: CONTEXT kind: violation
// reason: this parameter is a mutable reference but is not used mutably
#![allow(dead_code)]
pub fn total(values: &mut Vec<i32>) -> i32 { values.iter().sum() }
