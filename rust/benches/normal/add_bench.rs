macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn invoice_total() {
      crate::runner::run_case(
        "invoice_total",
        r#"[["push",12],["push",8],["add","int"],["push",3],["mul","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn discount_amount() {
      crate::runner::run_case(
        "discount_amount",
        r#"[["push",100],["push",15],["sub","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn unit_conversion() {
      crate::runner::run_case(
        "unit_conversion",
        r#"[["push",24],["to_float"],["push",2.5],["mul","flt"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn average_score() {
      crate::runner::run_case(
        "average_score",
        r#"[["push",40],["push",60],["add","int"],["push",2],["div","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn bucket_remainder() {
      crate::runner::run_case(
        "bucket_remainder",
        r#"[["push",53],["push",7],["mod","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
  };
}
pub(crate) use cases;
