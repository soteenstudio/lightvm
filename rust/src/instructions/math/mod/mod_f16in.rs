use half::f16;
pub fn mod_f16in(a: u16, b: u16) -> u16 {
  let val_a = f16::from_bits(a);
  let val_b = f16::from_bits(b);
  let result = val_a % val_b;
  result.to_bits()
}
