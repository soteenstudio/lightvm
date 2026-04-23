use crate::types::{primitive_types::PrimitiveTypes, value::Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // TAMBAHIN INI
pub enum Instructions {
  Push(Value),
  Val(String),
  Set(String),
  Get(String),
  Add(PrimitiveTypes), // String di sini mungkin buat nentuin NumType-nya
  Sub(PrimitiveTypes),
  Mul(PrimitiveTypes),
  Div(PrimitiveTypes),
  Mod(PrimitiveTypes),
  Gt(PrimitiveTypes),
  Lt(PrimitiveTypes),
  Ge(PrimitiveTypes),
  Le(PrimitiveTypes),
  Eq(PrimitiveTypes),
  Neq(PrimitiveTypes),
  And,
  Or,
  Print,
  Println,
  IfFalse(usize), // Alamat jump pake usize biar kenceng
  Jump(usize),
  Inc(String),
  Dec(String),
  Call(String, u32),
  // Func: name, params_count, start_ip, end_ip, param_names
  Func(String, u32, usize, usize, Vec<String>),
  Stop,
  Return,
  Break,
  Access(String),
  AccessIndex,
  ToString, // Ganti nama dikit biar gak konflik sama reserved word
  ToNumber,
  ToInteger,
  ToFloat,
  MakeObj(u32),
  MakeArray(u32),
  TypeOf,
  Length,
  Concat,
  Dup,
  SetProp(String),
  Import(String, String),
  Export(String),
  Instantiate(String, u32),
}
