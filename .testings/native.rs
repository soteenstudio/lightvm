/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

use lightvm::{LightVM, types::capability::Capability};  

fn main() {
  // Inisialisasi VM dengan kapabilitas yang dibutuhin
  let mut vm = LightVM::new(vec![Capability::Control, Capability::Observe]);
  
  // Ambil raw JSON dan lakuin optimasi bytecode
  let raw = r#"[
    ["val", "x"],
    ["push", 5],
    ["set", "x"],
    ["get", "x"],
    ["println"]
  ]"#;
  let optimized_json = LightVM::tools().optimize_bytecode(raw);
  
  // Load bytecode hasil optimasi ke dalam VM
  vm.load(optimized_json.clone());
  
  // Eksekusi VM sekali run
  let res = vm.run(None);
  
  // Opsional: lu bisa cetak hasil `res` di sini kalau mau mastiin outputnya
  println!("===> Execution finished <===");
  println!("Output: {:?}", res);
}
