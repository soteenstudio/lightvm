use crate::types::value::Value;
use half::f16;
use std::sync::Arc;
pub fn coshv_f16in(arr_a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len());
  for x in arr_a.iter() {
    let cosh = f16::from_f32(x.as_f16().to_f32().cosh());
    res.push(Value::Float16(cosh));
  }
  Arc::new(res)
}
