// rule: NAME kind: control
#![allow(dead_code)]
pub struct Line { pub size: usize }
impl Line { pub fn size(&self) -> usize { let size = self.size; size } }
pub fn other(size: usize) -> usize { size }
