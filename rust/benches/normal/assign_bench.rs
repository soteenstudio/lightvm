macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn counter_update() {
      crate::runner::run_case("counter_update", r#"[["val","count"],["push",1],["set","count"],["inc","count","int"],["get","count"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn balance_adjustment() {
      crate::runner::run_case("balance_adjustment", r#"[["val","balance"],["push",90],["set","balance"],["get","balance"],["push",10],["sub","int"],["set","balance"],["get","balance"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn status_replacement() {
      crate::runner::run_case("status_replacement", r#"[["val","status"],["push","pending"],["set","status"],["push","ready"],["set","status"],["get","status"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn tally_decrement() {
      crate::runner::run_case("tally_decrement", r#"[["val","tally"],["push",8],["set","tally"],["dec","tally","int"],["get","tally"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn running_sum() {
      crate::runner::run_case("running_sum", r#"[["val","sum"],["push",10],["set","sum"],["get","sum"],["push",20],["add","int"],["set","sum"],["get","sum"],["stop"]]"#, None, $optimize);
    }
  };
}
pub(crate) use cases;
