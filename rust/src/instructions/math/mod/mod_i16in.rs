#[inline(always)]
pub fn mod_i16in(a: i16, b: i16) -> i16 {
  if b == 0 {
    return 0;
  }
  a.wrapping_rem(b)
}
