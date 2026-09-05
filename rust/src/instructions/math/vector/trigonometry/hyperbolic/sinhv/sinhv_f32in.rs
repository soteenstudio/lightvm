use crate::types::value::Value;
use std::sync::Arc;
pub fn sinhv_f32in(arr_a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len());
  for x in arr_a.iter() {
    let sinh = x.as_f32().sinh();
    res.push(Value::Float32(sinh));
  }
  Arc::new(res)
}
