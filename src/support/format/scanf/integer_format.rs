use {
  super::{
    Consumer,
    ScanfArgument,
    utils::{write_signed_integer, write_unsigned_integer}
  },
  crate::{
    support::{
      format::error::FormatError,
      locale::ctype::CtypeObject,
      string::conversion::{StrToError, strtoint},
      traits::char::get_ascii_char
    },
    types::{intmax_t, uintmax_t}
  },
  smallvec::SmallVec
};

const INT_STR_ARRAY_SIZE: usize = 512;

#[inline]
fn try_push_into_slice<C: Consumer>(
  buf: &mut SmallVec<[C::FormatChar; INT_STR_ARRAY_SIZE]>,
  value: C::FormatChar
) -> Result<(), FormatError> {
  if buf.try_reserve_exact(1).is_err() {
    return Err(FormatError::Allocation);
  }
  buf.push(value);
  Ok(())
}

#[inline]
pub fn format_integer<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject
) -> Result<(), FormatError> {
  if arg.allocate {
    return Err(FormatError::BadMatch);
  }

  let width = if arg.width == 0 || arg.width > INT_STR_ARRAY_SIZE {
    INT_STR_ARRAY_SIZE
  } else {
    arg.width
  };
  let spec = char::from_u32((ctype.casemap.tolower)(arg.specifier.into()))
    .unwrap_or('\0');

  let mut buf: SmallVec<[C::FormatChar; INT_STR_ARRAY_SIZE]> = SmallVec::new();
  let (is_signed, mut base): (bool, u32) = match spec {
    | 'i' => (true, 0),
    | 'd' => (true, 10),
    | 'b' => (false, 2),
    | 'o' => (false, 8),
    | 'x' | 'p' => (false, 16),
    | _ => (false, 10)
  };

  let bin_fmt = spec == 'i' || spec == 'b';
  let hex_fmt = spec == 'i' || spec == 'x';

  loop {
    let c = consumer.consume_unicode_char()?;
    if !(ctype.casemap.isspace)(c.into()) {
      break;
    }
  }

  let mut pfx_len = 0usize;
  let mut seen_digit = false;
  let mut k = 0usize;
  while k < width {
    let c_cur = match consumer.consume() {
      | Err(FormatError::EndOfFile) => break,
      | Err(e) => return Err(e),
      | Ok(c) => c
    };

    let ch = get_ascii_char(c_cur).to_char();
    println!("consumed: {ch:?}");

    if (ctype.casemap.isspace)(ch.into()) {
      consumer.vomit(c_cur)?;
      break;
    }

    if ch == '-' || ch == '+' {
      if k != 0 {
        consumer.vomit(c_cur)?;
        break;
      }
    } else if ch == '0' {
      if base == 0 {
        pfx_len = 1;
        base = 8;
      } else if (hex_fmt || bin_fmt) && pfx_len == 0 {
        pfx_len = 1;
      } else {
        pfx_len = 0;
      }
      seen_digit = true;
    } else if hex_fmt && pfx_len == 1 && ch.to_ascii_lowercase() == 'x' {
      base = 16;
      pfx_len = 2;
      seen_digit = false;
    } else if bin_fmt && pfx_len == 1 && ch.to_ascii_lowercase() == 'b' {
      base = 2;
      pfx_len = 2;
      seen_digit = false;
    } else if base == 0 {
      if ch.is_ascii_digit() {
        base = 10;
        seen_digit = true;
      } else {
        consumer.vomit(c_cur)?;
        return Err(FormatError::BadMatch);
      }
    } else if !ch.is_digit(base) {
      consumer.vomit(c_cur)?;
      if seen_digit {
        break;
      }
      return Err(FormatError::BadMatch);
    } else {
      seen_digit = true;
      pfx_len = 0;
    }

    println!(
      "before push: ch={ch:?}, k={k}, seen_digit={seen_digit}, base={base}"
    );

    try_push_into_slice::<C>(&mut buf, c_cur)?;

    println!("after push: len={}", buf.len());

    k += 1;
  }

  if is_signed {
    let s: String =
      buf.iter().copied().map(|c| get_ascii_char(c).to_char()).collect();
    eprintln!("integer string: \"{s}\"");
    eprintln!(
      "integer: spec={spec:?}, base={base}, seen_digit={seen_digit}, len={}",
      buf.len()
    );

    let result: strtoint::StrToIntResult<intmax_t> =
      strtoint::strtoint(&buf, base as i32, ctype);

    eprintln!("strtoint: error={:?}, value={:?}", result.error, result.value);
    if result.error == Some(StrToError::InvalidNumber) {
      return Err(FormatError::BadMatch);
    } else if !arg.suppress {
      consumer.increase_converted();
      return write_signed_integer(result.value, ptr, arg);
    } else {
      Ok(())
    }
  } else {
    let s: String =
      buf.iter().copied().map(|c| get_ascii_char(c).to_char()).collect();
    eprintln!("integer string: \"{s}\"");
    eprintln!(
      "integer: spec={spec:?}, base={base}, seen_digit={seen_digit}, len={}",
      buf.len()
    );

    let result: strtoint::StrToIntResult<uintmax_t> =
      strtoint::strtoint(&buf, base as i32, ctype);

    eprintln!("strtoint: error={:?}, value={:?}", result.error, result.value);

    if result.error == Some(StrToError::InvalidNumber) {
      return Err(FormatError::BadMatch);
    } else if !arg.suppress {
      consumer.increase_converted();
      return write_unsigned_integer(result.value, ptr, arg);
    } else {
      Ok(())
    }
  }
}
