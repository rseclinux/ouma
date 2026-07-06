use {
  crate::support::{
    float::Sign,
    traits::float::{Float, FloatBits, FloatType}
  },
  core::{
    arch::asm,
    ops::{Add, Div, Mul, Rem, Sub}
  },
  num_traits::{
    ConstOne,
    ConstZero,
    Num,
    One,
    Zero,
    cast::{NumCast, ToPrimitive}
  }
};

#[cfg(target_arch = "x86")]
#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
pub struct F80([u8; 10]);

#[cfg(target_arch = "x86_64")]
#[derive(Copy, Clone, Debug)]
#[repr(C, align(16))]
pub struct F80([u8; 10]);

#[cfg(target_arch = "x86")]
const F80_SIZE: usize = 12;

#[cfg(target_arch = "x86_64")]
const F80_SIZE: usize = 16;

impl F80 {
  const ZERO: Self = Self([00, 00, 00, 00, 00, 00, 00, 00, 00, 00]);
  const ONE: Self =
    Self([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xff, 0x3f]);

  #[inline]
  pub const fn zero() -> Self {
    Self::ZERO
  }

  #[inline]
  pub const fn one() -> Self {
    Self::ONE
  }

  #[inline]
  pub fn to_bits(self) -> u128 {
    let data = self.0;
    let mut result: [u8; 16] = [0; 16];
    result[..10].copy_from_slice(&data);
    u128::from_le_bytes(result)
  }

  #[inline]
  pub fn from_bits(v: u128) -> Self {
    debug_assert_eq!(v >> 80, 0, "fp80: upper 48 bits must be zero");

    let mut result: [u8; 10] = [0; 10];
    let bytes = v.to_le_bytes();
    let bytes = bytes.as_slice();
    result.copy_from_slice(&bytes[..10]);
    Self(result)
  }

  pub fn from_be_bytes(_: [u8; F80_SIZE]) -> Self {
    unreachable!()
  }

  #[inline]
  pub fn from_le_bytes(bytes: [u8; F80_SIZE]) -> Self {
    let mut result: [u8; 10] = [0; 10];
    let bytes = bytes.as_slice();
    result.copy_from_slice(&bytes[..10]);
    Self(result)
  }

  #[inline]
  pub fn from_ne_bytes(bytes: [u8; F80_SIZE]) -> Self {
    Self::from_le_bytes(bytes)
  }

  pub fn to_be_bytes(self) -> [u8; F80_SIZE] {
    unreachable!()
  }

  #[inline]
  pub fn to_le_bytes(self) -> [u8; F80_SIZE] {
    let data = self.0;
    let mut result: [u8; F80_SIZE] = [0; F80_SIZE];
    result[..10].copy_from_slice(&data);
    result
  }

  #[inline]
  pub fn to_ne_bytes(self) -> [u8; F80_SIZE] {
    self.to_le_bytes()
  }

  #[inline]
  pub fn from_f64(v: f64) -> Self {
    let mut result: Self = Self::ZERO;
    let result_ptr = result.0.as_mut_ptr();
    let data_ptr = &raw const v;
    unsafe {
      asm!("fldl ({0})",
                "fstpt ({1})",
                in(reg) data_ptr,
                in(reg) result_ptr,
                options(att_syntax, nostack));
    }
    result
  }

  #[inline]
  pub fn as_f64(self) -> f64 {
    let mut result: f64 = 0.0;
    let data_ptr = self.0.as_ptr();
    unsafe {
      asm!("fldt ({0})",
                "fstpl ({1})",
                in(reg) data_ptr,
                in(reg) &mut result,
                options(att_syntax, nostack));
    }
    result
  }

  #[inline]
  pub fn from_f32(v: f32) -> Self {
    let mut result: Self = Self::ZERO;
    let result_ptr = result.0.as_mut_ptr();
    let data_ptr = &raw const v;
    unsafe {
      asm!("flds ({0})",
                "fstpt ({1})",
                in(reg) data_ptr,
                in(reg) result_ptr,
                options(att_syntax, nostack));
    }
    result
  }

  #[inline]
  pub fn as_f32(self) -> f32 {
    let mut result: f32 = 0.0;
    let data_ptr = self.0.as_ptr();
    unsafe {
      asm!("fldt ({0})",
                "fstps ({1})",
                in(reg) data_ptr,
                in(reg) &mut result,
                options(att_syntax, nostack));
    }
    result
  }
}

impl core::cmp::PartialEq for F80 {
  #[inline]
  fn eq(
    &self,
    other: &Self
  ) -> bool {
    let lhs = self.0.as_ptr();
    let rhs = other.0.as_ptr();
    let result: u8;
    unsafe {
      asm!(
          "fldt ({0})",
          "fldt ({1})",
          "fucomip %st(1), %st(0)",
          "fstp %st(0)",
          "sete {2}",
          in(reg) rhs,
          in(reg) lhs,
          out(reg_byte) result,
          options(att_syntax, nostack),
      );
    }
    result != 0
  }
}

impl core::cmp::PartialOrd for F80 {
  #[inline]
  fn partial_cmp(
    &self,
    other: &Self
  ) -> Option<core::cmp::Ordering> {
    let lhs = self.0.as_ptr();
    let rhs = other.0.as_ptr();
    let above: u8;
    let below: u8;
    let unordered: u8;
    unsafe {
      asm!(
          "fldt ({1})",
          "fldt ({0})",
          "fucomip %st(1), %st(0)",
          "fstp %st(0)",
          "seta {2}",
          "setb {3}",
          "setp {4}",
          in(reg) lhs,
          in(reg) rhs,
          out(reg_byte) above,
          out(reg_byte) below,
          out(reg_byte) unordered,
          options(att_syntax, nostack),
      );
    }
    if unordered != 0 {
      return None;
    }
    if above != 0 {
      Some(core::cmp::Ordering::Greater)
    } else if below != 0 {
      Some(core::cmp::Ordering::Less)
    } else {
      Some(core::cmp::Ordering::Equal)
    }
  }
}

impl ToPrimitive for F80 {
  #[inline]
  fn to_f64(&self) -> Option<f64> {
    Some(self.as_f64())
  }

  #[inline]
  fn to_i64(&self) -> Option<i64> {
    Some(self.as_f64().to_bits() as i64)
  }

  #[inline]
  fn to_u64(&self) -> Option<u64> {
    Some(self.as_f64().to_bits())
  }
}

impl NumCast for F80 {
  #[inline]
  fn from<T: ToPrimitive>(n: T) -> Option<Self> {
    Some(Self::from_f64(n.to_f64()?))
  }
}

impl core::ops::Add for F80 {
  type Output = Self;

  #[inline]
  fn add(
    self,
    rhs: Self
  ) -> Self::Output {
    let mut result: [u8; 10] = [0; 10];
    let lhs = self.0.as_ptr();
    let rhs = rhs.0.as_ptr();
    let result_ptr = result.as_mut_ptr();

    unsafe {
      asm!(
          "fldt ({0})",
          "fldt ({1})",
          "faddp",
          "fstpt ({2})",
          in(reg) lhs,
          in(reg) rhs,
          in(reg) result_ptr,
          options(nostack, att_syntax)
      );
    };

    Self(result)
  }
}

impl core::ops::AddAssign for F80 {
  #[inline]
  fn add_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.add(rhs);
    *self = result;
  }
}

impl core::ops::Sub for F80 {
  type Output = Self;

  #[inline]
  fn sub(
    self,
    rhs: Self
  ) -> Self::Output {
    let mut result: [u8; 10] = [0; 10];
    let lhs = self.0.as_ptr();
    let rhs = rhs.0.as_ptr();
    let result_ptr = result.as_mut_ptr();

    unsafe {
      asm!(
          "fldt ({0})",
          "fldt ({1})",
          "fsubp",
          "fstpt ({2})",
          in(reg) lhs,
          in(reg) rhs,
          in(reg) result_ptr,
          options(nostack, att_syntax)
      );
    };

    Self(result)
  }
}

impl core::ops::SubAssign for F80 {
  #[inline]
  fn sub_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.sub(rhs);
    *self = result;
  }
}

impl core::ops::Mul for F80 {
  type Output = Self;

  #[inline]
  fn mul(
    self,
    rhs: Self
  ) -> Self::Output {
    let mut result: [u8; 10] = [0; 10];
    let lhs = self.0.as_ptr();
    let rhs = rhs.0.as_ptr();
    let result_ptr = result.as_mut_ptr();

    unsafe {
      asm!(
          "fldt ({0})",
          "fldt ({1})",
          "fmulp",
          "fstpt ({2})",
          in(reg) lhs,
          in(reg) rhs,
          in(reg) result_ptr,
          options(nostack, att_syntax)
      );
    };

    Self(result)
  }
}

impl core::ops::MulAssign for F80 {
  #[inline]
  fn mul_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.mul(rhs);
    *self = result;
  }
}

impl core::ops::Div for F80 {
  type Output = Self;

  #[inline]
  fn div(
    self,
    rhs: Self
  ) -> Self::Output {
    let mut result: [u8; 10] = [0; 10];
    let lhs = self.0.as_ptr();
    let rhs = rhs.0.as_ptr();
    let result_ptr = result.as_mut_ptr();

    unsafe {
      asm!(
          "fldt ({1})",
          "fldt ({0})",
          "fdivp",
          "fstpt ({2})",
          in(reg) lhs,
          in(reg) rhs,
          in(reg) result_ptr,
          options(nostack, att_syntax)
      );
    };

    Self(result)
  }
}

impl core::ops::DivAssign for F80 {
  #[inline]
  fn div_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.div(rhs);
    *self = result;
  }
}

impl core::ops::Rem for F80 {
  type Output = Self;

  #[inline]
  fn rem(
    self,
    rhs: Self
  ) -> Self::Output {
    let mut result: [u8; 10] = [0; 10];
    let lhs = self.0.as_ptr();
    let rhs = rhs.0.as_ptr();
    let result_ptr = result.as_mut_ptr();

    unsafe {
      asm!(
          "fldt ({1})",
          "fldt ({0})",
          "2:",
          "fprem1",
          "fnstsw ax",
          "testb $4, %ah",
          "jnz 2b",
          "fstp %st(1)",
          "fstpt ({2})",
          in(reg) lhs,
          in(reg) rhs,
          in(reg) result_ptr,
          out("ax") _,
          options(nostack, att_syntax),
      );
    };

    Self(result)
  }
}

impl core::ops::RemAssign for F80 {
  #[inline]
  fn rem_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.rem(rhs);
    *self = result;
  }
}

impl core::ops::Add<f64> for F80 {
  type Output = Self;

  #[inline]
  fn add(
    self,
    rhs: f64
  ) -> Self::Output {
    let rhs: Self = Self::from_f64(rhs);
    self + rhs
  }
}

impl core::ops::Sub<f64> for F80 {
  type Output = Self;

  #[inline]
  fn sub(
    self,
    rhs: f64
  ) -> Self::Output {
    let rhs: Self = Self::from_f64(rhs);
    rhs - self
  }
}

impl core::ops::Mul<f64> for F80 {
  type Output = Self;

  #[inline]
  fn mul(
    self,
    rhs: f64
  ) -> Self::Output {
    let rhs: Self = Self::from_f64(rhs);
    self * rhs
  }
}

impl core::ops::Div<f64> for F80 {
  type Output = Self;

  #[inline]
  fn div(
    self,
    rhs: f64
  ) -> Self::Output {
    let rhs: Self = Self::from_f64(rhs);
    self / rhs
  }
}

impl core::ops::Rem<f64> for F80 {
  type Output = Self;

  #[inline]
  fn rem(
    self,
    rhs: f64
  ) -> Self::Output {
    let rhs: Self = Self::from_f64(rhs);
    self % rhs
  }
}

impl core::ops::Neg for F80 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self {
    let mut result: [u8; 10] = [0; 10];
    let src = self.0.as_ptr();
    let dst = result.as_mut_ptr();
    unsafe {
      asm!(
          "fldt ({0})",
          "fchs",
          "fstpt ({1})",
          in(reg) src,
          in(reg) dst,
          options(nostack, att_syntax),
      );
    }
    Self(result)
  }
}

impl Zero for F80 {
  #[inline]
  fn zero() -> Self {
    Self::ZERO
  }

  #[inline]
  fn is_zero(&self) -> bool {
    let exp_sign = u16::from_le_bytes([self.0[8], self.0[9]]);
    let exp = exp_sign & 0x7fff;
    let sig_bytes: [u8; 8] = [
      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
      self.0[6], self.0[7]
    ];
    let sig = u64::from_le_bytes(sig_bytes);
    exp == 0 && sig == 0
  }
}

impl One for F80 {
  #[inline]
  fn one() -> Self {
    Self::ONE
  }
}

impl ConstZero for F80 {
  const ZERO: Self = Self::ZERO;
}

impl ConstOne for F80 {
  const ONE: Self = Self::ONE;
}

impl Num for F80 {
  type FromStrRadixErr = ();

  // TODO: port it from num_traits crate someday...
  fn from_str_radix(
    _str: &str,
    _radix: u32
  ) -> Result<Self, Self::FromStrRadixErr> {
    todo!()
  }
}

impl Float for F80 {
  type ArrayType = [u8; F80_SIZE];

  #[inline]
  fn from_be_bytes(bytes: Self::ArrayType) -> Self {
    Self::from_be_bytes(bytes)
  }

  #[inline]
  fn from_le_bytes(bytes: Self::ArrayType) -> Self {
    Self::from_le_bytes(bytes)
  }

  #[inline]
  fn from_ne_bytes(bytes: Self::ArrayType) -> Self {
    Self::from_ne_bytes(bytes)
  }

  #[inline]
  fn to_be_bytes(self) -> Self::ArrayType {
    self.to_be_bytes()
  }

  #[inline]
  fn to_le_bytes(self) -> Self::ArrayType {
    self.to_le_bytes()
  }

  #[inline]
  fn to_ne_bytes(self) -> Self::ArrayType {
    self.to_ne_bytes()
  }
}

impl FloatBits for F80 {
  type StorageType = u128;

  const FLOAT_TYPE: FloatType = FloatType::IntelExtended;
  const SIGN_LEN: u32 = 1;
  const EXPONENT_LEN: u32 = 15;
  const MANTISSA_LEN: u32 = 64;
  const FRACTION_LEN: u32 = Self::MANTISSA_LEN - 1;
  const DECIMAL_DIG: u32 = 20;

  const MANTISSA_MASK: u128 = mask_trailing_ones!(u128, Self::MANTISSA_LEN);
  const EXPONENT_MASK: u128 =
    mask_trailing_ones!(u128, Self::EXPONENT_LEN) << Self::MANTISSA_LEN;
  const SIGN_MASK: u128 = 1u128 << (Self::EXPONENT_LEN + Self::MANTISSA_LEN);
  const EXP_MANT_MASK: u128 =
    mask_trailing_ones!(u128, Self::EXPONENT_LEN + Self::MANTISSA_LEN);
  const FRACTION_MASK: u128 = mask_trailing_ones!(u128, Self::FRACTION_LEN);

  #[inline]
  fn from_bits(v: Self::StorageType) -> Self {
    Self::from_bits(v)
  }

  #[inline]
  fn to_bits(self) -> Self::StorageType {
    self.to_bits()
  }

  #[inline]
  fn get_explicit_mantissa(self) -> Self::StorageType {
    Self::mantissa_bits(self)
  }

  #[inline]
  fn min_value(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::mant_msb(), Self::EXP_MIN))
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
      Self::MANT_BITS_ALL_ONES ^ Self::mant_msb(),
      Self::EXP_SUBNORMAL
    ))
  }

  #[inline]
  fn inf(sign: Sign) -> Self {
    Self::from_bits(Self::encode(sign, Self::mant_msb(), Self::EXP_INF))
  }

  #[inline]
  fn nan(
    sign: Sign,
    v: Self::StorageType
  ) -> Self {
    Self::from_bits(Self::encode(
      sign,
      Self::mant_msb() | (Self::mant_msb() >> 1u32) | v,
      Self::EXP_INF
    ))
  }

  #[inline]
  fn signaling_nan(
    sign: Sign,
    v: Self::StorageType
  ) -> Self {
    let v =
      if v == Self::StorageType::zero() { Self::mant_msb() >> 2u32 } else { v };
    Self::from_bits(Self::encode(sign, Self::mant_msb() | v, Self::EXP_INF))
  }

  #[inline]
  fn is_nan(self) -> bool {
    if Self::exponent_bits(self) == Self::encode_exponent(Self::EXP_INF) {
      !self.is_inf()
    } else if Self::exponent_bits(self) !=
      Self::encode_exponent(Self::EXP_SUBNORMAL)
    {
      Self::mantissa_bits(self) & Self::mant_msb() == Self::StorageType::zero()
    } else {
      false
    }
  }

  #[inline]
  fn is_quiet_nan(self) -> bool {
    Self::exp_mant_bits(self) >=
      Self::encode_mant_exp(
        Self::mant_msb() | Self::mant_msb() >> 1,
        Self::EXP_INF
      )
  }

  #[inline]
  fn is_signaling_nan(self) -> bool {
    Self::is_nan(self) && !Self::is_quiet_nan(self)
  }

  #[inline]
  fn is_inf(self) -> bool {
    Self::exp_mant_bits(self) ==
      Self::encode_mant_exp(Self::mant_msb(), Self::EXP_INF)
  }

  #[inline]
  fn is_finite(self) -> bool {
    !Self::is_inf(self) && !Self::is_nan(self)
  }

  #[inline]
  fn is_subnormal(self) -> bool {
    Self::exponent_bits(self) == Self::encode_exponent(Self::EXP_SUBNORMAL)
  }

  #[inline]
  fn is_normal(self) -> bool {
    let exp = Self::exponent_bits(self);
    if exp == Self::encode_exponent(Self::EXP_SUBNORMAL) ||
      exp == Self::encode_exponent(Self::EXP_INF)
    {
      return false;
    }
    Self::to_bits(self) & (1u128 << Self::FRACTION_LEN) != 0
  }

  #[inline]
  fn set_implicit_bit(
    self,
    v: bool
  ) -> Self {
    let explicit_bit_mask: Self::StorageType =
      Self::StorageType::one() << Self::FRACTION_LEN;
    let mut bits = Self::to_bits(self);
    let implicit_bit_set =
      (bits & explicit_bit_mask) != Self::StorageType::zero();
    if implicit_bit_set != v {
      bits = bits ^ explicit_bit_mask;
    }
    Self::from_bits(bits)
  }
}
