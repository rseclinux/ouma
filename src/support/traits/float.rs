// Ported from LLVM libc.
//
// See: https://github.com/llvm/llvm-project/blob/10e7761cac92ee695d2a74a813ad3ebba4e649c0/libc/src/__support/FPUtil/FPBits.h
//

use {
  crate::support::float::{Sign, f128::F128},
  core::{num::FpCategory, ops::Neg},
  num_traits::{ConstOne, ConstZero, Num, NumAssign, NumCast, One, Zero}
};

#[derive(PartialEq)]
pub enum FloatType {
  IEEE754Binary16,
  IEEE754Binary32,
  IEEE754Binary64,
  IEEE754Binary128,
  IntelExtended
}

pub trait Float:
  Num
  + Copy
  + NumCast
  + NumAssign
  + PartialOrd
  + Neg<Output = Self>
  + ConstZero
  + ConstOne {
  type ArrayType;

  fn from_be_bytes(bytes: Self::ArrayType) -> Self;
  fn from_le_bytes(bytes: Self::ArrayType) -> Self;
  fn from_ne_bytes(bytes: Self::ArrayType) -> Self;

  fn to_be_bytes(self) -> Self::ArrayType;
  fn to_le_bytes(self) -> Self::ArrayType;
  fn to_ne_bytes(self) -> Self::ArrayType;
}

pub trait FloatBits: Float {
  type StorageType: Copy
    + PartialEq
    + PartialOrd
    + From<u32>
    + core::ops::BitOr<Output = Self::StorageType>
    + core::ops::BitXor<Output = Self::StorageType>
    + core::ops::BitAnd<Output = Self::StorageType>
    + core::ops::Shl<Output = Self::StorageType>
    + core::ops::Shl<u32, Output = Self::StorageType>
    + core::ops::Shr<Output = Self::StorageType>
    + core::ops::Shr<u32, Output = Self::StorageType>
    + num_traits::PrimInt
    + num_traits::Zero
    + num_traits::One
    + num_traits::ConstZero
    + num_traits::ConstOne
    + num_traits::AsPrimitive<u64>;

  const FLOAT_TYPE: FloatType;
  const EXPONENT_LEN: u32;
  const MANTISSA_LEN: u32;
  const FRACTION_LEN: u32;
  const DECIMAL_DIG: u32;

  const MANTISSA_MASK: Self::StorageType;
  const EXPONENT_MASK: Self::StorageType;
  const SIGN_MASK: Self::StorageType;
  const EXP_MANT_MASK: Self::StorageType;
  const FRACTION_MASK: Self::StorageType;

  const SIGN_LEN: u32 = 1;
  const TOTAL_LEN: u32 =
    Self::SIGN_LEN + Self::EXPONENT_LEN + Self::MANTISSA_LEN;
  const EXPONENT_BIAS: u32 = (1u32 << (Self::EXPONENT_LEN - 1)) - 1;
  const STORAGE_LEN: u32 = core::mem::size_of::<Self::StorageType>() as u32 * 8;

  const EXP_ZERO: u32 = Self::EXPONENT_BIAS;
  const EXP_MIN: u32 = 1;
  const EXP_MAX: u32 = (1u32 << Self::EXPONENT_LEN) - 2;
  const EXP_INF: u32 = (1u32 << Self::EXPONENT_LEN) - 1;
  const EXP_SUBNORMAL: u32 = 0;
  const MAX_BIASED_EXPONENT: i32 = (1 << Self::EXPONENT_LEN) - 1;

  const MANT_ZERO: Self::StorageType = Self::StorageType::ZERO;
  const MANT_LSB: Self::StorageType = Self::StorageType::ONE;
  const MANT_BITS_ALL_ONES: Self::StorageType = Self::MANTISSA_MASK;

  fn from_bits(v: Self::StorageType) -> Self;
  fn to_bits(self) -> Self::StorageType;

  #[inline]
  fn mant_msb() -> Self::StorageType {
    <Self::StorageType as From<u32>>::from(1u32) << (Self::MANTISSA_LEN - 1)
  }

  #[inline]
  fn masked_merge(
    a: Self::StorageType,
    b: Self::StorageType,
    mask: Self::StorageType
  ) -> Self::StorageType {
    a ^ ((a ^ b) & mask)
  }

  #[inline]
  fn encode_exponent(value: u32) -> Self::StorageType {
    (<Self::StorageType as From<u32>>::from(value) << Self::MANTISSA_LEN) &
      Self::EXPONENT_MASK
  }

  #[inline]
  fn encode_mantissa(value: Self::StorageType) -> Self::StorageType {
    value & Self::MANTISSA_MASK
  }

  #[inline]
  fn encode_mant_exp(
    mantissa: Self::StorageType,
    exponent: u32
  ) -> Self::StorageType {
    Self::encode_exponent(exponent) | Self::encode_mantissa(mantissa)
  }

  #[inline]
  fn encode(
    sign: Sign,
    mantissa: Self::StorageType,
    exponent: u32
  ) -> Self::StorageType {
    if sign == Sign::Negative {
      return Self::SIGN_MASK | Self::encode_mant_exp(mantissa, exponent);
    }
    Self::encode_mant_exp(mantissa, exponent)
  }

  #[inline]
  fn exponent_bits(self) -> Self::StorageType {
    Self::to_bits(self) & Self::EXPONENT_MASK
  }

  #[inline]
  fn mantissa_bits(self) -> Self::StorageType {
    Self::to_bits(self) & Self::MANTISSA_MASK
  }

  #[inline]
  fn exp_mant_bits(self) -> Self::StorageType {
    Self::to_bits(self) & Self::EXP_MANT_MASK
  }

  #[inline]
  fn get_biased_exponent(self) -> u32 {
    let shifted = Self::exponent_bits(self) >> Self::MANTISSA_LEN;
    <u32 as NumCast>::from(shifted).unwrap_or(Self::EXP_MIN)
  }

  #[inline]
  fn set_biased_exponent(
    mut self,
    biased: u32
  ) -> Self {
    let bits = Self::masked_merge(
      Self::to_bits(self),
      Self::encode_exponent(biased),
      Self::EXPONENT_MASK
    );
    self = Self::from_bits(bits);
    self
  }

  #[inline]
  fn get_exponent(self) -> i32 {
    Self::get_biased_exponent(self) as i32 - Self::EXPONENT_BIAS as i32
  }

  #[inline]
  fn get_explicit_exponent(self) -> i32 {
    if Self::is_zero(&self) {
      0
    } else if Self::is_subnormal(self) {
      1 - Self::EXPONENT_BIAS as i32
    } else {
      Self::get_biased_exponent(self) as i32 - Self::EXPONENT_BIAS as i32
    }
  }

  #[inline]
  fn inf(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::MANT_ZERO, Self::EXP_INF))
  }

  #[inline]
  fn nan(
    sign: Sign,
    v: Self::StorageType
  ) -> Self {
    Self::from_bits(Self::encode(sign, Self::mant_msb() | v, Self::EXP_INF))
  }

  #[inline]
  fn signaling_nan(
    sign: Sign,
    v: Self::StorageType
  ) -> Self {
    let v =
      if v == Self::StorageType::zero() { Self::mant_msb() >> 1u32 } else { v };
    Self::from_bits(Self::encode(sign, v, Self::EXP_INF))
  }

  #[inline]
  fn min_value(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::MANT_ZERO, Self::EXP_MIN))
  }

  #[inline]
  fn max_value(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::MANT_BITS_ALL_ONES, Self::EXP_MAX))
  }

  #[inline]
  fn min_subnormal_value(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::MANT_LSB, Self::EXP_SUBNORMAL))
  }

  #[inline]
  fn max_subnormal_value(sign: Sign) -> Self {
    Self::from_bits(Self::encode(
      sign,
      Self::MANT_BITS_ALL_ONES,
      Self::EXP_SUBNORMAL
    ))
  }

  #[inline]
  fn is_nan(self) -> bool {
    Self::exp_mant_bits(self) >
      Self::encode_mant_exp(Self::MANT_ZERO, Self::EXP_INF)
  }

  #[inline]
  fn is_quiet_nan(self) -> bool {
    Self::exp_mant_bits(self) >=
      Self::encode_mant_exp(Self::mant_msb(), Self::EXP_INF)
  }

  #[inline]
  fn is_signaling_nan(self) -> bool {
    Self::is_nan(self) && !Self::is_quiet_nan(self)
  }

  #[inline]
  fn is_inf(self) -> bool {
    Self::exp_mant_bits(self) ==
      Self::encode_mant_exp(Self::MANT_ZERO, Self::EXP_INF)
  }

  #[inline]
  fn is_finite(self) -> bool {
    Self::exponent_bits(self) != Self::encode_exponent(Self::EXP_INF)
  }

  #[inline]
  fn is_inf_or_nan(self) -> bool {
    !Self::is_finite(self)
  }

  #[inline]
  fn is_normal(self) -> bool {
    Self::is_finite(self) && !Self::is_subnormal(self)
  }

  #[inline]
  fn is_subnormal(self) -> bool {
    Self::exponent_bits(self) == Self::encode_exponent(Self::EXP_SUBNORMAL)
  }

  #[inline]
  fn get_sign(self) -> Sign {
    let bits = Self::to_bits(self);
    if bits & Self::SIGN_MASK != Self::StorageType::zero() {
      Sign::Negative
    } else {
      Sign::Positive
    }
  }

  #[inline]
  fn set_sign(
    mut self,
    sign: Sign
  ) -> Self {
    let mut bits = Self::to_bits(self);
    if sign != Self::get_sign(self) {
      bits = bits ^ Self::SIGN_MASK;
    }
    self = Self::from_bits(bits);
    self
  }

  #[inline]
  fn is_sign_positive(self) -> bool {
    Self::get_sign(self) == Sign::Positive
  }

  #[inline]
  fn is_sign_negative(self) -> bool {
    Self::get_sign(self) == Sign::Negative
  }

  #[inline]
  fn abs(self) -> Self {
    let abs_bits = Self::to_bits(self) & Self::EXP_MANT_MASK;
    Self::from_bits(abs_bits)
  }

  #[inline]
  fn classify(self) -> FpCategory {
    if Self::is_nan(self) {
      FpCategory::Nan
    } else if Self::is_inf(self) {
      FpCategory::Infinite
    } else if Self::exp_mant_bits(self) == Self::StorageType::zero() {
      FpCategory::Zero
    } else if Self::is_subnormal(self) {
      FpCategory::Subnormal
    } else {
      FpCategory::Normal
    }
  }

  #[inline]
  fn get_explicit_mantissa(self) -> Self::StorageType {
    if Self::is_subnormal(self) {
      return Self::mantissa_bits(self);
    }
    (Self::StorageType::one() << Self::MANTISSA_LEN) | Self::mantissa_bits(self)
  }

  #[inline]
  fn set_mantissa(
    mut self,
    v: Self::StorageType
  ) -> Self {
    let bits = Self::masked_merge(Self::to_bits(self), v, Self::FRACTION_MASK);
    self = Self::from_bits(bits);
    self
  }

  #[inline]
  fn set_significand(
    mut self,
    v: Self::StorageType
  ) -> Self {
    let bits = Self::masked_merge(Self::to_bits(self), v, Self::MANTISSA_MASK);
    self = Self::from_bits(bits);
    self
  }

  #[inline]
  fn set_implicit_bit(
    self,
    _: bool
  ) -> Self {
    self
  }

  #[inline]
  fn create_value(
    sign: Sign,
    biased_exp: u32,
    mantissa: Self::StorageType
  ) -> Self {
    Self::from_bits(Self::encode(sign, mantissa, biased_exp))
  }
}

macro_rules! impl_float_repr {
  (
        $float:ty,
        $storage:ty,
        $float_type:expr,
        $exp_len:expr,
        $mantissa_len:expr,
        $fractional_len:expr,
        $decimal_dig:expr,
        $array_type:ty
    ) => {
    impl Float for $float {
      type ArrayType = $array_type;

      fn from_be_bytes(bytes: Self::ArrayType) -> Self {
        <$float>::from_be_bytes(bytes)
      }

      fn from_le_bytes(bytes: Self::ArrayType) -> Self {
        <$float>::from_le_bytes(bytes)
      }

      fn from_ne_bytes(bytes: Self::ArrayType) -> Self {
        <$float>::from_ne_bytes(bytes)
      }

      fn to_be_bytes(self) -> Self::ArrayType {
        <$float>::to_be_bytes(self)
      }

      fn to_le_bytes(self) -> Self::ArrayType {
        <$float>::to_le_bytes(self)
      }

      fn to_ne_bytes(self) -> Self::ArrayType {
        <$float>::to_ne_bytes(self)
      }
    }

    impl FloatBits for $float {
      type StorageType = $storage;
      const FLOAT_TYPE: FloatType = $float_type;
      const EXPONENT_LEN: u32 = $exp_len;
      const FRACTION_LEN: u32 = $fractional_len;
      const MANTISSA_LEN: u32 = $mantissa_len;
      const DECIMAL_DIG: u32 = $decimal_dig;

      const MANTISSA_MASK: $storage =
        mask_trailing_ones!($storage, $mantissa_len);
      const EXPONENT_MASK: $storage =
        mask_trailing_ones!($storage, $exp_len) << $mantissa_len;
      const SIGN_MASK: $storage = 1 << ($exp_len + $mantissa_len);
      const EXP_MANT_MASK: $storage =
        mask_trailing_ones!($storage, $exp_len + $mantissa_len);
      const FRACTION_MASK: $storage =
        mask_trailing_ones!($storage, $fractional_len);

      fn from_bits(v: $storage) -> Self {
        <$float>::from_bits(v as _)
      }
      fn to_bits(self) -> $storage {
        <$float>::to_bits(self) as _
      }
    }
  };
}

impl_float_repr!(f32, u32, FloatType::IEEE754Binary32, 8, 23, 23, 6, [u8; 4]);
impl_float_repr!(f64, u64, FloatType::IEEE754Binary64, 11, 52, 52, 17, [u8; 8]);
impl_float_repr!(
  F128,
  u128,
  FloatType::IEEE754Binary128,
  15,
  112,
  112,
  36,
  [u8; 16]
);
