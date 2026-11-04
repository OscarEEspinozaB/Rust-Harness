// rule: WORLD kind: control
#![allow(dead_code)]
use std::fs::File;
use std::io::Read;
use std::time::Instant;
pub fn size(file: &mut File, buffer: &mut Vec<u8>) -> std::io::Result<usize> { file.read_to_end(buffer) }
pub fn elapsed(start: Instant, end: Instant) -> std::time::Duration { end - start }
