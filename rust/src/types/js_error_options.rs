#[cfg(feature = "node")]
use napi_derive::napi;
#[cfg(feature = "node")]
#[napi(object)]
#[derive(Default, ts_rs::TS)]
#[ts(export, rename = "ErrorOptions")]
pub struct JSErrorOptions {
  pub backtrace: Option<bool>,
  pub explain: Option<bool>,
  pub hint: Option<bool>,
}
