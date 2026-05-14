#[inline(always)]
pub fn mod_i64in(a: i64, b: i64) -> i64 {
  if b == 0 {
    return 0;
  }
  a.wrapping_rem(b)
}
