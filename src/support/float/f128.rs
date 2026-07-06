use {
  core::ops::{Add, Div, Mul, Rem, Sub},
  num_traits::{
    ConstOne,
    ConstZero,
    Num,
    One,
    Zero,
    cast::{NumCast, ToPrimitive}
  }
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F128(pub f128);

impl F128 {
  #[inline]
  pub fn from_bits(v: u128) -> Self {
    Self(f128::from_bits(v))
  }

  #[inline]
  pub fn to_bits(self) -> u128 {
    self.0.to_bits()
  }

  #[inline]
  pub fn from_be_bytes(bytes: [u8; 16]) -> Self {
    Self(f128::from_be_bytes(bytes))
  }

  #[inline]
  pub fn from_le_bytes(bytes: [u8; 16]) -> Self {
    Self(f128::from_le_bytes(bytes))
  }

  #[inline]
  pub fn from_ne_bytes(bytes: [u8; 16]) -> Self {
    Self(f128::from_ne_bytes(bytes))
  }

  #[inline]
  pub fn to_be_bytes(self) -> [u8; 16] {
    self.0.to_be_bytes()
  }

  #[inline]
  pub fn to_le_bytes(self) -> [u8; 16] {
    self.0.to_le_bytes()
  }

  #[inline]
  pub fn to_ne_bytes(self) -> [u8; 16] {
    self.0.to_ne_bytes()
  }
}

impl ConstZero for F128 {
  const ZERO: Self = Self(0.0);
}

impl ConstOne for F128 {
  const ONE: Self = Self(1.0);
}

impl Zero for F128 {
  #[inline]
  fn zero() -> Self {
    Self(f128::from(0.0))
  }

  #[inline]
  fn is_zero(&self) -> bool {
    self.0 == 0.0
  }
}

impl One for F128 {
  #[inline]
  fn one() -> Self {
    Self(f128::from(1.0))
  }
}

impl core::ops::Deref for F128 {
  type Target = f128;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl core::ops::Add for F128 {
  type Output = Self;

  #[inline]
  fn add(
    self,
    rhs: Self
  ) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl core::ops::AddAssign for F128 {
  #[inline]
  fn add_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.add(rhs);
    *self = result;
  }
}

impl core::ops::Sub for F128 {
  type Output = Self;

  #[inline]
  fn sub(
    self,
    rhs: Self
  ) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl core::ops::SubAssign for F128 {
  #[inline]
  fn sub_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.sub(rhs);
    *self = result;
  }
}

impl core::ops::Mul for F128 {
  type Output = Self;

  #[inline]
  fn mul(
    self,
    rhs: Self
  ) -> Self::Output {
    Self(self.0 * rhs.0)
  }
}

impl core::ops::MulAssign for F128 {
  #[inline]
  fn mul_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.mul(rhs);
    *self = result;
  }
}

impl core::ops::Div for F128 {
  type Output = Self;

  #[inline]
  fn div(
    self,
    rhs: Self
  ) -> Self::Output {
    Self(self.0 / rhs.0)
  }
}

impl core::ops::DivAssign for F128 {
  #[inline]
  fn div_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.div(rhs);
    *self = result;
  }
}

impl core::ops::Rem for F128 {
  type Output = Self;

  #[inline]
  fn rem(
    self,
    rhs: Self
  ) -> Self::Output {
    Self(self.0 % rhs.0)
  }
}

impl core::ops::RemAssign for F128 {
  #[inline]
  fn rem_assign(
    &mut self,
    rhs: Self
  ) {
    let result = self.rem(rhs);
    *self = result;
  }
}

impl core::ops::Neg for F128 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self(self.0.neg())
  }
}

impl ToPrimitive for F128 {
  #[inline]
  fn to_i64(&self) -> Option<i64> {
    Some(self.0.to_bits() as u64 as i64)
  }

  #[inline]
  fn to_u64(&self) -> Option<u64> {
    Some(self.0.to_bits() as u64)
  }
}

impl NumCast for F128 {
  #[inline]
  fn from<T: ToPrimitive>(n: T) -> Option<Self> {
    Some(Self(f128::from_bits(n.to_u128()?)))
  }
}

impl Num for F128 {
  type FromStrRadixErr = ();

  // TODO: port it from num_traits crate someday...
  fn from_str_radix(
    _str: &str,
    _radix: u32
  ) -> Result<Self, Self::FromStrRadixErr> {
    todo!()
  }
}
