/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

#[allow(clippy::type_complexity)]
pub mod instructions;
pub mod interfaces;
pub mod optimizer;
pub mod types;
pub mod utils;
pub mod vm;
#[cfg(feature = "node")]
pub use crate::interfaces::napi_interface::NodeLightVM;
#[cfg(not(feature = "node"))]
pub use interfaces::interface::LightVM;
