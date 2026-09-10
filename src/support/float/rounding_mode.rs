use crate::arch::fenv;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Rounding {
  ToNearest,
  Downward,
  Upward,
  TowardZero
}

#[inline]
pub fn get_rounding() -> Rounding {
  let result = fenv::fegetround();
  if fenv::FE_DOWNWARD == result {
    return Rounding::Downward;
  }
  if fenv::FE_UPWARD == result {
    return Rounding::Upward;
  }
  if fenv::FE_TOWARDZERO == result {
    return Rounding::TowardZero;
  }
  Rounding::ToNearest
}
