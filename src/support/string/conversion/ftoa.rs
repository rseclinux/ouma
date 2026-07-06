/*
 * Dragon4 floating point to number conversion
 * based off Google's double-decimal bignum-dtoa.cc
 * https://github.com/google/double-conversion/blob/44944accfd5516a094e1e730334764e0a908aaff/double-conversion/bignum-dtoa.cc
 */

use {
  crate::support::{
    float::{Sign, f128::F128, rounding_mode::Rounding},
    traits::float::FloatBits
  },
  bnum::{cast::CastFrom, prelude::*},
  core::{ascii, cmp::Ordering},
  libm::ceil,
  num_traits::{ConstOne, ConstZero}
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;

// Those widths are calculated using the following formula:
// F::EXP_MAX + 1 + log2(2-2.pow(F::MANTISSA_LEN - 1))
pub type BnumF32 = bnum::t!(U256p);
pub type BnumF64 = bnum::t!(U1088p);
pub type BnumF128 = bnum::t!(U16504p);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub type BnumF80 = t!(U16448p);

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

#[derive(Clone, Copy, Default)]
struct ScaledValues<T: DragonInt> {
  pub numerator: T,
  pub denominator: T,
  pub delta_plus: T,
  pub delta_minus: T
}

#[inline]
fn compute_scaled_val_pos_exp<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  need_boundary_deltas: bool,
  numerator: &mut S,
  denominator: &mut S,
  delta_plus: &mut S,
  delta_minus: &mut S
) where
  S: DragonInt {
  *numerator = mantissa;
  *numerator <<= exponent as u32;

  *denominator = S::pow10(est_pow as u32);

  if need_boundary_deltas {
    *denominator <<= 1u32;
    *numerator <<= 1u32;

    *delta_plus = S::ONE;
    *delta_plus <<= exponent as u32;
    *delta_minus = S::ONE;
    *delta_minus <<= exponent as u32;
  }
}

#[inline]
fn compute_scaled_val_neg_exp_pos_power<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  need_boundary_deltas: bool,
  numerator: &mut S,
  denominator: &mut S,
  delta_plus: &mut S,
  delta_minus: &mut S
) where
  S: DragonInt {
  *numerator = mantissa;

  *denominator = S::pow10(est_pow as u32);
  *denominator <<= (-exponent) as u32;

  if need_boundary_deltas {
    *denominator <<= 1u32;
    *numerator <<= 1u32;

    *delta_plus = S::ONE;
    *delta_minus = S::ONE;
  }
}

#[inline]
fn compute_scaled_val_neg_exp_neg_power<S>(
  mantissa: S,
  exponent: i32,
  est_pow: i32,
  need_boundary_deltas: bool,
  numerator: &mut S,
  denominator: &mut S,
  delta_plus: &mut S,
  delta_minus: &mut S
) where
  S: DragonInt {
  let pow10 = S::pow10((-est_pow) as u32);

  *numerator = mantissa * pow10;

  *denominator = S::ONE;
  *denominator <<= (-exponent) as u32;

  if need_boundary_deltas {
    *numerator <<= 1u32;
    *denominator <<= 1u32;

    *delta_plus = pow10;
    *delta_minus = pow10;
  }
}

#[inline]
fn compute_initial_scaled_values<S, T>(
  mantissa: T::StorageType,
  exponent: i32,
  est_pow: i32,
  lower_boundary_is_closer: bool,
  need_boundary_deltas: bool
) -> ScaledValues<S>
where
  S: DragonInt + CastFrom<T::StorageType>,
  T: FloatBits {
  let mut numerator = S::ZERO;
  let mut denominator = S::ZERO;
  let mut delta_plus = S::ZERO;
  let mut delta_minus = S::ZERO;

  let mantissa_bn = S::cast_from(mantissa);

  if exponent >= 0 {
    compute_scaled_val_pos_exp(
      mantissa_bn,
      exponent,
      est_pow,
      need_boundary_deltas,
      &mut numerator,
      &mut denominator,
      &mut delta_plus,
      &mut delta_minus
    );
  } else if est_pow >= 0 {
    compute_scaled_val_neg_exp_pos_power(
      mantissa_bn,
      exponent,
      est_pow,
      need_boundary_deltas,
      &mut numerator,
      &mut denominator,
      &mut delta_plus,
      &mut delta_minus
    );
  } else {
    compute_scaled_val_neg_exp_neg_power(
      mantissa_bn,
      exponent,
      est_pow,
      need_boundary_deltas,
      &mut numerator,
      &mut denominator,
      &mut delta_plus,
      &mut delta_minus
    );
  }

  if need_boundary_deltas && lower_boundary_is_closer {
    numerator <<= 1u32;
    denominator <<= 1u32;
    delta_plus <<= 1u32;
  }

  ScaledValues { numerator, denominator, delta_plus, delta_minus }
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
fn adj_to_legal_range<S: DragonInt>(
  is_even: bool,
  scaled: &mut ScaledValues<S>
) -> bool {
  let cmp =
    plus_compare(scaled.numerator, scaled.delta_plus, scaled.denominator);
  let in_range = if is_even { cmp.is_ge() } else { cmp.is_gt() };
  if !in_range {
    scaled.numerator = scaled.numerator.mul10();
    scaled.delta_minus = scaled.delta_minus.mul10();
    if scaled.delta_minus != scaled.delta_plus {
      scaled.delta_plus = scaled.delta_plus.mul10();
    } else {
      scaled.delta_plus = scaled.delta_minus;
    }
    return true;
  }
  false
}

#[inline]
fn estimate_power<T: FloatBits>(exponent: i32) -> i32 {
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
    ((exponent + T::FRACTION_LEN as i32 - 1) as f64) * K1_LOG10 - 1e-10;
  ceil(estimate) as i32
}

fn dragon4_shortest<T: FloatBits + DragonFloat>(
  value: T,
  buffer: &mut [u8]
) -> (usize, i32)
where
  T::Bn: DragonInt + CastFrom<T::StorageType>,
  u8: CastFrom<T::Bn> {
  let mantissa = value.get_explicit_mantissa();
  let exponent = value.get_explicit_exponent() - T::FRACTION_LEN as i32;
  let is_even: bool = (mantissa & T::StorageType::ONE) == T::StorageType::ZERO;

  let mut est_pow = estimate_power::<T>(exponent);

  let lower_boundary_is_closer = value.get_biased_exponent() == T::EXP_MIN &&
    value.mantissa_bits() == T::StorageType::ZERO;

  // Where the digit (non-string) generation happens
  let mut scaled = compute_initial_scaled_values::<T::Bn, T>(
    mantissa,
    exponent,
    est_pow,
    lower_boundary_is_closer,
    true
  );
  if adj_to_legal_range(is_even, &mut scaled) {
    est_pow -= 1;
  }

  if scaled.delta_minus == scaled.delta_plus {
    scaled.delta_plus = scaled.delta_minus;
  }

  // Generate ASCII chars of digits and store them in buffer
  let mut len = 0usize;
  loop {
    let digit = u8::cast_from(scaled.numerator / scaled.denominator);
    scaled.numerator = scaled.numerator % scaled.denominator;

    buffer[len] =
      ascii::Char::digit(digit).unwrap_or(ascii::Char::Null).to_u8();
    len += 1;

    let in_delta_room_minus = if is_even {
      scaled.numerator.cmp(&scaled.delta_minus).is_le()
    } else {
      scaled.numerator.cmp(&scaled.delta_minus).is_lt()
    };
    let in_delta_room_plus = if is_even {
      plus_compare(scaled.numerator, scaled.delta_plus, scaled.denominator)
        .is_ge()
    } else {
      plus_compare(scaled.numerator, scaled.delta_plus, scaled.denominator)
        .is_gt()
    };

    if !in_delta_room_minus && !in_delta_room_plus {
      scaled.numerator = scaled.numerator.mul10();
      scaled.delta_minus = scaled.delta_minus.mul10();

      if scaled.delta_minus != scaled.delta_plus {
        scaled.delta_plus = scaled.delta_plus.mul10();
      }
    } else if in_delta_room_minus && in_delta_room_plus {
      let cmp =
        plus_compare(scaled.numerator, scaled.numerator, scaled.denominator);
      match cmp {
        | Ordering::Less => {},
        | Ordering::Greater => {
          buffer[len - 1] += 1;
        },
        | Ordering::Equal => {
          if (buffer[len - 1] - b'0') % 2 != 0 {
            buffer[len - 1] += 1;
          }
        },
      }
      break;
    } else if in_delta_room_minus {
      break;
    } else {
      buffer[len - 1] += 1;
      break;
    }
  }

  (len, est_pow)
}

pub struct FtoaReturn {
  pub digits: [ascii::Char; 40],
  pub ndigits: usize,
  pub exponenta: i32
}

impl Default for FtoaReturn {
  #[inline]
  fn default() -> Self {
    Self { digits: [ascii::Char::Null; 40], ndigits: 0, exponenta: 0 }
  }
}

pub fn format_float<T: FloatBits + DragonFloat>(
  value: T,
  precision: i32,
  round: Rounding
) -> FtoaReturn
where
  T::Bn: DragonInt + CastFrom<T::StorageType>,
  u8: CastFrom<T::Bn> {
  let mut buffer = [0u8; 40];
  let mut result = [ascii::Char::Null; 40];

  let (mut written, mut exponenta) = dragon4_shortest::<T>(value, &mut buffer);

  loop {
    if written == 0 {
      return FtoaReturn::default();
    }

    if buffer[written - 1] != b'0' {
      break;
    }
    written -= 1;
  }

  let mut ndigits = written;

  let round = if value.get_sign() == Sign::Negative {
    match round {
      | Rounding::Upward => Rounding::Downward,
      | Rounding::Downward => Rounding::Upward,
      | r => r
    }
  } else {
    round
  };

  if precision >= 0 {
    if ndigits > precision as usize {
      ndigits = precision as usize;
    }

    if precision + exponenta <= 0 {
      let should_round = round == Rounding::Upward ||
        (round == Rounding::ToNearest &&
          precision + exponenta == 0 &&
          buffer[0] >= b'5');

      if should_round {
        result[0] = ascii::Char::Digit1;
        ndigits = 1;
        exponenta = 1 - precision;
        return FtoaReturn { digits: result, ndigits, exponenta };
      } else {
        ndigits = 0;
        exponenta = 0;
        return FtoaReturn { digits: result, ndigits, exponenta };
      }
    }
  }

  // Handle carry + round
  let should_round = match round {
    | Rounding::Upward => ndigits < written,
    | Rounding::ToNearest => ndigits < written && buffer[ndigits] >= b'5',
    | _ => false
  };

  if should_round {
    let mut carry = true;
    for i in (0..ndigits).rev() {
      if carry {
        buffer[i] += 1;
        carry = buffer[i] == b'0' + 10;
        if carry {
          buffer[i] = b'0';
        }
      }
    }
    if carry {
      result[0] = ascii::Char::Digit1;
      ndigits = 1;
      exponenta += 1;
      return FtoaReturn { digits: result, ndigits, exponenta };
    }
  }

  for i in 0..ndigits {
    result[i] = ascii::Char::from_u8(buffer[i]).unwrap_or(ascii::Char::Null);
  }

  FtoaReturn { digits: result, ndigits, exponenta }
}
