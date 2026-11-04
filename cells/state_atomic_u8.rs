// rule: STATE kind: violation
// reason: disallowed type `std::sync::atomic::AtomicU8`
#![allow(dead_code)]
pub fn f(flag: &std::sync::atomic::AtomicU8) -> u8 { flag.load(std::sync::atomic::Ordering::Relaxed) }
