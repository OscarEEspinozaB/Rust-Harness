// rule: WORLD kind: gap
#![allow(dead_code)]
type Opened = std::fs::File;
pub fn make() -> std::io::Result<Opened> { Opened::create_new("x") }
