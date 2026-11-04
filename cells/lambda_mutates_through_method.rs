// rule: LAMBDA kind: gap
#![allow(dead_code)]
pub fn copy(values: &[i32]) -> Vec<i32> { let mut out = Vec::new(); values.iter().for_each(|v| out.push(*v)); out }
