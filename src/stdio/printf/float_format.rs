use {
  super::{Argument, Emitter},
  crate::{
    stdio::{
      format::{Float, FormatError},
      grouping::NumericGrouping
    },
    support::{
      float::rounding_mode::quick_get_round,
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::{
        ftoa::{self, DragonFloat},
        ryu
      },
      traits::float::FloatBits
    }
  },
  core::ascii
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatConv {
  E,
  F,
  G
}

fn format_float_dragon4<T: DragonFloat, E: Emitter>(
  emitter: &mut E,
  num: T,
  conv: &FloatConv,
  arg: &Argument,
  _ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError>
where
  u8: bnum::cast::CastFrom<T::Bn>,
  T::Bn: bnum::cast::CastFrom<T::StorageType> {
  let decimal_point: char = numeric.get_decimal_point().unwrap_or('\0');
  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');

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

  if *conv == FloatConv::E {
    precision = T::DECIMAL_DIG.min(precision + 1);
    e_mode = true;
  } else if *conv == FloatConv::G {
    precision = precision.max(1);
    precision = T::DECIMAL_DIG.min(precision);
  }

  let total_prec =
    if *conv == FloatConv::F { T::DECIMAL_DIG } else { precision };

  let ftoa_result =
    ftoa::format_float(num, total_prec as i32, quick_get_round());
  let mut ndigits = ftoa_result.ndigits;
  let exponenta = ftoa_result.exponenta;

  if *conv == FloatConv::G {
    let save = precision;

    if !arg.flags.alternate_form {
      // remove trailing zeroes
      while ndigits > 1 &&
        ftoa_result.digits[ndigits - 1] == ascii::Char::Digit0
      {
        ndigits -= 1;
      }
      precision = ndigits as u32;
    }

    // Guess if we need scientific mode or we're good with fixed one
    if -4 <= exponenta && exponenta < (save as i32) {
      if exponenta < (precision as i32) {
        precision = precision - (exponenta + 1) as u32;
      } else {
        precision = 0;
      }
    } else {
      e_mode = true;
      precision = precision.saturating_sub(1);
    }
  }

  let mut width: usize;
  if e_mode {
    width = 3;
    width += (exponenta) as usize;
  } else {
    width = if exponenta > 0 { (exponenta + 1) as usize } else { 1 };
  }

  let _use_grouping =
    arg.flags.group_decimals && !grouping.is_empty() && thousands_sep != '\0';
  let thousands_sep_len = E::get_unicode_char_len(thousands_sep).max(1);

  let grouping = NumericGrouping::new(grouping, width);
  width += grouping.width * thousands_sep_len;

  if sign.is_some() {
    width += 1;
  }
  if precision != 0 {
    width += (precision + 1) as usize;
  } else if arg.flags.alternate_form {
    width += 1;
  }

  let total_width = if arg.width > width { arg.width - width } else { 0 };

  if !(arg.flags.left_align || arg.flags.leading_zeroes) {
    if total_width > 0 {
      emitter.pad_to(ascii::Char::Space, total_width)?;
    }
  }

  if let Some(s) = sign {
    emitter.emit_ascii_char(s)?;
  }

  if arg.flags.leading_zeroes {
    if total_width > 0 {
      emitter.pad_to(ascii::Char::Digit0, total_width)?;
    }
  }

  if !e_mode {
    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';
    let mut sz: isize = if exponenta > 0 { exponenta as isize } else { 0 };
    let mut c: ascii::Char;

    loop {
      if sz == -1 && decimal_point != '\0' {
        emitter.emit_unicode_char(decimal_point)?;
      }

      if 0 <= exponenta &&
        (exponenta as isize) - sz < (ftoa_result.ndigits as isize)
      {
        let idx: usize = ((exponenta as isize) - sz) as usize;
        c = ftoa_result.digits[idx];
      } else {
        c = ascii::Char::Digit0;
      }

      sz -= 1;
      if sz < -(precision as isize) {
        break;
      }

      emitter.emit_ascii_char(c)?;
    }

    emitter.emit_ascii_char(c)?;
    if print_radixchar && sz == -1 {
      emitter.emit_unicode_char(decimal_point)?;
    }
  } else {
    panic!("Scientific format is not implemented yet :(");
  }

  Ok(())
}

fn format_float_ryu<E: Emitter>(
  emitter: &mut E,
  num: f64,
  conv: &FloatConv,
  arg: &Argument,
  _ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let decimal_point: char = numeric.get_decimal_point().unwrap_or('\0');
  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');

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

  if *conv == FloatConv::E {
    precision = f64::DECIMAL_DIG.min(precision + 1);
    e_mode = true;
  } else if *conv == FloatConv::G {
    precision = precision.max(1);
    precision = f64::DECIMAL_DIG.min(precision);
  }

  let ftoa_result = if *conv == FloatConv::F {
    ryu::format_ryu(num, precision as i32, quick_get_round())
  } else {
    ryu::format_ryu_exp(num, precision as i32, quick_get_round())
  };
  let mut ndigits = ftoa_result.ndigits;
  let exponenta = ftoa_result.exponenta;

  if *conv == FloatConv::G {
    let save = precision;

    if !arg.flags.alternate_form {
      // remove trailing zeroes
      while ndigits > 1 &&
        ftoa_result.digits[ndigits - 1] == ascii::Char::Digit0
      {
        ndigits -= 1;
      }
      precision = ndigits as u32;
    }

    // Guess if we need scientific mode or we're good with fixed one
    if -4 <= exponenta && exponenta < (save as i32) {
      if exponenta < (precision as i32) {
        precision = precision - (exponenta + 1) as u32;
      } else {
        precision = 0;
      }
    } else {
      e_mode = true;
      precision = precision.saturating_sub(1);
    }
  }

  let mut width: usize;
  if e_mode {
    width = 3;
    width += (exponenta) as usize;
  } else {
    width = if exponenta > 0 { (exponenta + 1) as usize } else { 1 };
  }

  let _use_grouping =
    arg.flags.group_decimals && !grouping.is_empty() && thousands_sep != '\0';
  let thousands_sep_len = E::get_unicode_char_len(thousands_sep).max(1);

  let grouping = NumericGrouping::new(grouping, width);
  width += grouping.width * thousands_sep_len;

  if sign.is_some() {
    width += 1;
  }
  if precision != 0 {
    width += (precision + 1) as usize;
  } else if arg.flags.alternate_form {
    width += 1;
  }

  let total_width = if arg.width > width { arg.width - width } else { 0 };

  if !(arg.flags.left_align || arg.flags.leading_zeroes) {
    if total_width > 0 {
      emitter.pad_to(ascii::Char::Space, total_width)?;
    }
  }

  if let Some(s) = sign {
    emitter.emit_ascii_char(s)?;
  }

  if arg.flags.leading_zeroes {
    if total_width > 0 {
      emitter.pad_to(ascii::Char::Digit0, total_width)?;
    }
  }

  if !e_mode {
    let print_radixchar =
      (arg.flags.alternate_form || precision > 0) && decimal_point != '\0';
    let mut sz: isize = if exponenta > 0 { exponenta as isize } else { 0 };
    let mut c: ascii::Char;

    loop {
      if sz == -1 && decimal_point != '\0' {
        emitter.emit_unicode_char(decimal_point)?;
      }

      if 0 <= exponenta &&
        (exponenta as isize) - sz < (ftoa_result.ndigits as isize)
      {
        let idx: usize = ((exponenta as isize) - sz) as usize;
        c = ftoa_result.digits[idx];
      } else {
        c = ascii::Char::Digit0;
      }

      sz -= 1;
      if sz < -(precision as isize) {
        break;
      }

      emitter.emit_ascii_char(c)?;
    }

    emitter.emit_ascii_char(c)?;
    if print_radixchar && sz == -1 {
      emitter.emit_unicode_char(decimal_point)?;
    }
  } else {
    panic!("Scientific format is not implemented yet :(");
  }

  Ok(())
}

pub fn format_float<E: Emitter>(
  emitter: &mut E,
  num: Float,
  conv: &FloatConv,
  arg: &Argument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  match num {
    | Float::Double(x) => {
      if x.is_finite() {
        format_float_ryu(emitter, x, conv, arg, ctype, numeric)
      } else {
        panic!("inf nan not yet implemented :(");
      }
    },
    | Float::LongDouble(x) => {
      if x.is_finite() {
        format_float_dragon4(emitter, x, conv, arg, ctype, numeric)
      } else {
        panic!("inf nan not yet implemented :(");
      }
    },
  }
}
