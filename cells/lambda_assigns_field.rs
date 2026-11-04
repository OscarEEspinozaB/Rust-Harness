// rule: LAMBDA kind: violation
// reason: [LAMBDA] closure writes `state`
#![allow(dead_code)]
pub struct State { pub n: i32 }
pub fn set(state: &mut State) { let mut apply = || state.n = 1; apply(); }
