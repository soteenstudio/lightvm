use lightvm::{
  LightVM,
  types::{
    capability::Capability,
    runtime_config::RuntimeConfig,
    value::{RunOptions, Value},
    vmconfig::VmConfig,
  },
};

pub struct Workload {
  name: &'static str,
  raw: &'static str,
  export: Option<&'static str>,
  args: &'static [i32],
}

const WORKLOADS: [Workload; 18] = [
  Workload {
    name: "integer_order_totals",
    raw: r#"[
      ["push", 3],
      ["push", 120],
      ["mul", "int"],
      ["push", 2],
      ["push", 45],
      ["mul", "int"],
      ["add", "int"],
      ["push", 15],
      ["add", "int"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "price_and_tax",
    raw: r#"[
      ["push", 19.75],
      ["to_double"],
      ["push", 3],
      ["to_double"],
      ["mul", "dbl"],
      ["push", 0.08],
      ["to_double"],
      ["mul", "dbl"],
      ["push", 19.75],
      ["to_double"],
      ["push", 3],
      ["to_double"],
      ["mul", "dbl"],
      ["add", "dbl"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "conversion_arithmetic_pipeline",
    raw: r#"[
      ["push", "24"],
      ["to_integer"],
      ["push", 6],
      ["mul", "int"],
      ["to_double"],
      ["push", 2.0],
      ["to_double"],
      ["div", "dbl"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "shopping_cart_quantities",
    raw: r#"[
      ["val", "apples"],
      ["val", "oranges"],
      ["push", 2],
      ["set", "apples"],
      ["push", 3],
      ["set", "oranges"],
      ["get", "apples"],
      ["push", 4],
      ["add", "int"],
      ["set", "apples"],
      ["get", "apples"],
      ["get", "oranges"],
      ["add", "int"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "account_balance_updates",
    raw: r#"[
      ["val", "balance"],
      ["push", 500],
      ["set", "balance"],
      ["get", "balance"],
      ["push", 75],
      ["sub", "int"],
      ["set", "balance"],
      ["get", "balance"],
      ["push", 20],
      ["add", "int"],
      ["set", "balance"],
      ["get", "balance"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "application_state_updates",
    raw: r#"[
      ["val", "requests"],
      ["val", "errors"],
      ["val", "active"],
      ["push", 12],
      ["set", "requests"],
      ["push", 2],
      ["set", "errors"],
      ["push", 5],
      ["set", "active"],
      ["get", "requests"],
      ["push", 1],
      ["add", "int"],
      ["set", "requests"],
      ["get", "active"],
      ["push", 1],
      ["sub", "int"],
      ["set", "active"],
      ["get", "requests"],
      ["get", "errors"],
      ["sub", "int"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "user_facing_message",
    raw: r#"[
      ["push", "Hello, "],
      ["push", "Taylor"],
      ["concat"],
      ["push", "! Your order is ready."],
      ["concat"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "structured_status_text",
    raw: r#"[
      ["push", "status="],
      ["push", "ready"],
      ["concat"],
      ["push", "; items="],
      ["concat"],
      ["push", 3],
      ["to_string"],
      ["concat"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "multi_step_string_assembly",
    raw: r#"[
      ["push", "Invoice "],
      ["push", "A-104"],
      ["concat"],
      ["push", " / "],
      ["concat"],
      ["push", "September"],
      ["concat"],
      ["push", " / paid"],
      ["concat"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "conditional_unreachable_work",
    raw: r#"[
      ["push", false],
      ["if_false", 6],
      ["push", 999],
      ["push", 888],
      ["mul", "int"],
      ["stop"],
      ["push", 42],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "jump_heavy_workflow",
    raw: r#"[
      ["jump", 2],
      ["push", 999],
      ["jump", 4],
      ["push", 999],
      ["jump", 6],
      ["push", 999],
      ["push", 10],
      ["push", 20],
      ["add", "int"],
      ["push", 2],
      ["mul", "int"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "redundant_intermediate_calculations",
    raw: r#"[
      ["push", 5],
      ["push", 8],
      ["add", "int"],
      ["push", 4],
      ["mul", "int"],
      ["push", 2],
      ["push", 3],
      ["add", "int"],
      ["sub", "int"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "exported_two_argument_calculation",
    raw: r#"[
      ["jump", 6],
      ["func", "sum", 2, 2, 5, "left", "right"],
      ["get", "left"],
      ["get", "right"],
      ["add", "int"],
      ["return"],
      ["stop"],
      ["export", "sum"]
    ]"#,
    export: Some("sum"),
    args: &[12, 30],
  },
  Workload {
    name: "exported_business_calculation",
    raw: r#"[
      ["jump", 10],
      ["func", "net", 3, 2, 9, "price", "quantity", "discount"],
      ["get", "price"],
      ["get", "quantity"],
      ["mul", "int"],
      ["get", "discount"],
      ["sub", "int"],
      ["push", 5],
      ["add", "int"],
      ["return"],
      ["stop"],
      ["export", "net"]
    ]"#,
    export: Some("net"),
    args: &[25, 4, 10],
  },
  Workload {
    name: "sequential_exported_workflow",
    raw: r#"[
      ["jump", 12],
      ["func", "workflow", 1, 2, 6, "input"],
      ["get", "input"],
      ["call", "adjust", 1],
      ["push", 2],
      ["mul", "int"],
      ["return"],
      ["func", "adjust", 1, 8, 11, "amount"],
      ["get", "amount"],
      ["push", 3],
      ["add", "int"],
      ["return"],
      ["stop"],
      ["export", "workflow"]
    ]"#,
    export: Some("workflow"),
    args: &[8],
  },
  Workload {
    name: "array_construction_index_access",
    raw: r#"[
      ["push", 7],
      ["push", 11],
      ["push", 19],
      ["make_array", 3],
      ["push", 1],
      ["access_index"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "object_construction_property_access",
    raw: r#"[
      ["push", "name"],
      ["push", "lamp"],
      ["push", "price"],
      ["push", 25],
      ["make_obj", 2],
      ["access", "price"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
  Workload {
    name: "collection_inspection_length",
    raw: r#"[
      ["push", [7, 11, 19]],
      ["inspect_arr"],
      ["length"],
      ["return"]
    ]"#,
    export: None,
    args: &[],
  },
];

fn config(nightly: bool) -> VmConfig {
  VmConfig {
    caps: vec![Capability::Control, Capability::Debug, Capability::Observe],
    runtime_config: nightly.then_some(RuntimeConfig { nightly: true }),
    ..Default::default()
  }
}

fn run_case(workload: &Workload, optimized: bool) {
  let nightly = workload.export.is_some();
  let bytecode = if optimized {
    let optimizer = LightVM::new(config(nightly));
    Some(optimizer.tools().optimize_bytecode(workload.raw))
  } else {
    None
  };
  let mode = if optimized { "optimize" } else { "normal" };
  let benchmark = LightVM::new(config(nightly))
    .tools()
    .bench(&format!("{}_{}", workload.name, mode))
    .expect("benchmark requires debug capability");
  if let Some(name) = workload.export {
    let args: Vec<Value> = workload.args.iter().copied().map(Value::from).collect();
    benchmark.run(
      || {
        let mut vm = LightVM::new(config(nightly));
        if let Some(ref optimized_bytecode) = bytecode {
          vm.load(optimized_bytecode.clone());
        } else {
          vm.load(workload.raw);
        }
        let function = vm.export(name.to_string());
        let result = function.call(&mut vm, args.clone());
        assert!(result.is_number(), "{} must return a number", workload.name);
        (vm, function)
      },
      // Timed operation: ExportedHandle::call(...).
      |(vm, function)| function.call(vm, args.clone()),
    );
  } else {
    benchmark.run(
      || {
        let mut vm = LightVM::new(config(nightly));
        if let Some(ref optimized_bytecode) = bytecode {
          vm.load(optimized_bytecode.clone());
        } else {
          vm.load(workload.raw);
        }
        let result = vm.run(Some(RunOptions {
          capture_return: true,
          ..Default::default()
        }));
        assert!(
          result.contains("\"status\":\"success\"") && result.contains("\"defined\":true"),
          "{} must produce a value: {result}",
          workload.name
        );
        vm
      },
      // Timed operation: vm.run(None).
      |vm| vm.run(None),
    );
  }
}

pub fn run_target(target: usize, optimized: bool) {
  for workload in &WORKLOADS[target * 3..target * 3 + 3] {
    run_case(workload, optimized);
  }
}
