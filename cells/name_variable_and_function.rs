// rule: NAME kind: violation
// reason: [NAME] `step` names a variable and a function
#![allow(dead_code)]
pub fn walk() -> i32 { fn step() -> i32 { 1 } let step = step(); step }
