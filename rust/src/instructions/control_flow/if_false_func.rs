use crate::types::value::Value;
#[inline(always)]
pub fn if_false_func(cond: Value) -> bool {
  !cond.as_bool()
}
