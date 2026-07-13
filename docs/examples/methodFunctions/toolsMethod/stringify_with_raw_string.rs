let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
let stringify = tools.stringify_ltc(raw);
println!("{:#}", stringify.clone());
