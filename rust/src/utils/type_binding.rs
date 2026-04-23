use crate::types::primitive_types::PrimitiveTypes;
pub fn type_binding(num_type: &str) -> PrimitiveTypes {
  match num_type {
    "int" => PrimitiveTypes::Int,
    "lng" => PrimitiveTypes::Lng,
    "flt" => PrimitiveTypes::Flt,
    "dbl" => PrimitiveTypes::Dbl,
    _ => PrimitiveTypes::Dbl,
  }
}
