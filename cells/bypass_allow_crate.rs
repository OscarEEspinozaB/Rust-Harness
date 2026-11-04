// rule: BYPASS kind: violation
// reason: [BYPASS] `clippy::disallowed_methods`
#![allow(clippy::disallowed_methods)]
pub fn clock() -> std::time::Instant { std::time::Instant::now() }
