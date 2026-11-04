// rule: STATE kind: violation
// reason: [STATE] static item
#![allow(dead_code)]
pub struct Table { pub size: i32 }
pub static TABLE: Table = Table { size: 3 };
pub fn size() -> i32 { TABLE.size }
