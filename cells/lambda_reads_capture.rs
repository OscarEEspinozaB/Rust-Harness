// rule: LAMBDA kind: control
#![allow(dead_code)]
pub fn scale(values: &[i32], factor: i32) -> Vec<i32> { values.iter().map(|v| v * factor).collect() }
