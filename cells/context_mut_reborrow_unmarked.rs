// rule: CONTEXT kind: gap
#![allow(dead_code)]
fn write(slot: &mut i32) { *slot = 1; }
pub fn call_site_shows_no_mark(slot: &mut i32) { write(slot); }
