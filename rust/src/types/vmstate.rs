use napi_derive::napi;
#[napi(string_enum)]
pub enum VmState {
    Idle,
    Running,
    Halted,
}