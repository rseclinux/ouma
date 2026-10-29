use {
  super::{Consumer, ScanfArgument, utils::write_floating_point_value},
  crate::support::{
    format::error::FormatError,
    locale::{ctype::CtypeObject, numeric::NumericObject}
  },
  smallvec::SmallVec
};

const INF_STR: &[u32] = u32str_from_ascii!(b"infinity");

const FLOAT_STR_ARRAY_SIZE: usize = 2048; // Enough to round trip with Ryu

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloatParserState {
  Start,
  MaybeHex,
  Digits,
  Exponent
}

#[inline]
fn try_push_into_slice(
  buf: &mut SmallVec<[u32; FLOAT_STR_ARRAY_SIZE]>,
  value: u32
) -> Result<(), FormatError> {
  if buf.try_reserve_exact(1).is_err() {
    return Err(FormatError::InvalidArg);
  }
  buf.push(value);
  Ok(())
}

#[inline]
pub fn format_float<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  if arg.allocate {
    return Err(FormatError::BadMatch);
  }
  if ptr.is_null() && !arg.suppress {
    return Err(FormatError::InvalidArg);
  }

  let width = if arg.width == 0 || arg.width > FLOAT_STR_ARRAY_SIZE {
    usize::MAX
  } else {
    arg.width
  };
  let decimal_point = numeric.get_decimal_point().unwrap_or('\0');

  let mut buf: SmallVec<[u32; FLOAT_STR_ARRAY_SIZE]> = SmallVec::new();

  loop {
    let c = consumer.consume_u32()?;
    if !(ctype.casemap.isspace)(c) {
      consumer.vomit_u32(c)?;
      break;
    }
  }

  let mut k = 0usize;
  let mut state = FloatParserState::Start;
  let mut got_mant_digits = false;
  let mut is_hex = false;

  while k < width {
    let c_cur = match consumer.consume_u32() {
      | Err(FormatError::EndOfFile) => break,
      | Err(e) => return Err(e),
      | Ok(c) => c
    };

    if (ctype.casemap.isspace)(c_cur) {
      consumer.vomit_u32(c_cur)?;
      break;
    }

    let ch = char::from_u32(c_cur).unwrap_or('\0');

    if state == FloatParserState::MaybeHex && !is_hex {
      state = FloatParserState::Digits;
      if ch.to_ascii_lowercase() == 'x' {
        try_push_into_slice(&mut buf, c_cur)?;
        is_hex = true;
        k += 1;
        continue;
      }
      got_mant_digits = true;
    }

    let parsing_exponent = state == FloatParserState::Exponent;

    if (ch == '+' || ch == '-') && !parsing_exponent {
      try_push_into_slice(&mut buf, c_cur)?;
    } else if ch.to_ascii_lowercase() == 'i' {
      try_push_into_slice(&mut buf, c_cur)?;

      let mut inf_idx = 1usize;

      while k + inf_idx < width {
        if inf_idx > 7 {
          break;
        }

        let c = match consumer.consume_u32() {
          | Err(FormatError::EndOfFile) => break,
          | Err(e) => return Err(e),
          | Ok(c) => c
        };

        if INF_STR[inf_idx] != (ctype.casemap.tolower)(c) {
          break;
        }

        try_push_into_slice(&mut buf, c)?;

        inf_idx += 1;
      }

      return write_floating_point_value(
        consumer, &buf, ptr, arg, ctype, numeric
      );
    } else if ch.to_ascii_lowercase() == 'n' {
      try_push_into_slice(&mut buf, c_cur)?;

      let mut nan_idx = 0usize;

      while k + nan_idx + 1 < width {
        let c = match consumer.consume_u32() {
          | Err(FormatError::EndOfFile) => break,
          | Err(e) => return Err(e),
          | Ok(c) => c
        };

        match nan_idx {
          | 0 => {
            if c != 'a'.into() && c != 'A'.into() {
              break;
            } else {
              try_push_into_slice(&mut buf, c)?;
            }
          },
          | 1 => {
            if c != 'n'.into() && c != 'N'.into() {
              break;
            } else {
              try_push_into_slice(&mut buf, c)?;
            }
          },
          | 2 => {
            if c != '('.into() {
              break;
            } else {
              try_push_into_slice(&mut buf, c)?;
            }
          },
          | _ => {
            if c == ')'.into() {
              try_push_into_slice(&mut buf, c)?;
              break;
            } else if !(ctype.casemap.isalnum)(c) && c != '_'.into() {
              break;
            } else {
              try_push_into_slice(&mut buf, c)?;
            }
          },
        };

        nan_idx += 1;
      }

      if nan_idx > 3 && !buf.ends_with(&[')' as u32]) {
        for _ in (3..buf.len()).rev() {
          if let Some(p) = buf.pop() {
            consumer.vomit_u32(p)?;
          }
        }
      }

      return write_floating_point_value(
        consumer, &buf, ptr, arg, ctype, numeric
      );
    } else if ch == '0' && !parsing_exponent {
      if state == FloatParserState::Start {
        state = FloatParserState::MaybeHex;
      } else {
        state = FloatParserState::Digits;
      }
      try_push_into_slice(&mut buf, c_cur)?;
    } else if !parsing_exponent {
      if ch.is_digit(if is_hex { 16 } else { 10 }) {
        try_push_into_slice(&mut buf, c_cur)?;
        got_mant_digits = true;
      } else {
        if ch == decimal_point {
          try_push_into_slice(&mut buf, c_cur)?;
        } else if ch == '+' || ch == '-' {
          try_push_into_slice(&mut buf, c_cur)?;
          got_mant_digits = false;
        } else if (ch.to_ascii_lowercase() == 'p' && is_hex) ||
          (ch.to_ascii_lowercase() == 'e' && !is_hex)
        {
          if !got_mant_digits {
            consumer.vomit_u32(c_cur)?;
            break;
          }

          try_push_into_slice(&mut buf, c_cur)?;
          state = FloatParserState::Exponent;
        } else {
          consumer.vomit_u32(c_cur)?;
          break;
        }
      }
    } else if state == FloatParserState::Exponent {
      if ch == '-' || ch == '+' || ch.is_digit(10) {
        try_push_into_slice(&mut buf, c_cur)?;
      } else {
        consumer.vomit_u32(c_cur)?;
        break;
      }
    }

    k += 1;
  }

  if is_hex && !got_mant_digits {
    if let Some(idx) =
      buf.iter().rposition(|&c| c == 'x' as u32 || c == 'X' as u32)
    {
      while buf.len() > idx {
        if let Some(c) = buf.pop() {
          consumer.vomit_u32(c)?;
        }
      }
    }
  }

  let ends_with_number =
    buf.last().and_then(|&c| Some((ctype.casemap.isdigit)(c))).unwrap_or(false);

  if state == FloatParserState::Exponent && !ends_with_number {
    let prior_exp = buf
      .iter()
      .copied()
      .rposition(|c| {
        c == 'e'.into() || c == 'E'.into() || c == 'p'.into() || c == 'P'.into()
      })
      .map(|idx| buf.iter().copied().count() - idx)
      .unwrap_or(0);
    let offset = buf.len() - prior_exp;
    for _ in offset..buf.len() {
      if let Some(c) = buf.pop() {
        consumer.vomit_u32(c)?;
      }
    }

    if k >= width {
      return Err(FormatError::BadMatch);
    }
  }

  write_floating_point_value(consumer, &buf, ptr, arg, ctype, numeric)
}
