let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
if let Some(stringify) = tools.stringify_ltc_or_display(raw) {
  println!("{:#}", stringify);
}
