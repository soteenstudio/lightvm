pub mod instructions;
pub mod optimizer;
pub mod types;
pub mod utils;
pub mod vm;
use napi_derive::napi;
use types::instructions::Instructions;
#[napi(js_name = "LightVM")]
pub struct LightVM {
  bytecode: Instructions,
}