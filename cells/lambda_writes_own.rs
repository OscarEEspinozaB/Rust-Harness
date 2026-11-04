// rule: LAMBDA kind: control
#![allow(dead_code)]
pub fn bump(values: &[i32]) -> Vec<i32> { values.iter().map(|v| { let mut own = *v; own += 1; own }).collect() }
pub fn add(slots: &mut [i32]) { slots.iter_mut().for_each(|slot| *slot += 1); }
