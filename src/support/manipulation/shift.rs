#[inline(always)]
pub fn lowest_set_bit(mask: usize) -> usize {
  ((mask - 1) & mask) ^ mask
}

#[inline(always)]
pub fn shiftout(
  x: usize,
  mask: usize
) -> usize {
  (x & mask) / lowest_set_bit(mask)
}

#[inline(always)]
pub fn shiftin(
  x: usize,
  mask: usize
) -> usize {
  x * lowest_set_bit(mask)
}
