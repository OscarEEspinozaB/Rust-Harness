// rule: STATE kind: control
#![allow(dead_code)]
macro_rules! name { () => { pub fn name() -> &'static str { "x" } } }
name!();
pub fn other() -> &'static str { "y" }
