// rule: COMPILER kind: violation
// reason: cannot return reference
#![allow(dead_code)]
pub fn f<'a>() -> &'a i32 { let v = 1; &v }
