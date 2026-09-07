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
  fn payload(&self, level: &str, message: &str, timestamp_ns: String) -> serde_json::Value {
    serde_json::json!({
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
    })
  }
  pub fn send_log(&self, level: &str, message: &str) -> Result<(), Box<ureq::Error>> {
    let timestamp_ns = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos()
      .to_string();
    ureq::post(&self.endpoint)
      .set("Authorization", &self.auth_header)
      .set("Content-Type", "application/json")
      .send_json(self.payload(level, message, timestamp_ns))
      .map_err(Box::new)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::TelemetryClient;
  use std::io::{Read, Write};
  use std::net::TcpListener;
  use std::thread;

  #[test]
  fn payload_contains_expected_fields() {
    let client = TelemetryClient::new("http://localhost", "Bearer token", "lightvm");
    let payload = client.payload("info", "run succeeded", "123456789".to_string());

    assert_eq!(payload["streams"][0]["stream"]["app"], "lightvm");
    assert_eq!(payload["streams"][0]["stream"]["level"], "info");
    assert_eq!(payload["streams"][0]["values"][0][0], "123456789");
    assert_eq!(payload["streams"][0]["values"][0][1], "run succeeded");
  }

  #[test]
  fn send_log_returns_server_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
      let (mut stream, _) = listener.accept().unwrap();
      let mut request = [0; 4096];
      stream.read(&mut request).unwrap();
      stream
        .write_all(b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
    });
    let client = TelemetryClient::new(&endpoint, "Bearer token", "lightvm");

    let error = client.send_log("info", "run succeeded").unwrap_err();

    assert!(matches!(*error, ureq::Error::Status(500, _)));
    server.join().unwrap();
  }
}
