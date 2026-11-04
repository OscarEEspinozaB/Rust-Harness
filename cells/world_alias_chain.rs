// rule: WORLD kind: violation
// reason: [WORLD] `std::io::stdout`
#![allow(dead_code)]
use std::io as channel;
use channel as pipe;
pub fn bytes() -> bool { pipe::stdout().lock().get_ref_is_some() }
trait Probe { fn get_ref_is_some(&self) -> bool; }
impl<T> Probe for T { fn get_ref_is_some(&self) -> bool { true } }
