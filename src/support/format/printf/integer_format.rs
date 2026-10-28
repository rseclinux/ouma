use {
  super::{
    Emitter,
    PrintfArgument,
    values::{Signed, Unsigned}
  },
  crate::{
    support::{
      format::{error::FormatError, grouping::NumericGrouping},
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::itoa
    },
    types::{intmax_t, uintmax_t}
  },
  core::{ascii, mem}
};

#[inline]
pub fn format_signed<E: Emitter>(
  emitter: &mut E,
  value: Signed,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let value: intmax_t = match value {
    | Signed::Byte(v) => v as intmax_t,
    | Signed::Short(v) => v as intmax_t,
    | Signed::Int(v) => v as intmax_t,
    | Signed::Long(v) => v as intmax_t,
    | Signed::LongLong(v) => v as intmax_t,
    | Signed::Intmax(v) => v,
    | Signed::Size(v) => v as intmax_t,
    | Signed::Pointer(v) => v as intmax_t
  };

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let lowercase = (ctype.casemap.islower)(arg.specifier as u32);

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];
  let use_grouping =
    arg.flags.group_decimals && !grouping.is_empty() && thousands_sep != '\0';

  let mut sign: Option<ascii::Char> = None;
  if value.is_negative() {
    sign = Some(ascii::Char::HyphenMinus);
  } else if arg.flags.prepend_plus {
    sign = Some(ascii::Char::PlusSign);
  } else if arg.flags.space_prefix {
    sign = Some(ascii::Char::Space);
  }

  let mut prefix = [ascii::Char::Null; 2];
  let mut prefix_len = 0usize;
  if let Some(s) = sign {
    prefix[0] = s;
    prefix_len = 1;
  }

  const ITOA_BUFLEN: usize = mem::size_of::<intmax_t>();
  let mut buffer = [ascii::Char::Null; ITOA_BUFLEN * 8];

  let result = itoa::format_integer(
    value.unsigned_abs(),
    itoa::ItoaFormat::Decimal,
    &mut buffer,
    lowercase
  );

  let digits: &[ascii::Char] =
    if result.is_empty() { &[ascii::Char::Digit0] } else { &result[..] };
  let ndigits =
    if value == 0 && arg.precision == Some(0) { 0 } else { digits.len() };

  let mut width = ndigits;
  let mut ng = NumericGrouping::new(grouping, width);

  if use_grouping {
    width += ng.width * E::get_unicode_char_len(thousands_sep).max(1);
  }
  if let Some(prec) = arg.precision &&
    width < (prec as usize)
  {
    width = prec as usize;
  }
  width += prefix_len;

  if arg.flags.leading_zeroes && arg.precision.is_none() {
    for i in 0..prefix_len {
      emitter.emit_ascii_char(prefix[i])?;
    }
    emitter.pad_to(arg, ascii::Char::Digit0, width)?;
  } else {
    if !arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
    for i in 0..prefix_len {
      emitter.emit_ascii_char(prefix[i])?;
    }
  }

  let mut prec = arg.precision.unwrap_or(0) as usize;
  while prec > ndigits {
    emitter.emit_ascii_char(ascii::Char::Digit0)?;
    prec -= 1;
  }

  for i in 0..ndigits {
    if use_grouping {
      if ng.step() {
        emitter.emit_unicode_char(thousands_sep)?;
      }
    }
    emitter.emit_ascii_char(digits[i])?;
  }

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  }

  Ok(())
}

#[inline]
pub fn format_unsigned<E: Emitter>(
  emitter: &mut E,
  value: Unsigned,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let value: uintmax_t = match value {
    | Unsigned::Byte(v) => v as uintmax_t,
    | Unsigned::Short(v) => v as uintmax_t,
    | Unsigned::Int(v) => v as uintmax_t,
    | Unsigned::Long(v) => v as uintmax_t,
    | Unsigned::LongLong(v) => v as uintmax_t,
    | Unsigned::Intmax(v) => v,
    | Unsigned::Size(v) => v as uintmax_t,
    | Unsigned::Pointer(v) => v as uintmax_t
  };

  let fmt = match arg.specifier {
    | 'b' | 'B' => itoa::ItoaFormat::Binary,
    | 'o' => itoa::ItoaFormat::Octal,
    | 'x' | 'X' => itoa::ItoaFormat::Hexadecimal,
    | _ => itoa::ItoaFormat::Decimal
  };

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let lowercase = (ctype.casemap.islower)(arg.specifier as u32);

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];
  let use_grouping = arg.flags.group_decimals &&
    !grouping.is_empty() &&
    thousands_sep != '\0' &&
    fmt == itoa::ItoaFormat::Decimal;

  let mut sign: Option<ascii::Char> = None;
  if arg.flags.prepend_plus {
    sign = Some(ascii::Char::PlusSign);
  } else if arg.flags.space_prefix {
    sign = Some(ascii::Char::Space);
  }

  let mut prefix = [ascii::Char::Null; 2];
  let mut prefix_len = 0usize;

  if fmt == itoa::ItoaFormat::Binary && arg.flags.alternate_form && value != 0 {
    prefix[0] = ascii::Char::Digit0;
    prefix[1] =
      if lowercase { ascii::Char::SmallB } else { ascii::Char::CapitalB };
    prefix_len = 2;
  } else if fmt == itoa::ItoaFormat::Octal &&
    arg.flags.alternate_form &&
    value != 0
  {
    prefix[0] = ascii::Char::Digit0;
    prefix_len = 1;
  } else if fmt == itoa::ItoaFormat::Hexadecimal &&
    arg.flags.alternate_form &&
    value != 0
  {
    prefix[0] = ascii::Char::Digit0;
    prefix[1] =
      if lowercase { ascii::Char::SmallX } else { ascii::Char::CapitalX };
    prefix_len = 2;
  } else if let Some(s) = sign {
    prefix[0] = s;
    prefix_len = 1;
  }

  const ITOA_BUFLEN: usize = mem::size_of::<uintmax_t>();
  let mut buffer = [ascii::Char::Null; ITOA_BUFLEN * 8];

  let result = itoa::format_integer(value, fmt, &mut buffer, lowercase);

  let digits: &[ascii::Char] =
    if result.is_empty() { &[ascii::Char::Digit0] } else { &result[..] };
  let ndigits =
    if value == 0 && arg.precision == Some(0) { 0 } else { digits.len() };

  let mut width = ndigits;
  let mut ng = NumericGrouping::new(grouping, width);

  if use_grouping {
    width += ng.width * E::get_unicode_char_len(thousands_sep).max(1);
  }
  if let Some(prec) = arg.precision &&
    width < (prec as usize)
  {
    width = prec as usize;
  }
  width += prefix_len;

  if arg.flags.leading_zeroes && arg.precision.is_none() {
    for i in 0..prefix_len {
      emitter.emit_ascii_char(prefix[i])?;
    }
    emitter.pad_to(arg, ascii::Char::Digit0, width)?;
  } else {
    if !arg.flags.left_align {
      emitter.pad_to(arg, ascii::Char::Space, width)?;
    }
    for i in 0..prefix_len {
      emitter.emit_ascii_char(prefix[i])?;
    }
  }

  let mut prec = arg.precision.unwrap_or(0) as usize;
  while prec > ndigits {
    emitter.emit_ascii_char(ascii::Char::Digit0)?;
    prec -= 1;
  }

  for i in 0..ndigits {
    if use_grouping {
      if ng.step() {
        emitter.emit_unicode_char(thousands_sep)?;
      }
    }
    emitter.emit_ascii_char(digits[i])?;
  }

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, width)?;
  }

  Ok(())
}
