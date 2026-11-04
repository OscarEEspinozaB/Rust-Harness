// rule: STATE kind: violation
// reason: disallowed type `std::sync::atomic::AtomicPtr`
#![allow(dead_code)]
pub fn f(slot: &std::sync::atomic::AtomicPtr<u8>) -> *mut u8 { slot.load(std::sync::atomic::Ordering::Relaxed) }
