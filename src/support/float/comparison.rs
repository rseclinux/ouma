use crate::support::traits::float::FloatBits;

#[inline]
pub fn eq<F: FloatBits>(
  lhs: F,
  rhs: F
) -> bool {
  if lhs.is_nan() || rhs.is_nan() {
    return false;
  }
  if lhs.is_zero() && rhs.is_zero() {
    return true;
  }
  lhs.as_uint_value() == rhs.as_uint_value()
}

#[inline]
pub fn lt<F: FloatBits>(
  lhs: F,
  rhs: F
) -> bool {
  if lhs.is_nan() || rhs.is_nan() {
    return false;
  }
  if lhs.is_zero() && rhs.is_zero() {
    return false;
  }
  if lhs.is_sign_negative() && rhs.is_sign_positive() {
    return true;
  }
  if lhs.is_sign_positive() && rhs.is_sign_negative() {
    return false;
  }
  if lhs.is_sign_negative() {
    return lhs.as_uint_value() > rhs.as_uint_value();
  }
  lhs.as_uint_value() < rhs.as_uint_value()
}

#[inline]
pub fn gt<F: FloatBits>(
  lhs: F,
  rhs: F
) -> bool {
  lt(rhs, lhs)
}

#[inline]
pub fn le<F: FloatBits>(
  lhs: F,
  rhs: F
) -> bool {
  lt(lhs, rhs) || eq(lhs, rhs)
}

#[inline]
pub fn ge<F: FloatBits>(
  lhs: F,
  rhs: F
) -> bool {
  gt(lhs, rhs) || eq(lhs, rhs)
}
