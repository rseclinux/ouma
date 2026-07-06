//
// Eisel-Lemire implementation was ported from LLVM libc:
// https://github.com/llvm/llvm-project/blob/10e7761cac92ee695d2a74a813ad3ebba4e649c0/libc/src/__support/str_to_float.h
//
// Big thanks to them :)
//

use {
  super::{
    IsSigned,
    b36_char_to_int,
    clinger::Clinger,
    detailed_powers_of_ten::*,
    get_char_with_index,
    strtoint
  },
  crate::{
    std::errno,
    support::{
      float::{
        Sign,
        f128::F128,
        rounding_mode::{Rounding, quick_get_round}
      },
      locale::{Locale, ctype::CtypeObject, get_slot},
      traits::{
        char::{CharToAscii, MatchChar},
        float::{Float, FloatBits}
      }
    }
  },
  bnum::cast::CastFrom,
  num_traits::{Bounded, One, PrimInt, Zero}
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;

#[derive(Clone, Copy, Debug)]
pub struct StrToFloatResult<T: Float> {
  pub value: T,
  pub len: usize,
  pub error: i32
}

impl<T: Float> Default for StrToFloatResult<T> {
  #[inline]
  fn default() -> Self {
    Self { value: T::zero(), len: 0, error: 0 }
  }
}

#[inline]
fn peek_isdigit<T: Into<CharToAscii> + Copy>(
  src: &[T],
  index: usize,
  ctype: &CtypeObject
) -> bool {
  let Some(x) = get_char_with_index(src, index) else {
    return false;
  };

  (ctype.casemap.isdigit)(x as u32)
}

#[inline]
fn peek_isxdigit<T: Into<CharToAscii> + Copy>(
  src: &[T],
  index: usize,
  ctype: &CtypeObject
) -> bool {
  let Some(x) = get_char_with_index(src, index) else {
    return false;
  };

  (ctype.casemap.isxdigit)(x as u32)
}

#[inline]
fn cast_f64_to_f32(i: f64) -> f32 {
  i as f32
}

#[inline]
fn cast_f64_to_f128(i: f64) -> F128 {
  F128(i as f128)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
fn cast_f64_to_f80(i: f64) -> F80 {
  F80::from_f64(i)
}

#[inline]
fn nan_mantissa_from_ncharseq<T: Into<CharToAscii> + Copy, F: FloatBits>(
  src: &[T],
  len: usize,
  ctype: &CtypeObject
) -> F::StorageType
where
  F::StorageType: num_traits::PrimInt
    + IsSigned
    + CastFrom<i32>
    + CastFrom<usize>
    + num_traits::WrappingNeg,
  usize: CastFrom<F::StorageType>,
  u8: CastFrom<F::StorageType> {
  let mut result = F::StorageType::zero();

  if let Some(c) = get_char_with_index(src, 0) &&
    (ctype.casemap.isdigit)(c as u32) &&
    len > 0
  {
    let r = strtoint::strtoint::<T, F::StorageType>(src, 0, ctype);
    if r.error == 0 {
      result = r.value;
    }
    if r.len != len {
      result = F::StorageType::zero();
    }
  }

  result
}

#[inline]
fn exp10_to_exp2(exp10: i32) -> i32 {
  // Valid if exp10 < 646_456_636.
  ((217_706i64 * exp10 as i64) >> 16) as i32
}

#[inline]
fn low64(num: u128) -> u64 {
  (num & 0xffffffffffffffff) as u64
}

#[inline]
fn high64(num: u128) -> u64 {
  (num >> 64) as u64
}

#[inline]
fn eisel_lemire_impl(
  sign: Sign,
  mantissa: u64,
  exp10: i32,
  round: &Rounding
) -> Option<f64> {
  // Check if exponent is out of range
  if exp10 < DETAILED_POWERS_OF_TEN_MIN_EXP_10 ||
    exp10 > DETAILED_POWERS_OF_TEN_MAX_EXP_10
  {
    return None;
  }

  let lz: u32 = mantissa.leading_zeros();

  // Normalize mantissa
  let mantissa = mantissa << lz;

  // Compute log2(exp10)
  let mut exp2 = exp10_to_exp2(exp10) +
    f64::STORAGE_LEN as i32 +
    f64::EXPONENT_BIAS as i32 -
    lz as i32;

  // Multiply!
  let table_offset: usize =
    (exp10 - DETAILED_POWERS_OF_TEN_MIN_EXP_10) as usize;
  let pow10: [u64; 2] = DETAILED_POWERS_OF_TEN[table_offset];
  let first_approx: u128 = u128::from(mantissa) * u128::from(pow10[1]);

  // The halfway constant is used to check if the bits that will be shifted away
  // initially are all 1. For doubles this is 64 (bitstype size) - 52 (final
  // mantissa size) - 3 (we shift away the last two bits separately for
  // accuracy, and the most significant bit is ignored.) = 9 bits. Similarly,
  // it's 6 bits for floats in this case.
  let halfway: u64 = (1u64 << (f64::STORAGE_LEN - (f64::FRACTION_LEN + 3))) - 1;

  // Wider approximation
  let final_approx: u128 = if (high64(first_approx) & halfway) == halfway &&
    low64(first_approx).wrapping_add(mantissa) < mantissa
  {
    let low_bits: u128 = u128::from(mantissa) * u128::from(pow10[0]);
    let second_approx: u128 = first_approx + u128::from(high64(low_bits));

    if (high64(second_approx) & halfway) == halfway &&
      low64(second_approx).wrapping_add(1) == 0 &&
      low64(low_bits).wrapping_add(mantissa) < mantissa
    {
      return None;
    }

    second_approx
  } else {
    first_approx
  };

  // Shifting to 54 bits for doubles and 25 bits for floats
  let msb: u64 = high64(final_approx) >> (f64::STORAGE_LEN as u64 - 1u64);
  let mut final_mantissa: u64 = high64(final_approx) >>
    (msb + f64::STORAGE_LEN as u64 - (f64::FRACTION_LEN as u64 + 3u64));

  exp2 -= (1u64 ^ msb) as u32 as i32;

  // Round according to rounding mode
  if *round == Rounding::ToNearest {
    if low64(final_approx) == 0 &&
      (high64(final_approx) & halfway) == 0 &&
      (final_mantissa & 3) == 1
    {
      return None;
    }

    final_mantissa += final_mantissa & 1u64;
  } else if *round == Rounding::Upward {
    if low64(final_approx) > 0 || (high64(final_approx) & halfway) > 0 {
      final_mantissa += 2;
    }
  }

  // From 54 to 53 bits for doubles and 25 to 24 bits for floats
  final_mantissa >>= 1;
  if (final_mantissa >> (f64::FRACTION_LEN + 1)) > 0 {
    final_mantissa >>= 1;
    exp2 += 1;
  }

  if (exp2 as u32) - 1 >= (1 << f64::EXPONENT_LEN) - 2 {
    return None;
  }

  Some(f64::create_value(sign, exp2 as u32, final_mantissa))
}

pub trait EiselLemire: FloatBits {
  fn eisel_lemire(
    sign: Sign,
    mantissa: Self::StorageType,
    exp10: i32,
    round: &Rounding
  ) -> Option<Self>;

  #[inline]
  fn get_upper_bound() -> i32 {
    Self::EXPONENT_BIAS as i32 / 3
  }

  #[inline]
  fn get_lower_bound() -> i32 {
    let bias = Self::EXPONENT_BIAS as i32;
    -((bias + (Self::FRACTION_LEN + Self::STORAGE_LEN) as i32) / 3)
  }
}

impl EiselLemire for f32 {
  #[inline]
  fn eisel_lemire(
    sign: Sign,
    mantissa: Self::StorageType,
    exp10: i32,
    round: &Rounding
  ) -> Option<Self> {
    let r = eisel_lemire_impl(sign, mantissa as u64, exp10, round)?;
    let r = cast_f64_to_f32(r);
    if !r.is_finite() {
      return None;
    }
    Some(r)
  }

  #[inline]
  fn get_upper_bound() -> i32 {
    39
  }

  #[inline]
  fn get_lower_bound() -> i32 {
    -(39 + 6 + 10)
  }
}

impl EiselLemire for f64 {
  #[inline]
  fn eisel_lemire(
    sign: Sign,
    mantissa: Self::StorageType,
    exp10: i32,
    round: &Rounding
  ) -> Option<Self> {
    eisel_lemire_impl(sign, mantissa as u64, exp10, round)
  }

  #[inline]
  fn get_upper_bound() -> i32 {
    309
  }

  #[inline]
  fn get_lower_bound() -> i32 {
    -(309 + 15 + 20)
  }
}

impl EiselLemire for F128 {
  #[inline]
  fn eisel_lemire(
    sign: Sign,
    mantissa: Self::StorageType,
    exp10: i32,
    round: &Rounding
  ) -> Option<Self> {
    let r = eisel_lemire_impl(sign, mantissa as u64, exp10, round)?;
    let r = cast_f64_to_f128(r);
    Some(r)
  }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl EiselLemire for F80 {
  #[inline]
  fn eisel_lemire(
    sign: Sign,
    mantissa: Self::StorageType,
    exp10: i32,
    round: &Rounding
  ) -> Option<Self> {
    let r = eisel_lemire_impl(sign, mantissa as u64, exp10, round)?;
    let r = cast_f64_to_f80(r);
    Some(r)
  }
}

fn clinger_fast_path<F: FloatBits + Clinger>(
  sign: Sign,
  mantissa: F::StorageType,
  exp10: i32,
  round: &Rounding
) -> Option<F> {
  if (mantissa >> F::FRACTION_LEN) > F::StorageType::zero() {
    return None;
  }

  let pow10 = F::get_power_of_ten_slice();
  let mut float_mantissa = F::mantissa_to_float(mantissa);
  let mut exp10 = exp10;
  let result: F::ClingerFloatType;
  let mut rounded_multiply = false;

  if exp10 == 0 {
    result = float_mantissa;
  } else if exp10 > 0 {
    if exp10 > F::EXACT_POWERS_OF_TEN + F::DIGITS_IN_MANTISSA {
      return None;
    }
    if exp10 > F::EXACT_POWERS_OF_TEN {
      float_mantissa *= pow10[(exp10 - F::EXACT_POWERS_OF_TEN) as usize];
      exp10 = F::EXACT_POWERS_OF_TEN;
    }
    if float_mantissa > F::MAX_EXACT_INT {
      return None;
    }
    result = float_mantissa * pow10[exp10 as usize];
    rounded_multiply = true;
  } else {
    if -exp10 > F::EXACT_POWERS_OF_TEN {
      return None;
    }
    result = float_mantissa / pow10[(-exp10) as usize];
  }

  let mut result = result;

  if rounded_multiply && !matches!(round, Rounding::ToNearest) {
    let negative_result = (-float_mantissa) * pow10[exp10 as usize];

    if result != -negative_result {
      let (lower_result, higher_result) = if result < -negative_result {
        (result, negative_result)
      } else {
        (negative_result, result)
      };

      result = match round {
        | Rounding::Upward => higher_result,
        | _ => lower_result
      };
    }
  }

  let result = F::from_clinger_float(result);
  Some(F::create_value(
    sign,
    result.get_biased_exponent(),
    result.get_explicit_mantissa()
  ))
}

#[inline]
fn decimal_exp_to_float<
  T: MatchChar + Into<CharToAscii> + Copy,
  F: EiselLemire + Clinger
>(
  exp10: i32,
  mantissa: F::StorageType,
  sign: Sign,
  round: &Rounding,
  is_truncated: bool,
  _s: &[T]
) -> StrToFloatResult<F> {
  let mut result = StrToFloatResult::<F>::default();

  if exp10 > F::get_upper_bound() {
    result.value = F::create_value(
      sign,
      F::MAX_BIASED_EXPONENT as u32,
      F::StorageType::zero()
    );
    result.error = errno::ERANGE;
    return result;
  }
  if exp10 < F::get_lower_bound() {
    result.value = F::create_value(sign, 0u32, F::StorageType::zero());
    result.error = errno::ERANGE;
    return result;
  }

  if !is_truncated {
    if let Some(clinger) = clinger_fast_path::<F>(sign, mantissa, exp10, round)
    {
      result.error = 0;
      result.value = clinger;
      return result;
    }
  }

  if let Some(first) = F::eisel_lemire(sign, mantissa, exp10, round) {
    if !is_truncated {
      result.error = 0;
      result.value = first;
      return result;
    }

    // If the mantissa is truncated, then the result may be off by the LSB, so
    // check if rounding the mantissa up changes the result. If not, then it's
    // safe, else use the fallback.
    if let Some(second) =
      F::eisel_lemire(sign, mantissa + 1.into(), exp10, round)
    {
      if second.get_explicit_mantissa() == first.get_explicit_mantissa() &&
        second.get_biased_exponent() == first.get_biased_exponent()
      {
        result.error = 0;
        result.value = first;
        return result;
      }
    }
  }

  // TODO: Implement High Precision Decimal by Nigel Tao

  result.error = errno::ERANGE;
  result.value = F::inf(sign);

  result
}

#[inline]
fn hexadecimal_exp_to_float<
  T: MatchChar + Into<CharToAscii> + Copy,
  F: FloatBits
>(
  exp2: i32,
  mantissa: F::StorageType,
  sign: Sign,
  round: &Rounding,
  is_truncated: bool
) -> StrToFloatResult<F> {
  let inf_exp: i32 = ((1u32 << F::EXPONENT_LEN) - 1u32) as i32;

  let mut result = StrToFloatResult::<F>::default();
  let mut mantissa = mantissa;
  let mut exp2 = exp2;

  let amount_to_shl: u32 = mantissa.leading_zeros();
  mantissa = mantissa << amount_to_shl;

  exp2 -= amount_to_shl as i32;

  let mut biased_exponent: i32 =
    exp2 + (F::STORAGE_LEN as i32) + (F::EXPONENT_BIAS as i32) - 1i32;
  if biased_exponent >= inf_exp {
    result.value = F::create_value(
      sign,
      (1u32 << F::EXPONENT_LEN) - 1u32,
      F::StorageType::zero()
    );
    result.error = errno::ERANGE;
    return result;
  }

  let mut amount_to_shr: u32 = F::STORAGE_LEN - F::FRACTION_LEN - 1u32;

  // Handle subnormals
  if biased_exponent <= 0 {
    amount_to_shr = (amount_to_shr as i32 + 1 - biased_exponent) as u32;
    biased_exponent = 0;

    if amount_to_shr > F::STORAGE_LEN {
      result.value = F::create_value(sign, 0, F::StorageType::zero());
      result.error = errno::ERANGE;
      return result;
    }
  }

  let round_bit_mask = F::StorageType::one() << (amount_to_shr - 1u32);
  let sticky_mask = round_bit_mask - F::StorageType::one();
  let round_bit = mantissa & round_bit_mask != F::StorageType::zero();
  let sticky_bit =
    (mantissa & sticky_mask != F::StorageType::zero()) || is_truncated;

  if amount_to_shr < F::STORAGE_LEN {
    // Shift the mantissa and clear the implicit bit.
    mantissa = mantissa >> amount_to_shr;
    mantissa = mantissa & F::FRACTION_MASK;
  } else {
    mantissa = F::StorageType::zero();
  }

  let least_significant_bit = bool::from(mantissa & 1u32.into() == 1u32.into());

  if *round == Rounding::ToNearest {
    if round_bit && (least_significant_bit || sticky_bit) {
      mantissa = mantissa + F::StorageType::one();
    }
  } else if *round == Rounding::Upward {
    if round_bit || sticky_bit {
      mantissa = mantissa + F::StorageType::one();
    }
  } else {
    if round_bit && sticky_bit {
      mantissa = mantissa + F::StorageType::one();
    }
  }

  if mantissa > F::FRACTION_MASK {
    biased_exponent += 1;

    if biased_exponent == inf_exp {
      result.error = errno::ERANGE;
    }
  }

  if biased_exponent == 0 {
    result.error = errno::ERANGE;
  }

  result.value =
    F::create_value(sign, biased_exponent as u32, mantissa & F::FRACTION_MASK);
  result
}

#[inline]
pub fn strtofloat<
  T: MatchChar + Into<CharToAscii> + Copy,
  F: EiselLemire + Clinger
>(
  src: &[T],
  locale: &Locale
) -> StrToFloatResult<F>
where
  F::StorageType: num_traits::PrimInt
    + IsSigned
    + CastFrom<i32>
    + CastFrom<usize>
    + num_traits::WrappingNeg,
  usize: CastFrom<F::StorageType>,
  u8: CastFrom<F::StorageType> {
  let mut result = StrToFloatResult::<F>::default();
  let mut index = 0usize;
  let mut has_number = false;

  let ctype = get_slot(&locale.ctype).unwrap_or_default();
  let numeric = get_slot(&locale.numeric).unwrap_or_default();

  let decimal_point: char = numeric.get_decimal_point().unwrap_or('.');

  while let Some(c) = get_char_with_index(src, index) &&
    (ctype.casemap.isspace)(c as u32)
  {
    index += 1;
  }

  let negative = if let Some(c) = get_char_with_index(src, index) &&
    c == '-'
  {
    index += 1;
    true
  } else {
    if let Some(c) = get_char_with_index(src, index) &&
      c == '+'
    {
      index += 1;
    }
    false
  };
  let sign = if negative { Sign::Negative } else { Sign::Positive };

  let round = match quick_get_round() {
    | Rounding::ToNearest => Rounding::ToNearest,
    | Rounding::Upward => {
      if sign == Sign::Positive {
        Rounding::Upward
      } else {
        Rounding::Downward
      }
    },
    | Rounding::Downward => {
      if sign == Sign::Positive {
        Rounding::Downward
      } else {
        Rounding::Upward
      }
    },
    | Rounding::TowardZero => Rounding::Downward,
    | _ => Rounding::ToNearest
  };

  // Handle infinity
  if (get_char_with_index(src, index) == Some('i') ||
    get_char_with_index(src, index) == Some('I')) &&
    (get_char_with_index(src, index + 1) == Some('n') ||
      get_char_with_index(src, index + 1) == Some('N')) &&
    (get_char_with_index(src, index + 2) == Some('f') ||
      get_char_with_index(src, index + 2) == Some('F'))
  {
    index += 3;
    if (get_char_with_index(src, index) == Some('i') ||
      get_char_with_index(src, index) == Some('I')) &&
      (get_char_with_index(src, index + 1) == Some('n') ||
        get_char_with_index(src, index + 1) == Some('N')) &&
      (get_char_with_index(src, index + 2) == Some('i') ||
        get_char_with_index(src, index + 2) == Some('I')) &&
      (get_char_with_index(src, index + 3) == Some('t') ||
        get_char_with_index(src, index + 3) == Some('T')) &&
      (get_char_with_index(src, index + 4) == Some('y') ||
        get_char_with_index(src, index + 4) == Some('Y'))
    {
      index += 5;
    }
    has_number = true;
    result.value = F::inf(sign);
  } else if (get_char_with_index(src, index) == Some('n') ||
    get_char_with_index(src, index) == Some('N')) &&
    (get_char_with_index(src, index + 1) == Some('a') ||
      get_char_with_index(src, index + 1) == Some('A')) &&
    (get_char_with_index(src, index + 2) == Some('n') ||
      get_char_with_index(src, index + 2) == Some('N'))
  {
    // Handle NaN
    index += 3;
    has_number = true;

    let mut nan_mant = F::StorageType::zero();

    if let Some(c) = get_char_with_index(src, index) &&
      c == '('
    {
      index += 1;

      let left_paren = index;
      while let Some(c) = get_char_with_index(src, index) &&
        ((ctype.casemap.isalnum)(c as u32) || c == '_')
      {
        index += 1;
      }
      if let Some(c) = get_char_with_index(src, index) &&
        c == ')'
      {
        let payload_len = index - left_paren;
        let offset = left_paren;

        index += 1; // consume ')'

        nan_mant = nan_mantissa_from_ncharseq::<T, F>(
          &src[offset..],
          payload_len,
          &ctype
        );
      } else {
        index = left_paren;
      }
    }

    result.value = F::nan(sign, nan_mant);
  } else if get_char_with_index(src, index) == Some('0') &&
    (get_char_with_index(src, index + 1) == Some('x') ||
      get_char_with_index(src, index + 1) == Some('X')) &&
    (peek_isxdigit(src, index + 2, &ctype) ||
      get_char_with_index(src, index + 2) == Some('.') &&
        peek_isxdigit(src, index + 3, &ctype))
  {
    index += 2; // consume "0x" / "0X"

    const HEX_BASE: u32 = 16;

    let max_div_by_base: F::StorageType =
      F::StorageType::max_value() / HEX_BASE.into();
    let mut mantissa: F::StorageType = F::StorageType::zero();
    let mut exponent = 0i32;
    let mut got_digit = false;
    let mut is_truncated = false;
    let mut parsed_radixchar = false;
    let mut out = StrToFloatResult::<F>::default();

    // Parse hex digits + radix point
    loop {
      if let Some(c) = get_char_with_index(src, index) &&
        (ctype.casemap.isalnum)(c as u32)
      {
        let Some(digit) = b36_char_to_int(c) else {
          break;
        };
        if digit < HEX_BASE {
          got_digit = true;
        } else {
          break;
        }

        if mantissa < max_div_by_base {
          mantissa = (mantissa * HEX_BASE.into()) + digit.into();
          if parsed_radixchar {
            exponent -= 1;
          }
        } else {
          if digit > 0 {
            is_truncated = true;
          }
          if !parsed_radixchar {
            exponent += 1;
          }
        }

        index += 1;
        continue;
      }

      // parse radix character
      if T::char_matches(decimal_point, src, index) {
        if parsed_radixchar {
          break;
        }
        parsed_radixchar = true;
        index += 1;

        continue;
      }

      break;
    }

    if got_digit {
      exponent *= 4;

      if get_char_with_index(src, index) == Some('p') ||
        get_char_with_index(src, index) == Some('P')
      {
        if peek_isdigit(src, index + 1, &ctype) ||
          ((get_char_with_index(src, index + 1) == Some('-') ||
            get_char_with_index(src, index + 1) == Some('+')) &&
            peek_isdigit(src, index + 2, &ctype))
        {
          index += 1;
          let exp_neg = if get_char_with_index(src, index) == Some('-') {
            index += 1;
            true
          } else {
            if get_char_with_index(src, index) == Some('+') {
              index += 1;
            }
            false
          };

          let mut suffix_exp = 0i32;
          while let Some(c) = get_char_with_index(src, index) &&
            c >= '0' &&
            c <= '9'
          {
            if suffix_exp < 1000000 {
              let d = c as u32 - '0' as u32;
              suffix_exp = suffix_exp * 10 + d as i32;
            }
            index += 1;
          }

          if exp_neg {
            suffix_exp = -suffix_exp;
          }

          exponent += suffix_exp;
        }
      }

      out.len = index;
      if mantissa == F::StorageType::zero() {
        out.value = F::zero().set_sign(sign);
      } else {
        let conv_result: StrToFloatResult<F> = hexadecimal_exp_to_float::<T, F>(
          exponent,
          mantissa,
          sign,
          &round,
          is_truncated
        );

        out.error = conv_result.error;
        out.value = conv_result.value;
      }

      out.value =
        out.value.set_implicit_bit(out.value.get_biased_exponent() != 0);

      return out;
    }
  } else {
    const BASE: u32 = 10;

    let max_div_by_base: F::StorageType =
      F::StorageType::max_value() / BASE.into();
    let mut mantissa: F::StorageType = F::StorageType::zero();
    let mut exponent = 0i32;
    let mut got_digit = false;
    let mut is_truncated = false;
    let mut parsed_radixchar = false;
    let mut out = StrToFloatResult::<F>::default();

    // Skip leading zeroes
    while let Some(c) = get_char_with_index(src, index) &&
      c == '0'
    {
      got_digit = true;
      index += 1;
    }

    // Parse numbers + radix
    loop {
      if let Some(c) = get_char_with_index(src, index) &&
        (ctype.casemap.isdigit)(c as u32)
      {
        let Some(digit) = b36_char_to_int(c) else {
          break;
        };
        got_digit = true;

        if mantissa < max_div_by_base {
          mantissa = (mantissa * BASE.into()) + digit.into();
          if parsed_radixchar {
            exponent -= 1;
          }
        } else {
          if digit > 0 {
            is_truncated = true;
          }
          if !parsed_radixchar {
            exponent += 1;
          }
        }

        index += 1;
        continue;
      }

      // parse radix character
      if T::char_matches(decimal_point, src, index) {
        if parsed_radixchar {
          break;
        }
        parsed_radixchar = true;
        index += 1;

        continue;
      }

      break;
    }

    if got_digit {
      // Parse exponent
      if get_char_with_index(src, index) == Some('e') ||
        get_char_with_index(src, index) == Some('E')
      {
        if peek_isdigit(src, index + 1, &ctype) ||
          ((get_char_with_index(src, index + 1) == Some('-') ||
            get_char_with_index(src, index + 1) == Some('+')) &&
            peek_isdigit(src, index + 2, &ctype))
        {
          index += 1;
          let exp_neg = if get_char_with_index(src, index) == Some('-') {
            index += 1;
            true
          } else {
            if get_char_with_index(src, index) == Some('+') {
              index += 1;
            }
            false
          };

          let mut suffix_exp = 0i32;
          while let Some(c) = get_char_with_index(src, index) &&
            c >= '0' &&
            c <= '9'
          {
            if suffix_exp < 1000000 {
              let d = c as u32 - '0' as u32;
              suffix_exp = suffix_exp * 10 + d as i32;
            }
            index += 1;
          }

          if exp_neg {
            suffix_exp = -suffix_exp;
          }

          exponent += suffix_exp;
        }
      }

      out.len = index;
      if mantissa == F::StorageType::zero() {
        out.value = F::zero().set_sign(sign);
      } else {
        let conv_result: StrToFloatResult<F> = decimal_exp_to_float::<T, F>(
          exponent,
          mantissa,
          sign,
          &round,
          is_truncated,
          src
        );

        out.error = conv_result.error;
        out.value = conv_result.value;
      }

      out.value =
        out.value.set_implicit_bit(out.value.get_biased_exponent() != 0);

      return out;
    }
  }

  if !has_number {
    result.error = errno::EINVAL;
    result.value = F::zero();
    result.len = 0;
    return result;
  } else {
    result.len = index;
  }

  result.value =
    result.value.set_implicit_bit(result.value.get_biased_exponent() != 0);

  result
}
