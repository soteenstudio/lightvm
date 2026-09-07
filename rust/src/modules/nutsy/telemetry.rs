/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use std::time::{SystemTime, UNIX_EPOCH};
pub struct TelemetryClient {
  endpoint: String,
  auth_header: String,
  app_name: String,
}
impl TelemetryClient {
  pub fn new(endpoint: &str, auth_header: &str, app_name: &str) -> Self {
    Self {
      endpoint: endpoint.to_string(),
      auth_header: auth_header.to_string(),
      app_name: app_name.to_string(),
    }
  }
  pub fn send_log(&self, level: &str, message: &str) {
    let timestamp_ns = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos()
      .to_string();
    let payload = serde_json::json!({
        "streams": [
            {
                "stream": {
                    "app": self.app_name,
                    "level": level
                },
                "values": [
                    [timestamp_ns, message]
                ]
            }
        ]
    });
    let endpoint = self.endpoint.clone();
    let auth = self.auth_header.clone();
    std::thread::spawn(move || {
      let _ = ureq::post(&endpoint)
        .set("Authorization", &auth)
        .set("Content-Type", "application/json")
        .send_json(payload);
    });
  }
}
