// rule: HIDDEN kind: gap
#![allow(dead_code)]
macro_rules! set { ($x:expr) => { $x = 1 } }
pub fn f() -> i32 { let mut a = 0; set!(a); a }
