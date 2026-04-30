use crate::instructions::comparison::lt::{
  lt_f32in::lt_f32in, lt_f64in::lt_f64in, lt_i32in::lt_i32in, lt_i64in::lt_i64in,
};
use crate::types::{primitive_types::PrimitiveTypes, value::Value};
#[inline]
pub fn lt_func(a: Value, b: Value, num_type: PrimitiveTypes) -> Value {
  match num_type {
    PrimitiveTypes::Int => Value::Bool(lt_i32in(a.as_i32(), b.as_i32())),
    PrimitiveTypes::Lng => Value::Bool(lt_i64in(a.as_i64(), b.as_i64())),
    PrimitiveTypes::Dbl => Value::Bool(lt_f64in(a.as_f64(), b.as_f64())),
    PrimitiveTypes::Flt => Value::Bool(lt_f32in(a.as_f32(), b.as_f32())),
    _ => Value::Bool(false),
  }
}
