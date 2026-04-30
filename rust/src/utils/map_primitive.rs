use crate::types::primitive_types::PrimitiveTypes;
use serde_json::Value as JsonValue;
pub fn map_primitive(val: Option<&JsonValue>) -> PrimitiveTypes {
  match val.and_then(|v| v.as_str()) {
    Some("int") => PrimitiveTypes::Int,
    Some("lng") => PrimitiveTypes::Lng,
    Some("flt") => PrimitiveTypes::Flt,
    Some("dbl") => PrimitiveTypes::Dbl,
    Some("str") => PrimitiveTypes::Str,
    _ => PrimitiveTypes::Int,
  }
}
