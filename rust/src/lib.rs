/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub(crate) mod instructions;
#[doc(hidden)]
pub(crate) mod interfaces;
#[doc(hidden)]
pub(crate) mod optimizer;
#[doc(hidden)]
pub(crate) mod traits;
#[doc(hidden)]
pub mod types;
#[doc(hidden)]
pub(crate) mod utils;
#[doc(hidden)]
pub(crate) mod vm;
#[doc(hidden)]
#[cfg(feature = "node")]
pub use crate::interfaces::napi_interface::NodeLightVM;
#[doc(hidden)]
#[cfg(feature = "wasm")]
pub use crate::interfaces::wasm_interface::WasmLightVM;
#[cfg(not(feature = "node"))]
pub use interfaces::interface::LightVM;
