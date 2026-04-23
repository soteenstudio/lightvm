use crate::types::value::Value;

#[inline(always)]
pub fn if_false_func(cond: Value) -> bool {
    // Pake as_bool() lo yang udah handle:
    // - bool asli
    // - int != 0
    // - string !empty
    // - null/undefined = false
    !cond.as_bool()
}
