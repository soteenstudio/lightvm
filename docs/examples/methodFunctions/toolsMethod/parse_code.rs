let str_val = r#"
  push 5; ;; IP=0
  val x; ;; IP=1
  set x; ;; IP=2
"#;
let parsed = tools.parse_ltc(str_val);
println!("{:#}", parsed.clone());
