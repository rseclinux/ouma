// This code has been ported from Google's library called double-conversion
//
// https://github.com/google/double-conversion/blob/master/double-conversion/bignum-dtoa.cc
//

use {
  crate::support::{
    float::{f128::F128, rounding_mode::Rounding},
    traits::float::FloatBits
  },
  bnum::{cast::CastFrom, prelude::*},
  core::ascii,
  num_traits::{ConstOne, ConstZero}
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;

// Those widths are calculated using the following formula:
// F::EXP_MAX + 1 + log2(2-2.pow(F::MANTISSA_LEN - 1))
type BnumF32 = bnum::t!(U256p);
type BnumF64 = bnum::t!(U1088p);
type BnumF128 = bnum::t!(U16504p);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type BnumF80 = t!(U16448p);

pub trait DragonInt:
  Copy
  + Default
  + PartialEq
  + PartialOrd
  + Ord
  + core::ops::Add<Output = Self>
  + core::ops::AddAssign
  + core::ops::Mul<Output = Self>
  + core::ops::MulAssign
  + core::ops::Div<Output = Self>
  + core::ops::DivAssign
  + core::ops::Rem<Output = Self>
  + core::ops::Sub<Output = Self>
  + core::ops::SubAssign
  + core::ops::Shl<u32, Output = Self>
  + core::ops::Shr<u32, Output = Self>
  + core::ops::ShlAssign<u32>
  + core::ops::ShrAssign<u32> {
  const ZERO: Self;
  const ONE: Self;
  const TEN: Self;

  #[inline]
  fn mul10(self) -> Self {
    (self << 3u32) + (self << 1u32)
  }

  #[inline]
  fn pow10(n: u32) -> Self {
    let mut result = Self::ONE;
    let mut base = Self::TEN;
    let mut exp = n;
    while exp > 0 {
      if exp & 1 != 0 {
        result = result * base;
      }
      if exp > 1 {
        base = base * base;
      }
      exp >>= 1;
    }
    result
  }
}

pub trait DragonFloat: FloatBits {
  type Bn: DragonInt;
}

impl_dragon_int!(BnumF32);
impl_dragon_int!(BnumF64);
impl_dragon_int!(BnumF128);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_dragon_int!(BnumF80);

impl DragonFloat for f32 {
  type Bn = BnumF32;
}

impl DragonFloat for f64 {
  type Bn = BnumF64;
}

impl DragonFloat for F128 {
  type Bn = BnumF128;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl DragonFloat for F80 {
  type Bn = BnumF80;
}

// Enough buffer size to store minimal subnormal values of long double
const DRAGON_STRING_BUFSZ: usize = 16384 + 128;

pub struct DragonReturn {
  pub digits: [ascii::Char; DRAGON_STRING_BUFSZ],
  pub ndigits: usize,
  pub exponenta: i32
}

impl Default for DragonReturn {
  #[inline]
  fn default() -> Self {
    Self {
      digits: [ascii::Char::Null; DRAGON_STRING_BUFSZ],
      ndigits: 0,
      exponenta: 0
    }
  }
}

#[derive(Clone, Copy, Default, Debug)]
struct ScaledValues<T: DragonInt> {
  pub numerator: T,
  pub denominator: T
}

#[inline]
fn should_round_up<B: DragonInt>(
  numerator: B,
  denominator: B,
  last_digit: u8,
  round: Rounding,
  negative: bool
) -> bool {
  match round {
    | Rounding::TowardZero => false,
    | Rounding::Upward => !negative && numerator != B::ZERO,
    | Rounding::Downward => negative && numerator != B::ZERO,
    | Rounding::ToNearest => {
      match plus_compare(numerator, numerator, denominator) {
        | core::cmp::Ordering::Greater => true,
        | core::cmp::Ordering::Less => false,
        | core::cmp::Ordering::Equal => last_digit % 2 == 1
      }
    }
  }
}

#[inline]
fn estimate_power<F: FloatBits>(exponent: i32) -> i32 {
  // This function estimates log10 of v where v = f*2^e (with e == exponent).
  // Note that 10^floor(log10(v)) <= v, but v <= 10^ceil(log10(v)).
  // Note that f is bounded by its container size. Let p = 53 (the double's
  // significand size). Then 2^(p-1) <= f < 2^p.
  //
  // Given that log10(v) == log2(v)/log2(10) and e+(len(f)-1) is quite close
  // to log2(v) the function is simplified to (e+(len(f)-1)/log2(10)).
  // The computed number undershoots by less than 0.631 (when we compute log3
  // and not log10).
  //
  // Optimization: since we only need an approximated result this computation
  // can be performed on 64 bit integers. On x86/x64 architecture the speedup is
  // not really measurable, though.
  //
  // Since we want to avoid overshooting we decrement by 1e10 so that
  // floating-point imprecisions don't affect us.
  //
  // Explanation for v's boundary m+: the computation takes advantage of
  // the fact that 2^(p-1) <= f < 2^p. Boundaries still satisfy this requirement
  // (even for denormals where the delta can be much more important).
  const K1_LOG10: f64 = 0.30102999566398114; // 1/lg(10)

  let estimate =
    ((exponent + F::FRACTION_LEN as i32 - 1) as f64) * K1_LOG10 - 1e-10;
  libm::ceil(estimate) as i32
}

#[inline]
fn compute_scaled_val_pos_exp<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  numerator: &mut S,
  denominator: &mut S
) where
  S: DragonInt {
  *numerator = mantissa;
  *numerator <<= exponent as u32;

  *denominator = S::pow10(est_pow as u32);
}

#[inline]
fn compute_scaled_val_neg_exp_pos_power<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  numerator: &mut S,
  denominator: &mut S
) where
  S: DragonInt {
  *numerator = mantissa;

  *denominator = S::pow10(est_pow as u32);
  *denominator <<= (-exponent) as u32;
}

#[inline]
fn compute_scaled_val_neg_exp_neg_power<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  numerator: &mut S,
  denominator: &mut S
) where
  S: DragonInt {
  let pow10 = S::pow10((-est_pow) as u32);

  *numerator = mantissa * pow10;

  *denominator = S::ONE;
  *denominator <<= (-exponent) as u32;
}

#[inline]
fn compute_initial_scaled_values<S, T>(
  mantissa: T::StorageType,
  exponent: i32,
  est_pow: i32
) -> ScaledValues<S>
where
  S: DragonInt + bnum::cast::CastFrom<T::StorageType>,
  T: FloatBits {
  let mut numerator = S::ZERO;
  let mut denominator = S::ZERO;

  let mantissa_bn = S::cast_from(mantissa);

  if exponent >= 0 {
    compute_scaled_val_pos_exp(
      mantissa_bn,
      exponent,
      est_pow,
      &mut numerator,
      &mut denominator
    );
  } else if est_pow >= 0 {
    compute_scaled_val_neg_exp_pos_power(
      mantissa_bn,
      exponent,
      est_pow,
      &mut numerator,
      &mut denominator
    );
  } else {
    compute_scaled_val_neg_exp_neg_power(
      mantissa_bn,
      exponent,
      est_pow,
      &mut numerator,
      &mut denominator
    );
  }

  ScaledValues { numerator, denominator }
}

#[inline]
fn plus_compare<B: DragonInt>(
  a: B,
  b: B,
  c: B
) -> core::cmp::Ordering {
  if c >= b {
    a.cmp(&(c - b))
  } else {
    // c < b → a + b > c for any a >= 0
    core::cmp::Ordering::Greater
  }
}

#[inline]
fn fixup_multiply10<B: DragonInt>(
  est_pow: i32,
  is_even: bool,
  scaled: &mut ScaledValues<B>
) -> i32 {
  let in_range = if is_even {
    scaled.numerator >= scaled.denominator
  } else {
    scaled.numerator > scaled.denominator
  };

  if in_range {
    est_pow + 1
  } else {
    scaled.numerator = scaled.numerator.mul10();
    est_pow
  }
}

#[inline]
fn generate_counted_digits<B: DragonInt>(
  count: usize,
  result: &mut DragonReturn,
  scaled: &mut ScaledValues<B>,
  round: Rounding,
  negative: bool
) where
  u8: bnum::cast::CastFrom<B> {
  if count <= 0 {
    result.ndigits = 0;
    return;
  }

  for i in 0..(count - 1) {
    let digit = u8::cast_from(scaled.numerator / scaled.denominator);
    debug_assert!(digit <= 9);
    result.digits[i] =
      ascii::Char::from_u8(b'0' + digit).unwrap_or(ascii::Char::Null);
    scaled.numerator = scaled.numerator % scaled.denominator;
    scaled.numerator = scaled.numerator.mul10();
  }

  let mut digit = u8::cast_from(scaled.numerator / scaled.denominator);
  scaled.numerator = scaled.numerator % scaled.denominator;
  if should_round_up(
    scaled.numerator,
    scaled.denominator,
    digit,
    round,
    negative
  ) {
    digit += 1;
  }

  debug_assert!(digit <= 10);
  result.digits[count - 1] =
    ascii::Char::from_u8(b'0' + digit).unwrap_or(ascii::Char::Null);

  for i in (1..count).rev() {
    if result.digits[i] != ascii::Char::Colon {
      break;
    }
    result.digits[i] = ascii::Char::Digit0;
    result.digits[i - 1] =
      ascii::Char::from_u8(result.digits[i - 1].to_u8() + 1)
        .unwrap_or(ascii::Char::Null);
  }

  if result.digits[0] == ascii::Char::Colon {
    result.digits[0] = ascii::Char::Digit1;
    result.exponenta += 1;
  }

  result.ndigits = count;
}

#[inline]
fn generate_fixed_digits<B: DragonInt>(
  requested: i32,
  result: &mut DragonReturn,
  scaled: &mut ScaledValues<B>,
  round: Rounding,
  negative: bool
) where
  u8: bnum::cast::CastFrom<B> {
  if -(result.exponenta) > requested {
    result.exponenta = -requested;
    result.ndigits = 0;
    return;
  } else if -(result.exponenta) == requested {
    scaled.denominator = scaled.denominator.mul10();

    if plus_compare(scaled.numerator, scaled.numerator, scaled.denominator)
      .is_ge()
    {
      result.digits[0] = ascii::Char::Digit1;
      result.ndigits = 1;
      result.exponenta += 1;
    } else {
      result.ndigits = 0;
    }
    return;
  } else {
    let needed = (result.exponenta + requested) as usize;
    generate_counted_digits(needed, result, scaled, round, negative);
  }
}

#[inline]
pub fn format_dragon4<T: FloatBits + DragonFloat>(
  value: T,
  precision: i32,
  round: Rounding
) -> DragonReturn
where
  T::Bn: DragonInt + bnum::cast::CastFrom<T::StorageType>,
  u8: bnum::cast::CastFrom<T::Bn> {
  let mantissa = value.get_explicit_mantissa();
  let exponent = value.get_explicit_exponent() - T::FRACTION_LEN as i32;

  let is_even = (mantissa & T::StorageType::ONE) == T::StorageType::ZERO;
  let est_power = estimate_power::<T>(exponent);

  let mut scaled =
    compute_initial_scaled_values::<T::Bn, T>(mantissa, exponent, est_power);

  let mut result = DragonReturn::default();
  result.exponenta = fixup_multiply10(est_power, is_even, &mut scaled);

  generate_fixed_digits(
    precision,
    &mut result,
    &mut scaled,
    round,
    value.is_sign_negative()
  );

  result
}

#[inline]
pub fn format_dragon4_exp<T: FloatBits + DragonFloat>(
  value: T,
  precision: i32,
  round: Rounding
) -> DragonReturn
where
  T::Bn: DragonInt + bnum::cast::CastFrom<T::StorageType>,
  u8: bnum::cast::CastFrom<T::Bn> {
  let mantissa = value.get_explicit_mantissa();
  let exponent = value.get_explicit_exponent() - T::FRACTION_LEN as i32;

  let is_even = (mantissa & T::StorageType::ONE) == T::StorageType::ZERO;
  let est_power = estimate_power::<T>(exponent);

  let mut scaled =
    compute_initial_scaled_values::<T::Bn, T>(mantissa, exponent, est_power);

  let mut result = DragonReturn::default();
  result.exponenta = fixup_multiply10(est_power, is_even, &mut scaled);

  if mantissa == T::StorageType::ZERO {
    result.exponenta = 1;
  }

  generate_counted_digits(
    precision as usize,
    &mut result,
    &mut scaled,
    round,
    value.is_sign_negative()
  );

  result
}
