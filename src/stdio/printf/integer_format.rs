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

const DEC_FMT_SIGNED_SIZE: usize = itoa::buffer_size::<intmax_t, 10>();
const BIN_FMT_UNSIGNED_SIZE: usize = itoa::buffer_size::<uintmax_t, 2>();
const OCT_FMT_UNSIGNED_SIZE: usize = itoa::buffer_size::<uintmax_t, 8>();
const DEC_FMT_UNSIGNED_SIZE: usize = itoa::buffer_size::<uintmax_t, 10>();
const HEX_FMT_UNSIGNED_SIZE: usize = itoa::buffer_size::<uintmax_t, 16>();

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

  let mut buffer = [ascii::Char::Null; DEC_FMT_SIGNED_SIZE];
  let mut prefix = [ascii::Char::Null; 3];
  let mut prefix_len = 0usize;

  let result = itoa::format_signed(
    num,
    itoa::ItoaFormat::Decimal,
    &mut buffer,
    is_lowercase
  );
  let mut ndigits = result.len();

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let use_grouping =
    arg.flags.group_decimals && !grouping.is_empty() && thousands_sep != '\0';

  let group_width_for = |total_digits: usize| -> usize {
    if use_grouping && total_digits > 0 {
      NumericGrouping::new(grouping, total_digits).width
    } else {
      0
    }
  };

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
    let group_width = group_width_for(zeroes + ndigits);
    let total_content = prefix_len + zeroes + ndigits + group_width;
    spaces = arg.width.saturating_sub(total_content);
    if arg.flags.left_align {
      zeroes = 0;
    }
  } else {
    if arg.flags.leading_zeroes && !arg.flags.left_align {
      let mut z = arg.width.saturating_sub(prefix_len + ndigits);
      let mut iterations_left = arg.width.saturating_add(1);
      loop {
        let gw = group_width_for(z + ndigits);
        let target_total = prefix_len + z + ndigits + gw;
        if target_total <= arg.width || iterations_left == 0 {
          break;
        }
        let overshoot = target_total - arg.width;
        z = z.saturating_sub(overshoot);
        iterations_left -= 1;
      }
      zeroes = z;
      spaces = 0;
    } else {
      zeroes = 0;
      let group_width = group_width_for(ndigits);
      let total_content = prefix_len + ndigits + group_width;
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
    if !use_grouping {
      if zeroes > 0 {
        emitter.pad_to(ascii::Char::Digit0, zeroes)?;
      }
      if ndigits > 0 {
        emitter.emit_ascii_slice(&result[..ndigits])?;
      }
    } else {
      let mut ng = NumericGrouping::new(grouping, zeroes + ndigits);
      for _ in 0..zeroes {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
        emitter.emit_ascii_char(ascii::Char::Digit0)?;
      }
      for d in &result[..ndigits] {
        if ng.step() {
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

  let mut buffer = [ascii::Char::Null; 256];
  let mut prefix = [ascii::Char::Null; 3];
  let mut prefix_len = 0usize;

  let (buffer_slice, fmt) = match arg.specifier {
    | 'b' | 'B' => {
      (&mut buffer[..BIN_FMT_UNSIGNED_SIZE], itoa::ItoaFormat::Binary)
    },
    | 'x' | 'X' => {
      (&mut buffer[..HEX_FMT_UNSIGNED_SIZE], itoa::ItoaFormat::Hexadecimal)
    },
    | 'o' => (&mut buffer[..OCT_FMT_UNSIGNED_SIZE], itoa::ItoaFormat::Octal),
    | _ => (&mut buffer[..DEC_FMT_UNSIGNED_SIZE], itoa::ItoaFormat::Decimal)
  };

  let result =
    itoa::format_unsigned(num, fmt.clone(), buffer_slice, is_lowercase);
  let mut ndigits = result.len();

  let grouping = numeric.grouping.as_slice();
  let gl = grouping.iter().copied().take_while(|&x| x != b'\0').count();
  let grouping = &grouping[..gl];

  let thousands_sep: char = numeric.get_thousands_sep().unwrap_or('\0');
  let use_grouping = arg.flags.group_decimals &&
    fmt == itoa::ItoaFormat::Decimal &&
    !grouping.is_empty() &&
    thousands_sep != '\0';

  let group_width_for = |total_digits: usize| -> usize {
    if use_grouping && total_digits > 0 {
      NumericGrouping::new(grouping, total_digits).width
    } else {
      0
    }
  };

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
    let group_width = group_width_for(zeroes + ndigits);
    let total_content = prefix_len + zeroes + ndigits + group_width;
    spaces = arg.width.saturating_sub(total_content);
    if arg.flags.left_align {
      zeroes = 0;
    }
  } else {
    if arg.flags.leading_zeroes && !arg.flags.left_align {
      let mut z = arg.width.saturating_sub(prefix_len + ndigits);
      let mut iterations_left = arg.width.saturating_add(1);
      loop {
        let gw = group_width_for(z + ndigits);
        let target_total = prefix_len + z + ndigits + gw;
        if target_total <= arg.width || iterations_left == 0 {
          break;
        }
        let overshoot = target_total - arg.width;
        z = z.saturating_sub(overshoot);
        iterations_left -= 1;
      }
      zeroes = z;
      spaces = 0;
    } else {
      zeroes = 0;
      let group_width = group_width_for(ndigits);
      let total_content = prefix_len + ndigits + group_width;
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
    if !use_grouping {
      if zeroes > 0 {
        emitter.pad_to(ascii::Char::Digit0, zeroes)?;
      }
      if ndigits > 0 {
        emitter.emit_ascii_slice(&result[..ndigits])?;
      }
    } else {
      let mut ng = NumericGrouping::new(grouping, zeroes + ndigits);
      for _ in 0..zeroes {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
        emitter.emit_ascii_char(ascii::Char::Digit0)?;
      }
      for d in &result[..ndigits] {
        if ng.step() {
          emitter.emit_unicode_char(thousands_sep)?;
        }
        emitter.emit_ascii_char(*d)?;
      }
    }
  }

  Ok(())
}
