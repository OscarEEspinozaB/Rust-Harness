// rule: STATE kind: control
#![allow(dead_code)]
pub struct Table { pub size: i32 }
pub const TABLE: Table = Table { size: 3 };
pub fn size() -> i32 { TABLE.size }
