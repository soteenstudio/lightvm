#[inline(always)]
pub fn mod_i32in(a: i32, b: i32) -> i32 {
  if b == 0 {
    return 0;
  }
  a.wrapping_rem(b)
}
