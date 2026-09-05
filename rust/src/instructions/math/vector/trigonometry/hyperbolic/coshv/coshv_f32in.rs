use crate::types::value::Value;
use std::sync::Arc;
pub fn coshv_f32in(arr_a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len());
  for x in arr_a.iter() {
    let cosh = x.as_f32().cosh();
    res.push(Value::Float32(cosh));
  }
  Arc::new(res)
}
