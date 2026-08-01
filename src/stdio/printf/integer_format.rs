use {
  super::Argument,
  crate::{
    intmax_t,
    stdio::{
      format::{FormatError, Signed, Unsigned},
      grouping::NumericGrouping,
      printf::Emitter
    },
    support::{
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::itoa
    },
    uintmax_t
  },
  core::ascii
};

#[inline]
pub fn format_signed<E: Emitter>(
  emitter: &mut E,
  num: Signed,
  arg: &Argument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let is_lowercase = (ctype.casemap.islower)(arg.specifier as u32);
  let num: intmax_t = match num {
    | Signed::Byte(x) => x.into(),
    | Signed::Short(x) => x.into(),
    | Signed::Int(x) => x.into(),
    | Signed::Long(x) => x.into(),
    | Signed::LongLong(x) => x.into(),
    | Signed::Size(x) => x as intmax_t,
    | Signed::Intmax(x) => x as intmax_t,
    | Signed::Ptrdiff(x) => x as intmax_t
  };

  let mut buffer = [ascii::Char::Null; size_of::<intmax_t>() * 8];
  let mut prefix = [ascii::Char::Null; 3];
  let mut prefix_len = 0usize;

  let result = itoa::format_signed(
    num,
    itoa::ItoaFormat::Decimal,
    &mut buffer,
    is_lowercase
  );
  let result_len = result.len();
  let mut ndigits = result_len;

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let use_grouping =
    arg.flags.group_decimals && !grouping.is_empty() && thousands_sep != '\0';
  let thousands_sep_len = E::get_unicode_char_len(thousands_sep).max(1);

  let mut grouping = NumericGrouping::new(grouping, ndigits);
  ndigits += grouping.width * thousands_sep_len;

  // Slice new result buffer
  let result = &buffer[..ndigits];

  let sign = if num.is_negative() {
    Some(ascii::Char::HyphenMinus)
  } else if arg.flags.prepend_plus {
    Some(ascii::Char::PlusSign)
  } else if arg.flags.space_prefix {
    Some(ascii::Char::Space)
  } else {
    None
  };

  if let Some(s) = sign {
    prefix[0] = s;
    prefix_len = 1;
  }

  let mut zeroes: usize;
  let spaces: usize;

  if let Some(p) = arg.precision {
    if num == 0 && p == 0 {
      ndigits = 0;
    }
    zeroes = (p as usize).saturating_sub(ndigits);
    let total_content = prefix_len + zeroes + ndigits;
    spaces = arg.width.saturating_sub(total_content);
    if arg.flags.left_align {
      zeroes = 0;
    }
  } else {
    if arg.flags.leading_zeroes && !arg.flags.left_align {
      let total_content = prefix_len + ndigits;
      zeroes = arg.width.saturating_sub(total_content);
      spaces = 0;
    } else {
      zeroes = 0;
      let total_content = prefix_len + ndigits;
      spaces = arg.width.saturating_sub(total_content);
    }
  }

  if arg.flags.left_align {
    if prefix_len > 0 {
      emitter.emit_ascii_slice(&prefix[..prefix_len])?;
    }
    if zeroes > 0 {
      emitter.pad_to(ascii::Char::Digit0, zeroes)?;
    }
    if ndigits > 0 {
      emitter.emit_ascii_slice(&result[..ndigits])?;
    }
    if spaces > 0 {
      emitter.pad_to(ascii::Char::Space, spaces)?;
    }
  } else {
    if spaces > 0 {
      emitter.pad_to(ascii::Char::Space, spaces)?;
    }
    if prefix_len > 0 {
      emitter.emit_ascii_slice(&prefix[..prefix_len])?;
    }
    if zeroes > 0 {
      emitter.pad_to(ascii::Char::Digit0, zeroes)?;
    }
    if !use_grouping {
      if ndigits > 0 {
        emitter.emit_ascii_slice(&buffer[..result_len])?;
      }
    } else {
      for d in &result[..result_len] {
        if grouping.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
        emitter.emit_ascii_char(*d)?;
      }
    }
  }

  Ok(())
}

#[inline]
pub fn format_unsigned<E: Emitter>(
  emitter: &mut E,
  num: Unsigned,
  arg: &Argument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let is_lowercase = (ctype.casemap.islower)(arg.specifier as u32);
  let num: uintmax_t = match num {
    | Unsigned::Byte(x) => x.into(),
    | Unsigned::Short(x) => x.into(),
    | Unsigned::Int(x) => x.into(),
    | Unsigned::Long(x) => x.into(),
    | Unsigned::LongLong(x) => x.into(),
    | Unsigned::Size(x) => x as uintmax_t,
    | Unsigned::Intmax(x) => x as uintmax_t,
    | Unsigned::Ptrdiff(x) => x as uintmax_t
  };

  let mut buffer = [ascii::Char::Null; size_of::<uintmax_t>() * 8];
  let mut prefix = [ascii::Char::Null; 3];
  let mut prefix_len = 0usize;

  let fmt = match arg.specifier {
    | 'b' | 'B' => itoa::ItoaFormat::Binary,
    | 'x' | 'X' => itoa::ItoaFormat::Hexadecimal,
    | 'o' => itoa::ItoaFormat::Octal,
    | _ => itoa::ItoaFormat::Decimal
  };

  let result =
    itoa::format_unsigned(num, fmt.clone(), &mut buffer, is_lowercase);
  let result_len = result.len();
  let mut ndigits = result_len;

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    fmt == itoa::ItoaFormat::Decimal;

  let thousands_sep_len = if size_of::<E::FormatChar>() == 1 {
    thousands_sep.len_utf8()
  } else {
    1usize
  };

  let mut grouping = NumericGrouping::new(grouping, ndigits);
  ndigits += grouping.width * thousands_sep_len;

  // Slice new result buffer
  let result = &buffer[..ndigits];

  if arg.flags.alternate_form && num != 0 {
    let spec_lower = (ctype.casemap.tolower)(arg.specifier as u32);
    if spec_lower == 'x' as u32 {
      let conv =
        if is_lowercase { ascii::Char::SmallX } else { ascii::Char::CapitalX };
      prefix[0] = ascii::Char::Digit0;
      prefix[1] = conv;
      prefix_len = 2;
    } else if spec_lower == 'b' as u32 {
      let conv =
        if is_lowercase { ascii::Char::SmallB } else { ascii::Char::CapitalB };
      prefix[0] = ascii::Char::Digit0;
      prefix[1] = conv;
      prefix_len = 2;
    } else if arg.specifier == 'o' {
      prefix[0] = ascii::Char::Digit0;
      prefix_len = 1;
    }
  }

  let mut zeroes: usize;
  let spaces: usize;

  if let Some(p) = arg.precision {
    if num == 0 && p == 0 {
      ndigits = 0;
    }
    zeroes = (p as usize).saturating_sub(ndigits);
    let total_content = prefix_len + zeroes + ndigits;
    spaces = arg.width.saturating_sub(total_content);
    if arg.flags.left_align {
      zeroes = 0;
    }
  } else {
    if arg.flags.leading_zeroes && !arg.flags.left_align {
      let total_content = prefix_len + ndigits;
      zeroes = arg.width.saturating_sub(total_content);
      spaces = 0;
    } else {
      zeroes = 0;
      let total_content = prefix_len + ndigits;
      spaces = arg.width.saturating_sub(total_content);
    }
  }

  if arg.flags.left_align {
    if prefix_len > 0 {
      emitter.emit_ascii_slice(&prefix[..prefix_len])?;
    }
    if zeroes > 0 {
      emitter.pad_to(ascii::Char::Digit0, zeroes)?;
    }
    if ndigits > 0 {
      emitter.emit_ascii_slice(&result[..ndigits])?;
    }
    if spaces > 0 {
      emitter.pad_to(ascii::Char::Space, spaces)?;
    }
  } else {
    if spaces > 0 {
      emitter.pad_to(ascii::Char::Space, spaces)?;
    }
    if prefix_len > 0 {
      emitter.emit_ascii_slice(&prefix[..prefix_len])?;
    }
    if zeroes > 0 {
      emitter.pad_to(ascii::Char::Digit0, zeroes)?;
    }
    if !use_grouping {
      if ndigits > 0 {
        emitter.emit_ascii_slice(&buffer[..result_len])?;
      }
    } else {
      for d in &result[..result_len] {
        if grouping.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
        emitter.emit_ascii_char(*d)?;
      }
    }
  }

  Ok(())
}
