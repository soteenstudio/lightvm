let str_val = r#"
  push 5; ;; IP=0
  val x; ;; IP=1
  set x; ;; IP=2
"#;
if let Some(parsed) = tools.parse_ltc_array_or_display(str_val) {
  println!("{:#}", parsed);
}
