// rule: CONTEXT kind: violation
// reason: not used mutably
#![allow(dead_code)]
fn total(values: &mut Vec<i32>) -> i32 { values.iter().sum() }
pub fn entry() -> i32 { let mut values = vec![1]; total(&mut values) }
