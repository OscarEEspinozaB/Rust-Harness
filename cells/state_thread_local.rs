// rule: STATE kind: violation
// reason: disallowed macro
#![allow(dead_code)]
use std::cell::Cell;
thread_local! { static COUNTER: Cell<i32> = const { Cell::new(0) }; }
pub fn next_value() -> i32 { COUNTER.with(|c| { c.set(c.get() + 1); c.get() }) }
