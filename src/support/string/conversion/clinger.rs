#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;
use crate::support::{
  float::f128::F128,
  traits::float::{Float, FloatBits}
};

pub trait Clinger: FloatBits {
  type ClingerFloatType: Float;

  const EXACT_POWERS_OF_TEN: i32;
  const DIGITS_IN_MANTISSA: i32;
  const MAX_EXACT_INT: Self::ClingerFloatType;

  fn get_power_of_ten_slice<'a>() -> &'a [Self::ClingerFloatType];
  fn mantissa_to_float(mantissa: Self::StorageType) -> Self::ClingerFloatType;
  fn from_clinger_float(v: Self::ClingerFloatType) -> Self;
}

impl Clinger for f32 {
  type ClingerFloatType = f32;

  const EXACT_POWERS_OF_TEN: i32 = 10;
  const DIGITS_IN_MANTISSA: i32 = 7;
  const MAX_EXACT_INT: f32 = 16777215.0f32;

  #[inline]
  fn get_power_of_ten_slice<'a>() -> &'a [Self::ClingerFloatType] {
    const ARRAY: [f32; 11] =
      [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10];
    &ARRAY
  }

  #[inline]
  fn mantissa_to_float(mantissa: Self::StorageType) -> Self::ClingerFloatType {
    mantissa as f32
  }

  #[inline]
  fn from_clinger_float(v: Self::ClingerFloatType) -> Self {
    v
  }
}

impl Clinger for f64 {
  type ClingerFloatType = f64;

  const EXACT_POWERS_OF_TEN: i32 = 22;
  const DIGITS_IN_MANTISSA: i32 = 15;
  const MAX_EXACT_INT: f64 = 9007199254740991.0f64;

  #[inline]
  fn get_power_of_ten_slice<'a>() -> &'a [Self::ClingerFloatType] {
    const ARRAY: [f64; 23] = [
      1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13,
      1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22
    ];
    &ARRAY
  }

  #[inline]
  fn mantissa_to_float(mantissa: Self::StorageType) -> Self::ClingerFloatType {
    mantissa as f64
  }

  #[inline]
  fn from_clinger_float(v: Self::ClingerFloatType) -> Self {
    v
  }
}

impl Clinger for F128 {
  type ClingerFloatType = F128;

  const EXACT_POWERS_OF_TEN: i32 = 48;
  const DIGITS_IN_MANTISSA: i32 = 33;
  const MAX_EXACT_INT: F128 = F128(10384593717069655257060992658440191.0f128);

  #[inline]
  fn get_power_of_ten_slice<'a>() -> &'a [Self::ClingerFloatType] {
    const ARRAY: [F128; 49] = [
      F128(1e0),
      F128(1e1),
      F128(1e2),
      F128(1e3),
      F128(1e4),
      F128(1e5),
      F128(1e6),
      F128(1e7),
      F128(1e8),
      F128(1e9),
      F128(1e10),
      F128(1e11),
      F128(1e12),
      F128(1e13),
      F128(1e14),
      F128(1e15),
      F128(1e16),
      F128(1e17),
      F128(1e18),
      F128(1e19),
      F128(1e20),
      F128(1e21),
      F128(1e22),
      F128(1e23),
      F128(1e24),
      F128(1e25),
      F128(1e26),
      F128(1e27),
      F128(1e28),
      F128(1e29),
      F128(1e30),
      F128(1e31),
      F128(1e32),
      F128(1e33),
      F128(1e34),
      F128(1e35),
      F128(1e36),
      F128(1e37),
      F128(1e38),
      F128(1e39),
      F128(1e40),
      F128(1e41),
      F128(1e42),
      F128(1e43),
      F128(1e44),
      F128(1e45),
      F128(1e46),
      F128(1e47),
      F128(1e48)
    ];
    &ARRAY
  }

  #[inline]
  fn mantissa_to_float(mantissa: Self::StorageType) -> Self::ClingerFloatType {
    let low = mantissa as u64;
    let high = (mantissa >> 64) as u64;
    F128(high as f128 * 18446744073709551616.0f128 + low as f128)
  }

  #[inline]
  fn from_clinger_float(v: Self::ClingerFloatType) -> Self {
    v
  }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Clinger for F80 {
  type ClingerFloatType = f64;

  const EXACT_POWERS_OF_TEN: i32 = 27;
  const DIGITS_IN_MANTISSA: i32 = 21;
  const MAX_EXACT_INT: f64 = 18446744073709551615.0f64;

  #[inline]
  fn get_power_of_ten_slice<'a>() -> &'a [Self::ClingerFloatType] {
    const ARRAY: [f64; 28] = [
      1e0f64, 1e1f64, 1e2f64, 1e3f64, 1e4f64, 1e5f64, 1e6f64, 1e7f64, 1e8f64,
      1e9f64, 1e10f64, 1e11f64, 1e12f64, 1e13f64, 1e14f64, 1e15f64, 1e16f64,
      1e17f64, 1e18f64, 1e19f64, 1e20f64, 1e21f64, 1e22f64, 1e23f64, 1e24f64,
      1e25f64, 1e26f64, 1e27f64
    ];
    &ARRAY
  }

  #[inline]
  fn mantissa_to_float(mantissa: Self::StorageType) -> Self::ClingerFloatType {
    mantissa as f64
  }

  #[inline]
  fn from_clinger_float(v: Self::ClingerFloatType) -> Self {
    F80::from_f64(v)
  }
}
