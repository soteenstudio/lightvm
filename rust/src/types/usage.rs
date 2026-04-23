use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Usage {
  pub read: HashSet<String>,
  pub written: HashSet<String>,
}
impl Usage {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn add_read(&mut self, variable: &str) {
    self.read.insert(variable.to_string());
  }
  pub fn add_written(&mut self, variable: &str) {
    self.written.insert(variable.to_string());
  }
}
