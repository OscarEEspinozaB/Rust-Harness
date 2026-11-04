// rule: STATE kind: violation
// reason: disallowed type
#![allow(dead_code)]
use std::sync::Mutex;
static STATE: Mutex<i32> = Mutex::new(0);
pub fn next_value() -> i32 { let mut guard = STATE.lock().unwrap(); *guard += 1; *guard }
