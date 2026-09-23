#[path = "../workloads.rs"]
mod workloads;

fn main() {
  workloads::run_target(3, true);
}
