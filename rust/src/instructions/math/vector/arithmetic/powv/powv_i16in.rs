use crate::instructions::math::arithmetic::pow::pow_i16in::pow_i16in;
use crate::types::value::Value;
use std::sync::Arc;
pub fn powv_i16in(arr_a: &Arc<Vec<Value>>, arr_b: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len().min(arr_b.len()));
  for (x, y) in arr_a.iter().zip(arr_b.iter()) {
    let base = x.as_i16();
    let exp = y.as_i16();
    res.push(Value::Int16(pow_i16in(base, exp)));
  }
  Arc::new(res)
}
