// rule: STATE kind: violation
// reason: [STATE] static item
#![allow(dead_code)]
#[global_allocator]
static ALLOCATOR: std::alloc::System = std::alloc::System;
pub static OTHER: i32 = 0;
