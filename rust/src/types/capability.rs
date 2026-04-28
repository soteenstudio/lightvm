use napi_derive::napi;
use serde::{Serialize, Deserialize};
#[napi(string_enum)]
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Capability {
  Control,
  Observe,
  Debug,
  Unsafe,
}