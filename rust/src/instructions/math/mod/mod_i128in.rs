#[inline(always)]
pub fn mod_i128in(a: i128, b: i128) -> i128 {
  if b == 0 {
    return 0;
  }
  a.wrapping_rem(b)
}
