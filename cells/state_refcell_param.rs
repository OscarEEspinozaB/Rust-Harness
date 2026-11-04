// rule: STATE kind: violation
// reason: disallowed type
#![allow(dead_code)]
use std::cell::RefCell;
pub fn looks_read_only(value: &RefCell<i32>) { *value.borrow_mut() += 1; }
