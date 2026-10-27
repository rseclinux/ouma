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
  num: T,
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

  let mut precision = arg.precision.unwrap_or(6);
  let mut e_mode = false;

  let sign = if num.is_sign_negative() {
    Some(ascii::Char::HyphenMinus)
  } else if arg.flags.prepend_plus {
    Some(ascii::Char::PlusSign)
  } else if arg.flags.space_prefix {
    Some(ascii::Char::Space)
  } else {
    None
  };

  if conv == FloatConv::Scientific {
    precision = T::DECIMAL_DIG.min(precision + 1);
    e_mode = true;
  } else if conv == FloatConv::Automatic {
    precision = precision.max(1);
    precision = T::DECIMAL_DIG.min(precision);
  }

  let mut result = ftoa::DragonReturn::default();
  if num.is_zero() {
    result.digits[0] = ascii::Char::Digit0;
    result.ndigits = 1;
    result.exponenta = 1;
  } else if conv == FloatConv::Fixed {
    result = ftoa::format_dragon4(num, precision as i32, get_rounding());
  } else {
    result = ftoa::format_dragon4_exp(num, precision as i32, get_rounding());
  }
  let mut ndigits = result.ndigits;
  let exponenta = result.exponenta;
  let x = exponenta - 1;

  if conv == FloatConv::Automatic {
    let save = precision;

    if !arg.flags.alternate_form {
      // remove trailing zeroes
      while ndigits > 1 && result.digits[ndigits - 1] == ascii::Char::Digit0 {
        ndigits -= 1;
      }
      precision = ndigits as u32;
    }

    // Guess if we need scientific mode or we're good with fixed one
    if -4 <= x && x < (save as i32) {
      precision = (precision as i32 - (x + 1)).max(0) as u32;
    } else {
      e_mode = true;
      precision = precision.saturating_sub(1);
    }
  } else if conv == FloatConv::Scientific {
    precision = precision.saturating_sub(1);
  }

  let mut left_digits_with_grouping =
    if exponenta >= 1 { exponenta as usize } else { 1 };
  let mut ng = NumericGrouping::new(grouping, left_digits_with_grouping);

  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    conv != FloatConv::Scientific;

  if use_grouping {
    left_digits_with_grouping +=
      ng.width * E::get_unicode_char_len(thousands_sep).max(1);
  }

  let mut width: usize;
  if e_mode {
    let mut ed: usize = 2;
    let mut t = x.unsigned_abs() / 100;
    while t > 0 {
      ed += 1;
      t /= 10;
    }
    width = 3 + ed;
  } else {
    width = left_digits_with_grouping;
  }

  if sign.is_some() {
    width += 1;
  }
  if precision != 0 {
    width += (precision + 1) as usize;
  } else if arg.flags.alternate_form {
    width += 1;
  }

  let zero_pad = arg.flags.leading_zeroes && !arg.flags.left_align;

  if !arg.flags.left_align && !zero_pad {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  }

  if let Some(s) = sign {
    emitter.emit_ascii_char(s)?;
  }

  if zero_pad {
    emitter.pad_to(arg, ascii::Char::Digit0, width)?;
  }

  if !e_mode {
    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';

    let mut position: isize;
    let mut idx: isize;
    if exponenta >= 1 {
      position = -(exponenta as isize);
      idx = 0;
    } else {
      position = -1;
      idx = (exponenta - 1) as isize;
    }

    while position < (precision as isize) {
      if position < 0 && use_grouping {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
      }

      let digit = if idx >= 0 && (idx as usize) < ndigits {
        result.digits[idx as usize]
      } else {
        ascii::Char::Digit0
      };

      emitter.emit_ascii_char(digit)?;

      idx += 1;
      position += 1;

      if position == 0 && print_radixchar {
        emitter.emit_unicode_char(decimal_point)?;
      }
    }
  } else {
    let negative_exponent = x < 0;

    let mut itoa_buf = [ascii::Char::Null; size_of::<intmax_t>() * 8];

    let itoa_result = itoa::format_integer(
      x,
      itoa::ItoaFormat::Decimal,
      &mut itoa_buf,
      lowercase
    );

    let exponent_mark =
      if lowercase { ascii::Char::SmallE } else { ascii::Char::CapitalE };

    if result.ndigits == 0 {
      result.digits[0] = ascii::Char::Digit0;
      ndigits = 1;
    }

    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';

    emitter.emit_ascii_char(result.digits[0])?;
    if print_radixchar {
      emitter.emit_unicode_char(decimal_point)?;
    }
    for i in 1..ndigits {
      emitter.emit_ascii_char(result.digits[i])?;
    }
    for _ in ndigits..(precision as usize + 1) {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    emitter.emit_ascii_char(exponent_mark)?;
    if negative_exponent {
      emitter.emit_ascii_char(ascii::Char::HyphenMinus)?;
    } else {
      emitter.emit_ascii_char(ascii::Char::PlusSign)?;
    }
    const MIN_EXP_WIDTH: usize = 2;
    let pad_zeroes = MIN_EXP_WIDTH.saturating_sub(itoa_result.len());
    for _ in 0..pad_zeroes {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    for d in itoa_result {
      emitter.emit_ascii_char(*d)?;
    }
  }

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  }

  Ok(())
}

#[inline]
pub fn format_float_ryu<E: Emitter>(
  emitter: &mut E,
  num: f64,
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

  let mut precision = arg.precision.unwrap_or(6);
  let mut e_mode = false;

  let sign = if num.is_sign_negative() {
    Some(ascii::Char::HyphenMinus)
  } else if arg.flags.prepend_plus {
    Some(ascii::Char::PlusSign)
  } else if arg.flags.space_prefix {
    Some(ascii::Char::Space)
  } else {
    None
  };

  if conv == FloatConv::Scientific {
    precision = f64::DECIMAL_DIG.min(precision + 1);
    e_mode = true;
  } else if conv == FloatConv::Automatic {
    precision = precision.max(1);
    precision = f64::DECIMAL_DIG.min(precision);
  }

  let mut result = ryu::RyuReturn::default();
  if num.is_zero() {
    result.digits[0] = ascii::Char::Digit0;
    result.ndigits = 1;
    result.exponenta = 0;
  } else if conv == FloatConv::Fixed {
    result = ryu::format_ryu(num, precision as i32, get_rounding());
  } else {
    result = ryu::format_ryu_exp(num, precision as i32, get_rounding());
  }
  let mut ndigits = result.ndigits;
  let exponenta = result.exponenta;
  let x = exponenta + 1;

  if conv == FloatConv::Automatic {
    let save = precision;

    if !arg.flags.alternate_form {
      // remove trailing zeroes
      while ndigits > 1 && result.digits[ndigits - 1] == ascii::Char::Digit0 {
        ndigits -= 1;
      }
      precision = ndigits as u32;
    }

    // Guess if we need scientific mode or we're good with fixed one
    if -4 <= exponenta && exponenta < (save as i32) {
      precision = (precision as i32 - (exponenta + 1)).max(0) as u32;
    } else {
      e_mode = true;
      precision = precision.saturating_sub(1);
    }
  } else if conv == FloatConv::Scientific {
    precision = precision.saturating_sub(1);
  }

  let mut left_digits_with_grouping = x.max(1) as usize;
  let mut ng = NumericGrouping::new(grouping, left_digits_with_grouping);

  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    conv != FloatConv::Scientific;

  if use_grouping {
    left_digits_with_grouping +=
      ng.width * E::get_unicode_char_len(thousands_sep).max(1);
  }

  let mut width: usize;
  if e_mode {
    let mut ed: usize = 2;
    let mut t = exponenta.unsigned_abs() / 100;
    while t > 0 {
      ed += 1;
      t /= 10;
    }
    width = 3 + ed;
  } else {
    width = left_digits_with_grouping;
  }

  if sign.is_some() {
    width += 1;
  }
  if precision != 0 {
    width += (precision + 1) as usize;
  } else if arg.flags.alternate_form {
    width += 1;
  }

  let zero_pad = arg.flags.leading_zeroes && !arg.flags.left_align;

  if !arg.flags.left_align && !zero_pad {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  }

  if let Some(s) = sign {
    emitter.emit_ascii_char(s)?;
  }

  if zero_pad {
    emitter.pad_to(arg, ascii::Char::Digit0, width)?;
  }

  if !e_mode {
    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';

    let mut position: isize;
    let mut idx: isize;
    if x >= 1 {
      position = -(x as isize);
      idx = 0;
    } else {
      position = -1;
      idx = (x - 1) as isize;
    }

    while position < (precision as isize) {
      if position < 0 && use_grouping {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
      }

      let digit = if idx >= 0 && (idx as usize) < ndigits {
        result.digits[idx as usize]
      } else {
        ascii::Char::Digit0
      };

      emitter.emit_ascii_char(digit)?;

      idx += 1;
      position += 1;

      if position == 0 && print_radixchar {
        emitter.emit_unicode_char(decimal_point)?;
      }
    }
  } else {
    let negative_exponent = exponenta < 0;

    let mut itoa_buf = [ascii::Char::Null; size_of::<intmax_t>() * 8];

    let itoa_result = itoa::format_integer(
      exponenta,
      itoa::ItoaFormat::Decimal,
      &mut itoa_buf,
      lowercase
    );

    let exponent_mark =
      if lowercase { ascii::Char::SmallE } else { ascii::Char::CapitalE };

    if result.ndigits == 0 {
      result.digits[0] = ascii::Char::Digit0;
      ndigits = 1;
    }

    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';

    emitter.emit_ascii_char(result.digits[0])?;
    if print_radixchar {
      emitter.emit_unicode_char(decimal_point)?;
    }
    for i in 1..ndigits {
      emitter.emit_ascii_char(result.digits[i])?;
    }
    for _ in ndigits..(precision as usize + 1) {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    emitter.emit_ascii_char(exponent_mark)?;
    if negative_exponent {
      emitter.emit_ascii_char(ascii::Char::HyphenMinus)?;
    } else {
      emitter.emit_ascii_char(ascii::Char::PlusSign)?;
    }
    const MIN_EXP_WIDTH: usize = 2;
    let pad_zeroes = MIN_EXP_WIDTH.saturating_sub(itoa_result.len());
    for _ in 0..pad_zeroes {
      emitter.emit_ascii_char(ascii::Char::Digit0)?;
    }
    for d in itoa_result {
      emitter.emit_ascii_char(*d)?;
    }
  }

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
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
