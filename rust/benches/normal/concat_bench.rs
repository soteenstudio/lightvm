macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn full_name() {
      crate::runner::run_case(
        "full_name",
        r#"[["push","Ada"],["push"," Lovelace"],["concat"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn log_prefix() {
      crate::runner::run_case(
        "log_prefix",
        r#"[["push","INFO: "],["push","connected"],["concat"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn url_path() {
      crate::runner::run_case(
        "url_path",
        r#"[["push","/api/"],["push","users"],["concat"],["push","/active"],["concat"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn formatted_count() {
      crate::runner::run_case(
        "formatted_count",
        r#"[["push","items: "],["push",7],["to_string"],["concat"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn string_length() {
      crate::runner::run_case(
        "string_length",
        r#"[["push","cached"],["push"," result"],["concat"],["length"],["stop"]]"#,
        None,
        $optimize,
      );
    }
  };
}
pub(crate) use cases;
