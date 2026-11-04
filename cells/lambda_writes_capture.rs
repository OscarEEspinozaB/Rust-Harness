// rule: LAMBDA kind: violation
// reason: [LAMBDA] closure writes `total`
#![allow(dead_code)]
pub fn sum(values: &[i32]) -> i32 { let mut total = 0; values.iter().for_each(|v| total += v); total }
