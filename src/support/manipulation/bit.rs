use core::{cmp, mem};

#[inline(always)]
pub fn bit(n: usize) -> u32 {
  if n >= (mem::size_of::<u32>() * 8) {
    0
  } else {
    1 << (n & ((mem::size_of::<u32>() * 8) - 1))
  }
}

#[inline(always)]
pub fn bits(
  m: usize,
  n: usize
) -> u32 {
  (bit(cmp::max(m, n).wrapping_add(1)).wrapping_sub(1)) ^
    (bit(cmp::min(m, n)).wrapping_sub(1))
}
