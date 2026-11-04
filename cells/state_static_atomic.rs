// rule: STATE kind: violation
// reason: disallowed type
#![allow(dead_code)]
use std::sync::atomic::{AtomicI32, Ordering};
static COUNTER: AtomicI32 = AtomicI32::new(0);
pub fn next_value() -> i32 { COUNTER.fetch_add(1, Ordering::Relaxed) }
