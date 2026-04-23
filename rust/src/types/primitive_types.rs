use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // TAMBAHIN INI
pub enum PrimitiveTypes {
  Int,
  Lng,
  Flt,
  Dbl,
  Str,
}
