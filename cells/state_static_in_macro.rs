// rule: STATE kind: violation
// reason: [STATE] static item
#![allow(dead_code)]
macro_rules! global { () => { pub static HIDDEN: i32 = 0; } }
global!();
