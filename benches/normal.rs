#[path = "../rust/benches/normal/add_bench.rs"]
mod raw_add_bench;
#[path = "../rust/benches/normal/assign_bench.rs"]
mod raw_assign_bench;
#[path = "../rust/benches/normal/concat_bench.rs"]
mod raw_concat_bench;
#[path = "../rust/benches/normal/dead_code_bench.rs"]
mod raw_dead_code_bench;
#[path = "../rust/benches/normal/function_call_bench.rs"]
mod raw_function_call_bench;
#[path = "../rust/benches/normal/io_bench.rs"]
mod raw_io_bench;
#[path = "../rust/benches/runner.rs"]
mod runner;

mod add_bench {
  use crate::raw_add_bench::cases;
  cases!(false);
}
mod assign_bench {
  use crate::raw_assign_bench::cases;
  cases!(false);
}
mod concat_bench {
  use crate::raw_concat_bench::cases;
  cases!(false);
}
mod dead_code_bench {
  use crate::raw_dead_code_bench::cases;
  cases!(false);
}
mod function_call_bench {
  use crate::raw_function_call_bench::cases;
  cases!(false);
}
mod io_bench {
  use crate::raw_io_bench::cases;
  cases!(false);
}
