// rule: RELEASE kind: control
#![allow(dead_code)]
pub fn f(input: i32) -> i32 { let data = [input].to_vec(); let first = data[0]; drop(data); first * 2 }
