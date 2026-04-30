use napi_derive::napi;
use serde::{Deserialize, Serialize};
#[napi]
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Capability {
  Control,
  Observe,
  Debug,
  Unsafe,
}
