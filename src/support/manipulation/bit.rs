use core::{cmp, mem};

#[inline(always)]
pub fn bit(n: usize) -> usize {
  if n >= (mem::size_of::<usize>() * 8) {
    0
  } else {
    1 << (n & ((mem::size_of::<usize>() * 8) - 1))
  }
}

#[inline(always)]
pub fn bits(
  m: usize,
  n: usize
) -> usize {
  (bit(cmp::max(m, n).wrapping_add(1)).wrapping_sub(1)) ^
    (bit(cmp::min(m, n)).wrapping_sub(1))
}
