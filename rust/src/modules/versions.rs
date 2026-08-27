/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#[derive(Debug)]
struct ModuleVersions {
  carzy: String,
  gazle: String,
  itme: String,
  krates: String,
  torja: String,
}
#[derive(Debug)]
pub struct InfoVM {
  name: String,
  version: String,
  modules: ModuleVersions,
}
pub fn get_versions() -> InfoVM {
  InfoVM {
    name: String::from("LightVM"),
    version: String::from("0.1.0-alpha.9-nightly"),
    modules: ModuleVersions {
      carzy: String::from("0.1.0"),
      gazle: String::from("0.1.0"),
      itme: String::from("0.1.0"),
      krates: String::from("0.1.0"),
      torja: String::from("0.1.0"),
    },
  }
}
