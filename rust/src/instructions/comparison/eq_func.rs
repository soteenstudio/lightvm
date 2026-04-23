use crate::instructions::comparison::eq::{
  eq_f32in::eq_f32in, eq_f64in::eq_f64in, eq_i32in::eq_i32in, eq_i64in::eq_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};

#[inline]
pub fn eq_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Bool(eq_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(eq_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(eq_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(eq_f32in(a.as_f32(), b.as_f32())),
    PrimitiveTypes::Str => Value::Bool(a.as_string() == b.as_string()),
  }
}
