use crate::types::value::Value;
pub struct VmResult {
  pub value: Value,
  pub outputs: Vec<String>,
  pub halted: bool,
}
