pub fn compute_hot_threshold(stack_len: usize) -> u32 {
  let base_hot: u32 = 10;
  let scale_step: usize = 100;
  let min_hot: u32 = 5;
  let max_hot: u32 = 2000;
  let scale = (stack_len / scale_step).max(1) as u32;
  let raw = base_hot * scale;
  raw.clamp(min_hot, max_hot)
}
