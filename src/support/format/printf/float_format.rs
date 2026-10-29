use {
  super::{Emitter, PrintfArgument, values::FloatValue},
  crate::{
    support::{
      float::rounding_mode::{Rounding, get_rounding},
      format::{
        error::FormatError,
        grouping::NumericGrouping,
        printf::CastToDouble
      },
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::{
        ftoa::{self, DragonFloat},
        itoa,
        ryu
      },
      traits::float::FloatBits
    },
    types::intmax_t
  },
  bnum::cast::CastFrom,
  core::ascii,
  num_traits::{One, Zero}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatConv {
  Fixed,
  Scientific,
  Automatic
}

const NAN_SMALL: &'static [ascii::Char] = ascii_str!(b"nan");
const NAN_CAPITAL: &'static [ascii::Char] = ascii_str!(b"NAN");
const INF_SMALL: &'static [ascii::Char] = ascii_str!(b"inf");
const INF_CAPITAL: &'static [ascii::Char] = ascii_str!(b"INF");

#[inline]
pub fn format_non_finite<T: FloatBits, E: Emitter>(
  emitter: &mut E,
  num: T,
  arg: &PrintfArgument,
  ctype: &CtypeObject
) -> Result<(), FormatError> {
  let lowercase = (ctype.casemap.islower)(arg.specifier as u32);

  let sign = if num.is_sign_negative() {
    Some(ascii::Char::HyphenMinus)
  } else if arg.flags.prepend_plus {
    Some(ascii::Char::PlusSign)
  } else if arg.flags.space_prefix {
    Some(ascii::Char::Space)
  } else {
    None
  };

  let s: usize = if sign.is_some() { 1 } else { 0 };

  let pad: usize = arg.width.saturating_sub(s).saturating_sub(3);

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, s + 3)?;
  }

  if let Some(s) = sign {
    emitter.emit_ascii_char(s)?;
  }

  if num.is_inf() {
    if lowercase {
      emitter.emit_ascii_slice(INF_SMALL)?;
    } else {
      emitter.emit_ascii_slice(INF_CAPITAL)?;
    }
  } else {
    if lowercase {
      emitter.emit_ascii_slice(NAN_SMALL)?;
    } else {
      emitter.emit_ascii_slice(NAN_CAPITAL)?;
    }
  }

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, s + 3)?;
  }

  Ok(())
}

#[inline]
pub fn format_float_dragon<T: DragonFloat, E: Emitter>(
  emitter: &mut E,
  value: T,
  conv: FloatConv,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError>
where
  T::Bn: ftoa::DragonInt + bnum::cast::CastFrom<T::StorageType>,
  u8: bnum::cast::CastFrom<T::Bn> {
  let decimal_point: char = numeric.get_decimal_point().unwrap_or('\0');
  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let lowercase = (ctype.casemap.islower)(arg.specifier as u32);

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    conv != FloatConv::Scientific;

  let mut precision = arg.precision.unwrap_or(6) as i32;
  let mut e_mode = conv == FloatConv::Scientific;
  let mut float_strip_trailing = false;

  let mut sign: Option<ascii::Char> = None;
  if value.is_sign_negative() {
    sign = Some(ascii::Char::HyphenMinus);
  } else if arg.flags.prepend_plus {
    sign = Some(ascii::Char::PlusSign);
  } else if arg.flags.space_prefix {
    sign = Some(ascii::Char::Space);
  }

  let mut needed_digits = if conv == FloatConv::Fixed {
    precision
  } else if conv == FloatConv::Scientific {
    precision + 1
  } else {
    precision
  };

  if conv == FloatConv::Automatic && needed_digits == 0 {
    needed_digits = 1;
  }

  let result = if conv == FloatConv::Fixed {
    ftoa::format_dragon4(value, needed_digits, get_rounding())
  } else {
    ftoa::format_dragon4_exp(value, needed_digits, get_rounding())
  };

  let mut digits = result.digits;
  let mut ndigits = result.ndigits;
  let mut exponent = result.exponenta - 1;

  if conv == FloatConv::Automatic {
    let p = needed_digits;
    if p > exponent && exponent >= -4 {
      precision = p - 1 - exponent;
      if !arg.flags.alternate_form {
        float_strip_trailing = true;
      }
    } else {
      e_mode = true;
      precision = p - 1;
    }
  }

  if !e_mode {
    if float_strip_trailing {
      let mut nd = ndigits;
      while nd > 0 && digits[nd - 1] == ascii::Char::Digit0 {
        nd -= 1;
      }
      precision = precision.min((nd as i32 - exponent - 1).max(0));
    }

    let print_radixchar = arg.flags.alternate_form || precision > 0;

    let mut left_digits_with_grouping = (exponent + 1).max(1) as usize;
    let mut ng = NumericGrouping::new(grouping, left_digits_with_grouping);

    if use_grouping {
      left_digits_with_grouping +=
        ng.width * E::get_unicode_char_len(thousands_sep).max(1);
    }

    let width = usize::from(sign.is_some()) +
      left_digits_with_grouping +
      usize::from(print_radixchar) +
      (precision as usize);

    if arg.flags.leading_zeroes {
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
      emitter.pad_to(arg, ascii::Char::Digit0, width)?;
    } else {
      if !arg.flags.left_align {
        emitter.pad_to(arg, ascii::Char::Space, width)?;
      }
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
    }

    let mut pos: isize;
    let mut idx: isize;
    if exponent >= 1 {
      pos = (-exponent) as isize - 1;
      idx = 0;
    } else {
      pos = -1;
      idx = (exponent as isize + 1) - 1;
    }

    while pos < (precision as isize) {
      if pos < 0 && use_grouping {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
      }

      let digit = if idx >= 0 && (idx as usize) < ndigits {
        digits[idx as usize]
      } else {
        ascii::Char::Digit0
      };
      emitter.emit_ascii_char(digit)?;

      idx += 1;
      pos += 1;

      if pos == 0 && print_radixchar {
        emitter.emit_unicode_char(decimal_point)?;
      }
    }

    if arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
  } else {
    if conv == FloatConv::Automatic && !arg.flags.alternate_form {
      while ndigits > 1 && digits[ndigits - 1] == ascii::Char::Digit0 {
        ndigits -= 1;
      }
      precision = ndigits as i32 - 1;
    }

    let mut exp_negative = false;
    if exponent < 0 {
      exp_negative = true;
      exponent = -exponent;
    }

    let mut exp_digits_buf =
      [ascii::Char::Null; core::mem::size_of::<intmax_t>() * 8];
    let exp_digits = itoa::format_integer(
      exponent,
      itoa::ItoaFormat::Decimal,
      &mut exp_digits_buf,
      lowercase
    );

    if ndigits == 0 {
      digits[0] = ascii::Char::Digit0;
      ndigits = 1;
    }

    let mut width: usize = if precision + 1 > (ndigits as i32) {
      (precision + 1) as usize
    } else {
      ndigits
    };

    let print_radix_char = arg.flags.alternate_form || width > 1;
    width += usize::from(sign.is_some()) +
      exp_digits.len().max(2) +
      if print_radix_char { 3 } else { 2 };

    if arg.flags.leading_zeroes {
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
      emitter.pad_to(arg, ascii::Char::Digit0, width)?;
    } else {
      if !arg.flags.left_align {
        emitter.pad_to(arg, ascii::Char::Space, width)?;
      }
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
    }

    emitter.emit_ascii_char(digits[0])?;
    if print_radix_char {
      emitter.emit_unicode_char(decimal_point)?;
    }

    for i in 1..ndigits {
      emitter.emit_ascii_char(digits[i])?;
    }

    emitter.emit_ascii_char(if lowercase {
      ascii::Char::SmallE
    } else {
      ascii::Char::CapitalE
    })?;
    emitter.emit_ascii_char(if exp_negative {
      ascii::Char::HyphenMinus
    } else {
      ascii::Char::PlusSign
    })?;

    if exp_digits.len() < 2 {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }

    let mut k = 0usize;
    while k < exp_digits.len() {
      emitter.emit_ascii_char(exp_digits[k])?;
      k += 1;
    }

    if arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
  }

  Ok(())
}

#[inline]
pub fn format_float_ryu<E: Emitter>(
  emitter: &mut E,
  value: f64,
  conv: FloatConv,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let decimal_point: char = numeric.get_decimal_point().unwrap_or('\0');
  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let lowercase = (ctype.casemap.islower)(arg.specifier as u32);

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    conv != FloatConv::Scientific;

  let mut precision = arg.precision.unwrap_or(6) as i32;
  let mut e_mode = conv == FloatConv::Scientific;
  let mut float_strip_trailing = false;

  let mut sign: Option<ascii::Char> = None;
  if value.is_sign_negative() {
    sign = Some(ascii::Char::HyphenMinus);
  } else if arg.flags.prepend_plus {
    sign = Some(ascii::Char::PlusSign);
  } else if arg.flags.space_prefix {
    sign = Some(ascii::Char::Space);
  }

  let mut needed_digits = if conv == FloatConv::Fixed {
    precision
  } else if conv == FloatConv::Scientific {
    precision + 1
  } else {
    precision
  };

  if conv == FloatConv::Automatic && needed_digits == 0 {
    needed_digits = 1;
  }

  let result = if conv == FloatConv::Fixed {
    ryu::format_ryu(value, needed_digits, get_rounding())
  } else {
    ryu::format_ryu_exp(value, needed_digits, get_rounding())
  };

  let mut digits = result.digits;
  let mut ndigits = result.ndigits;
  let mut exponent = result.exponenta;

  if conv == FloatConv::Automatic {
    let p = needed_digits;
    if p > exponent && exponent >= -4 {
      precision = p - 1 - exponent;
      if !arg.flags.alternate_form {
        float_strip_trailing = true;
      }
    } else {
      e_mode = true;
      precision = p - 1;
    }
  }

  if !e_mode {
    if float_strip_trailing {
      let mut nd = ndigits;
      while nd > 0 && digits[nd - 1] == ascii::Char::Digit0 {
        nd -= 1;
      }
      precision = precision.min((nd as i32 - exponent - 1).max(0));
    }

    let print_radixchar = arg.flags.alternate_form || precision > 0;

    let mut left_digits_with_grouping = (exponent + 1).max(1) as usize;
    let mut ng = NumericGrouping::new(grouping, left_digits_with_grouping);

    if use_grouping {
      left_digits_with_grouping +=
        ng.width * E::get_unicode_char_len(thousands_sep).max(1);
    }

    let width = usize::from(sign.is_some()) +
      left_digits_with_grouping +
      usize::from(print_radixchar) +
      (precision as usize);

    if arg.flags.leading_zeroes {
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
      emitter.pad_to(arg, ascii::Char::Digit0, width)?;
    } else {
      if !arg.flags.left_align {
        emitter.pad_to(arg, ascii::Char::Space, width)?;
      }
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
    }

    let mut pos: isize;
    let mut idx: isize;
    if exponent >= 1 {
      pos = (-exponent) as isize - 1;
      idx = 0;
    } else {
      pos = -1;
      idx = (exponent as isize + 1) - 1;
    }

    while pos < (precision as isize) {
      if pos < 0 && use_grouping {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
      }

      let digit = if idx >= 0 && (idx as usize) < ndigits {
        digits[idx as usize]
      } else {
        ascii::Char::Digit0
      };
      emitter.emit_ascii_char(digit)?;

      idx += 1;
      pos += 1;

      if pos == 0 && print_radixchar {
        emitter.emit_unicode_char(decimal_point)?;
      }
    }

    if arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
  } else {
    if conv == FloatConv::Automatic && !arg.flags.alternate_form {
      while ndigits > 1 && digits[ndigits - 1] == ascii::Char::Digit0 {
        ndigits -= 1;
      }
      precision = ndigits as i32 - 1;
    }

    let mut exp_negative = false;
    if exponent < 0 {
      exp_negative = true;
      exponent = -exponent;
    }

    let mut exp_digits_buf =
      [ascii::Char::Null; core::mem::size_of::<intmax_t>() * 8];
    let exp_digits = itoa::format_integer(
      exponent,
      itoa::ItoaFormat::Decimal,
      &mut exp_digits_buf,
      lowercase
    );

    if ndigits == 0 {
      digits[0] = ascii::Char::Digit0;
      ndigits = 1;
    }

    let mut width: usize = if precision + 1 > (ndigits as i32) {
      (precision + 1) as usize
    } else {
      ndigits
    };

    let print_radix_char = arg.flags.alternate_form || width > 1;
    width += usize::from(sign.is_some()) +
      exp_digits.len().max(2) +
      if print_radix_char { 3 } else { 2 };

    if arg.flags.leading_zeroes {
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
      emitter.pad_to(arg, ascii::Char::Digit0, width)?;
    } else {
      if !arg.flags.left_align {
        emitter.pad_to(arg, ascii::Char::Space, width)?;
      }
      if let Some(s) = sign {
        emitter.emit_ascii_char(s)?;
      }
    }

    emitter.emit_ascii_char(digits[0])?;
    if print_radix_char {
      emitter.emit_unicode_char(decimal_point)?;
    }

    for i in 1..ndigits {
      emitter.emit_ascii_char(digits[i])?;
    }

    emitter.emit_ascii_char(if lowercase {
      ascii::Char::SmallE
    } else {
      ascii::Char::CapitalE
    })?;
    emitter.emit_ascii_char(if exp_negative {
      ascii::Char::HyphenMinus
    } else {
      ascii::Char::PlusSign
    })?;

    if exp_digits.len() < 2 {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }

    let mut k = 0usize;
    while k < exp_digits.len() {
      emitter.emit_ascii_char(exp_digits[k])?;
      k += 1;
    }

    if arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
  }

  Ok(())
}

#[inline]
pub fn format_float_hexadecimal_typed<T: FloatBits, E: Emitter>(
  emitter: &mut E,
  num: T,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError>
where
  u8: CastFrom<T::StorageType> {
  let prec = arg.precision.unwrap_or(0);
  let decimal_point = numeric.get_decimal_point().unwrap_or('\0');

  let mut mantissa = num.get_explicit_mantissa();
  let mut exponent = num.get_explicit_exponent();

  if mantissa == T::StorageType::zero() {
    exponent = 0;
  }

  let sign = if num.is_sign_negative() {
    Some(ascii::Char::HyphenMinus)
  } else if arg.flags.prepend_plus {
    Some(ascii::Char::PlusSign)
  } else if arg.flags.space_prefix {
    Some(ascii::Char::Space)
  } else {
    None
  };

  const BITS_IN_HEX_DIGIT: usize = 4;

  let mut mant_buf = [ascii::Char::Null; 128];
  let mut mant_len: usize = (T::FRACTION_LEN as usize / BITS_IN_HEX_DIGIT) + 1;

  if (T::FRACTION_LEN as usize) & (BITS_IN_HEX_DIGIT - 1) != 0 &&
    mantissa > T::StorageType::zero()
  {
    exponent -= (T::FRACTION_LEN as usize % BITS_IN_HEX_DIGIT) as i32;
  }

  if let Some(p) = arg.precision {
    let intended = p as usize + 1;

    if intended < mant_len {
      let shift_amount = ((mant_len - intended) * BITS_IN_HEX_DIGIT) as u32;

      let shift: T::StorageType = T::StorageType::one() << shift_amount;
      let trunc_bits = mantissa & (shift - T::StorageType::one());
      let halfway = T::StorageType::one() << (shift_amount - 1);

      mantissa >>= shift_amount.into();

      match get_rounding() {
        | Rounding::ToNearest => {
          if trunc_bits > halfway {
            mantissa += T::StorageType::one();
          } else if trunc_bits == halfway {
            mantissa = mantissa + (mantissa & T::StorageType::one());
          }
        },
        | Rounding::Downward => {
          if trunc_bits > T::StorageType::zero() && num.is_sign_negative() {
            mantissa += T::StorageType::one();
          }
        },
        | Rounding::Upward => {
          if trunc_bits > T::StorageType::zero() && num.is_sign_positive() {
            mantissa += T::StorageType::one();
          }
        },
        | Rounding::TowardZero => {}
      };

      if mantissa >=
        (T::StorageType::one() <<
          (intended as u32 * BITS_IN_HEX_DIGIT as u32))
      {
        mantissa >>= (BITS_IN_HEX_DIGIT as u32).into();
        exponent += BITS_IN_HEX_DIGIT as i32;
      }

      mant_len = intended;
    }
  }

  let mut mant_cur = mant_len;
  let mut first_non_zero = 1;

  while mant_cur > 0 {
    let mant_mod_16 = mantissa % 16u32.into();
    let raw_digit = u8::cast_from(mant_mod_16);
    let mut digit = if raw_digit < 10 {
      ascii::Char::from_u8(b'0' + raw_digit).unwrap_or(ascii::Char::Null)
    } else {
      ascii::Char::from_u8(b'a' + (raw_digit - 10)).unwrap_or(ascii::Char::Null)
    };

    if (ctype.casemap.isupper)(arg.specifier as u32) {
      digit = digit.to_uppercase();
    }

    mant_buf[mant_cur - 1] = digit;

    if digit != core::ascii::Char::Digit0 && first_non_zero < mant_cur {
      first_non_zero = mant_cur;
    }

    mant_cur -= 1;
    mantissa >>= 4.into();
  }

  let mant_digits =
    if arg.precision.is_some() { mant_len } else { first_non_zero };

  const EXP_LEN: usize = (((5000 * 5) + 15) / 16) + 1;

  let mut exp_buf = [ascii::Char::Null; EXP_LEN];
  let mut exp_cur = EXP_LEN;

  let exp_is_negative = exponent < 0;
  if exp_is_negative {
    exponent = -exponent;
  }

  while exponent > 0 {
    let digit = (exponent % 10) as u8;
    exp_buf[exp_cur - 1] =
      ascii::Char::digit(digit).unwrap_or(ascii::Char::Null);
    exponent /= 10;
    exp_cur -= 1;
  }

  if exp_cur == EXP_LEN {
    exp_buf[EXP_LEN - 1] = ascii::Char::Digit0;
    exp_cur = EXP_LEN - 1;
  }

  exp_buf[exp_cur - 1] = if exp_is_negative {
    ascii::Char::HyphenMinus
  } else {
    ascii::Char::PlusSign
  };
  exp_cur -= 1;

  let mut trailing_zeroes = 0usize;

  let exponent_mark = if (ctype.casemap.islower)(arg.specifier as u32) {
    ascii::Char::SmallP
  } else {
    ascii::Char::CapitalP
  };

  let x = if (ctype.casemap.islower)(arg.specifier as u32) {
    ascii::Char::SmallX
  } else {
    ascii::Char::CapitalX
  };
  let prefix = &[ascii::Char::Digit0, x];

  if (prec as usize) > mant_digits - 1 {
    trailing_zeroes = (prec as usize) - (mant_digits - 1);
  }

  let print_radixchar =
    (mant_digits > 1 || arg.flags.alternate_form) && decimal_point != '\0';

  let exp_off: usize = EXP_LEN - exp_cur;
  let width: usize = usize::from(sign.is_some()) +
    prefix.len() +
    mant_digits +
    trailing_zeroes +
    usize::from(print_radixchar) +
    1 +
    exp_off;

  if arg.flags.left_align {
    if let Some(s) = sign {
      emitter.emit_ascii_char(s)?;
    }
    emitter.emit_ascii_slice(prefix)?;
    emitter.emit_ascii_char(mant_buf[0])?;
    if print_radixchar {
      emitter.emit_unicode_char(decimal_point)?;
    }
    if mant_digits > 1 {
      emitter.emit_ascii_slice(&mant_buf[1..mant_digits])?;
    }
    for _ in 0..trailing_zeroes {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    emitter.emit_ascii_char(exponent_mark)?;
    emitter.emit_ascii_slice(&exp_buf[exp_cur..])?;
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  } else {
    if !arg.flags.leading_zeroes {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
    if let Some(s) = sign {
      emitter.emit_ascii_char(s)?;
    }
    emitter.emit_ascii_slice(prefix)?;
    if arg.flags.leading_zeroes {
      emitter.pad_to(arg, ascii::Char::Digit0, width)?;
    }
    emitter.emit_ascii_char(mant_buf[0])?;
    if print_radixchar {
      emitter.emit_unicode_char(decimal_point)?;
    }
    if mant_digits > 1 {
      emitter.emit_ascii_slice(&mant_buf[1..mant_digits])?;
    }
    for _ in 0..trailing_zeroes {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    emitter.emit_ascii_char(exponent_mark)?;
    emitter.emit_ascii_slice(&exp_buf[exp_cur..])?;
  }

  Ok(())
}

#[inline]
pub fn format_float_decimal<E: Emitter>(
  emitter: &mut E,
  num: FloatValue,
  conv: FloatConv,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  match num {
    | FloatValue::Double(v) => {
      if !v.is_finite() {
        format_non_finite(emitter, v, arg, ctype)
      } else {
        format_float_ryu(emitter, v, conv, arg, ctype, numeric)
      }
    },
    | FloatValue::LongDouble(v) => {
      if !v.is_finite() {
        format_non_finite(emitter, v, arg, ctype)
      } else {
        let e = v.get_explicit_exponent();
        let use_ryu = (f64::MIN_EXP - 1..=f64::MAX_EXP - 1).contains(&e);
        if use_ryu {
          let f = v.cast_to_double();
          format_float_ryu(emitter, f, conv, arg, ctype, numeric)
        } else {
          format_float_dragon(emitter, v, conv, arg, ctype, numeric)
        }
      }
    },
  }
}

#[inline]
pub fn format_float_hexadecimal<E: Emitter>(
  emitter: &mut E,
  num: FloatValue,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  match num {
    | FloatValue::Double(v) => {
      if !v.is_finite() {
        format_non_finite(emitter, v, arg, ctype)
      } else {
        format_float_hexadecimal_typed(emitter, v, arg, ctype, numeric)
      }
    },
    | FloatValue::LongDouble(v) => {
      if !v.is_finite() {
        format_non_finite(emitter, v, arg, ctype)
      } else {
        format_float_hexadecimal_typed(emitter, v, arg, ctype, numeric)
      }
    },
  }
}
