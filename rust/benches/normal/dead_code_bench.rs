macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn active_branch() {
      crate::runner::run_case("active_branch", r#"[["push",true],["if_false",4],["push","active"],["jump",5],["push","inactive"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn inactive_branch() {
      crate::runner::run_case("inactive_branch", r#"[["push",false],["if_false",4],["push","active"],["jump",5],["push","inactive"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn skip_diagnostics() {
      crate::runner::run_case("skip_diagnostics", r#"[["jump",3],["push","unreachable"],["push",999],["push","ready"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn stock_threshold() {
      crate::runner::run_case("stock_threshold", r#"[["push",9],["push",5],["gt","int"],["if_false",6],["push","reorder"],["jump",7],["push","available"],["stop"]]"#, None, $optimize);
    }
    #[test]
    fn matching_route() {
      crate::runner::run_case("matching_route", r#"[["push",4],["push",4],["eq","int"],["if_false",6],["push","matched"],["jump",7],["push","other"],["stop"]]"#, None, $optimize);
    }
  };
}
pub(crate) use cases;
