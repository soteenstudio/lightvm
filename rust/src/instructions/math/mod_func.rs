use crate::instructions::math::r#mod::{
  mod_f32in::mod_f32in, mod_f64in::mod_f64in, mod_i32in::mod_i32in, mod_i64in::mod_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn mod_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Int32(mod_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Int64(mod_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Flt => Value::Float32(mod_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Dbl => Value::Float64(mod_f64in(a.as_f64(), b.as_f64())),
    _ => Value::Bool(false),
  }
}
