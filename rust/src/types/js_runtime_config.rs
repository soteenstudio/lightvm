#[cfg(feature = "node")]
use napi_derive::napi;
#[cfg(feature = "node")]
#[napi(object)]
#[derive(Default, ts_rs::TS)]
#[ts(export, rename = "RuntimeConfig")]
pub struct JSRuntimeConfig {
  pub nightly: Option<bool>,
}
