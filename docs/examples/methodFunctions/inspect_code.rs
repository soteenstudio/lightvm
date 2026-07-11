let report = vm.inspect();
println!("{}", serde_json::to_string_pretty(&report).unwrap());
