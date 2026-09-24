macro_rules! cases {
  ($optimize:expr) => {
    #[test]
    fn type_dispatch() {
      crate::runner::run_case(
        "type_dispatch",
        r#"[["push",42],["typeof"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn boolean_gate() {
      crate::runner::run_case(
        "boolean_gate",
        r#"[["push",true],["push",false],["or"],["not"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn stack_duplicate() {
      crate::runner::run_case(
        "stack_duplicate",
        r#"[["push",16],["dup"],["add","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn not_equal_check() {
      crate::runner::run_case(
        "not_equal_check",
        r#"[["push",3],["push",4],["neq","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn conversion_check() {
      crate::runner::run_case(
        "conversion_check",
        r#"[["push","12"],["to_integer"],["push",2],["add","int"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn array_construction() {
      crate::runner::run_case(
        "array_construction",
        r#"[["push",1],["push",2],["push",3],["make_array",3],["length"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn indexed_lookup() {
      crate::runner::run_case(
        "indexed_lookup",
        r#"[["push",10],["push",20],["push",30],["make_array",3],["push",1],["access_index"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn object_construction() {
      crate::runner::run_case(
        "object_construction",
        r#"[["push","id"],["push",12],["make_obj",1],["length"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn property_update() {
      crate::runner::run_case(
        "property_update",
        r#"[["push","id"],["push",12],["make_obj",1],["push",13],["set_prop","id"],["access","id"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn array_inspection() {
      crate::runner::run_case(
        "array_inspection",
        r#"[["push",1],["push",2],["make_array",2],["inspect_arr"],["stop"]]"#,
        None,
        $optimize,
      );
    }
    #[test]
    fn object_inspection() {
      crate::runner::run_case(
        "object_inspection",
        r#"[["push","id"],["push",12],["make_obj",1],["inspect_obj"],["stop"]]"#,
        None,
        $optimize,
      );
    }
  };
}
pub(crate) use cases;
