use crate::types::value::Value;
use std::sync::Arc;
pub fn tanhv_f64in(arr_a: &Arc<Vec<Value>>) -> Arc<Vec<Value>> {
  let mut res = Vec::with_capacity(arr_a.len());
  for x in arr_a.iter() {
    let tanh = x.as_f64().tanh();
    res.push(Value::Float64(tanh));
  }
  Arc::new(res)
}
