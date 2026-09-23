#[path = "../workloads.rs"]
mod workloads;

fn main() {
  workloads::run_target(2, false);
}
