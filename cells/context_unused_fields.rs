// rule: CONTEXT kind: gap
#![allow(dead_code)]
pub struct Ctx { pub a: i32, pub b: Vec<i32>, pub c: String, pub d: Vec<String>, pub e: u64, pub f: bool }
pub fn only_a(ctx: &mut Ctx) { ctx.a += 1; }
