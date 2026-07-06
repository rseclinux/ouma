use crate::support::traits::float::Float;

#[inline]
pub fn multiply_add<T: Float>(
  x: T,
  y: T,
  z: T
) -> T {
  x * y + z
}
