use napi_derive::napi;
#[napi(string_enum)]
#[derive(Debug)]
pub enum VmState {
    Idle,
    Running,
    Halted,
    Panic,
}