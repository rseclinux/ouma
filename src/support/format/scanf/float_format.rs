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
fn rollback<C: Consumer>(
  consumer: &mut C,
  buf: &mut SmallVec<[u32; FLOAT_STR_ARRAY_SIZE]>,
  n: usize
) -> Result<(), FormatError> {
  for _ in 0..n {
    if let Some(x) = buf.pop() {
      consumer.vomit_u32(x)?;
    }
  }
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

  let mut after_decimal = false;
  let mut got_exp_mark = false;
  let mut mayhex = false;
  let mut ishex = false;
  let mut seen_digit = false;
  let mut k = 0usize;
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
    let only_sign =
      buf.iter().all(|&b| b == u32::from('+') || b == u32::from('-'));
    let after_exp = got_exp_mark &&
      char::from_u32(buf.last().copied().unwrap_or(0))
        .is_some_and(|c| matches!(c, 'e' | 'E' | 'p' | 'P'));

    if ch == '-' || ch == '+' {
      if k != 0 && !after_exp {
        consumer.vomit_u32(c_cur)?;
        break;
      }
      try_push_into_slice(&mut buf, c_cur)?;
    } else if only_sign && (ctype.casemap.tolower)(ch.into()) == 'i'.into() {
      let avail = width.saturating_sub(k);
      if avail < 3 {
        return Err(FormatError::BadMatch);
      }
      try_push_into_slice(&mut buf, c_cur)?;
      let mut idx = 1usize;
      while idx < INF_STR.len().min(avail) {
        let c = match consumer.consume_u32() {
          | Err(FormatError::EndOfFile) => break,
          | Err(e) => return Err(e),
          | Ok(c) => c
        };
        if (ctype.casemap.tolower)(c) != INF_STR[idx] {
          consumer.vomit_u32(c)?;
          break;
        }
        try_push_into_slice(&mut buf, c)?;
        idx += 1;
      }
      if idx < 3 {
        return Err(FormatError::BadMatch);
      }
      if idx > 3 && idx < INF_STR.len() {
        rollback(consumer, &mut buf, idx - 3)?;
      }
      break;
    } else if only_sign && (ctype.casemap.tolower)(ch.into()) == 'n'.into() {
      let avail = width.saturating_sub(k);
      if avail < 3 {
        return Err(FormatError::BadMatch);
      }
      let start = buf.len();
      try_push_into_slice(&mut buf, c_cur)?;
      let mut idx = 0usize;
      loop {
        if buf.len() - start >= avail {
          break;
        }
        let c = match consumer.consume_u32() {
          | Err(FormatError::EndOfFile) => break,
          | Err(e) => return Err(e),
          | Ok(c) => c
        };
        let lwr = (ctype.casemap.tolower)(c);
        match idx {
          | 0 => {
            if lwr != 'a'.into() {
              consumer.vomit_u32(c)?;
              break;
            }
          },
          | 1 => {
            if lwr != 'n'.into() {
              consumer.vomit_u32(c)?;
              break;
            }
          },
          | 2 => {
            if c != '('.into() {
              consumer.vomit_u32(c)?;
              break;
            }
          },
          | _ => {
            if c == ')'.into() {
              try_push_into_slice(&mut buf, c)?;
              break;
            } else if !(ctype.casemap.isalnum)(c) && c != '_'.into() {
              consumer.vomit_u32(c)?;
              break;
            }
          },
        }
        try_push_into_slice(&mut buf, c)?;
        idx += 1;
      }
      let got = buf.len() - start;
      if got < 3 {
        return Err(FormatError::BadMatch);
      }
      if got > 3 && buf.last() != Some(&u32::from(')')) {
        rollback(consumer, &mut buf, got - 3)?;
      }
      break;
    } else if ch == '0' {
      mayhex = only_sign && !ishex;
      seen_digit = true;
      try_push_into_slice(&mut buf, c_cur)?;
    } else if mayhex && ch.to_ascii_lowercase() == 'x' {
      seen_digit = false;
      ishex = true;
      try_push_into_slice(&mut buf, c_cur)?;
    } else if ishex &&
      !got_exp_mark &&
      seen_digit &&
      ch.to_ascii_lowercase() == 'p'
    {
      got_exp_mark = true;
      seen_digit = false;
      try_push_into_slice(&mut buf, c_cur)?;
    } else if !ishex &&
      !got_exp_mark &&
      seen_digit &&
      ch.to_ascii_lowercase() == 'e'
    {
      got_exp_mark = true;
      seen_digit = false;
      try_push_into_slice(&mut buf, c_cur)?;
    } else if ch == decimal_point && !after_decimal && !got_exp_mark {
      after_decimal = true;
      try_push_into_slice(&mut buf, c_cur)?;
    } else if !ch.is_digit(if got_exp_mark || !ishex { 10 } else { 16 }) {
      consumer.vomit_u32(c_cur)?;
      if seen_digit || ishex || got_exp_mark {
        break;
      }
      return Err(FormatError::BadMatch);
    } else {
      seen_digit = true;
      try_push_into_slice(&mut buf, c_cur)?;
    }

    k += 1;
  }

  if ishex && !got_exp_mark && !seen_digit {
    rollback(consumer, &mut buf, 1 + after_decimal as usize)?;
  } else if got_exp_mark && !seen_digit {
    if k >= width {
      return Err(FormatError::BadMatch);
    }
    let n = if buf.last().is_some_and(|&b| b == '+' as u32 || b == '-' as u32) {
      2
    } else {
      1
    };
    rollback(consumer, &mut buf, n)?;
  }

  write_floating_point_value(consumer, &buf, ptr, arg, ctype, numeric)
}
