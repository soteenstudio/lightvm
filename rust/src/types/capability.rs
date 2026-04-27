use napi_derive::napi;
#[napi(string_enum)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Capability {
  Control,
  Observe,
  Debug,
  Unsafe,
}