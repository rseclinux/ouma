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
  if ptr.is_null() {
    return Err(FormatError::InvalidArg);
  }

  let width = if arg.width == 0 || arg.width > FLOAT_STR_ARRAY_SIZE {
    usize::MAX
  } else {
    arg.width
  };
  let _spec = char::from_u32((ctype.casemap.tolower)(arg.specifier.into()))
    .unwrap_or('\0');
  let _decimal_point = numeric.get_decimal_point().unwrap_or('\0');

  let mut buf: SmallVec<[u32; FLOAT_STR_ARRAY_SIZE]> = SmallVec::new();

  loop {
    let c = consumer.consume_u32()?;
    if !(ctype.casemap.isspace)(c) {
      consumer.vomit_u32(c)?;
      break;
    }
  }

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

    if ch == '-' || ch == '+' {
      if k != 0 {
        consumer.vomit_u32(c_cur)?;
        break;
      }
    } else if only_sign && (ctype.casemap.tolower)(ch.into()) == 'i'.into() {
      try_push_into_slice(&mut buf, c_cur)?;
      let mut idx = 1usize;
      while idx < INF_STR.len() {
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
      let start = buf.len();
      try_push_into_slice(&mut buf, c_cur)?;
      let mut idx = 0usize;
      loop {
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
    } else {
      eprintln!("Gotta skip: \"{ch}\"");
    }

    try_push_into_slice(&mut buf, c_cur)?;

    k += 1;
  }

  let s: String =
    buf.iter().copied().filter_map(|c| char::from_u32(c)).collect();
  eprintln!("String! \"{s}\"");

  write_floating_point_value(consumer, &buf, ptr, arg, ctype, numeric)
}
