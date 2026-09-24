macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn exported_add() {
      crate::runner::run_case("exported_add", r#"[["jump",7],["func","calculate",2,2,6,"a","b"],["get","a"],["get","b"],["add","int"],["return"],["stop"],["export","calculate"]]"#, Some("calculate"), $optimize);
    }
    #[test]
    fn exported_subtract() {
      crate::runner::run_case("exported_subtract", r#"[["jump",7],["func","calculate",2,2,6,"a","b"],["get","a"],["get","b"],["sub","int"],["return"],["stop"],["export","calculate"]]"#, Some("calculate"), $optimize);
    }
    #[test]
    fn exported_product() {
      crate::runner::run_case("exported_product", r#"[["jump",7],["func","calculate",2,2,6,"a","b"],["get","a"],["get","b"],["mul","int"],["return"],["stop"],["export","calculate"]]"#, Some("calculate"), $optimize);
    }
    #[test]
    fn exported_label() {
      crate::runner::run_case("exported_label", r#"[["jump",7],["func","calculate",2,2,6,"a","b"],["get","a"],["get","b"],["concat"],["return"],["stop"],["export","calculate"]]"#, Some("calculate"), $optimize);
    }
    #[test]
    fn exported_compare() {
      crate::runner::run_case("exported_compare", r#"[["jump",7],["func","calculate",2,2,6,"a","b"],["get","a"],["get","b"],["lt","int"],["return"],["stop"],["export","calculate"]]"#, Some("calculate"), $optimize);
    }
  };
}
pub(crate) use cases;
