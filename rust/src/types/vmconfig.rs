use crate::types::capability::Capability;
#[derive(Default)]
pub struct VmConfig {
  pub caps: Vec<Capability>,
  pub nightly: bool,
}
