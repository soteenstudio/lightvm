use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveTypes {
  Int,
  Lng,
  Flt,
  Dbl,
  Str,
}
