#[path = "../workloads.rs"]
mod workloads;

fn main() {
  workloads::run_target(5, true);
}
