// rule: HIDDEN kind: gap
#![allow(dead_code)]
pub struct Guard<'a> { pub count: &'a mut i32 }
impl Drop for Guard<'_> { fn drop(&mut self) { *self.count += 1; } }
