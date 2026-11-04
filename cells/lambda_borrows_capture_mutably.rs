// rule: LAMBDA kind: violation
// reason: [LAMBDA] closure writes `list`
#![allow(dead_code)]
fn fill(list: &mut Vec<i32>) { list.push(1); }
pub fn run() -> Vec<i32> { let mut list = Vec::new(); let mut go = || fill(&mut list); go(); list }
