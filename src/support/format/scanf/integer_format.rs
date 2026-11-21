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
      string::conversion::{StrToError, strtoint}
    },
    types::{intmax_t, uintmax_t}
  },
  smallvec::SmallVec
};

const INT_STR_ARRAY_SIZE: usize = 512;

#[inline]
fn try_push_into_slice(
  buf: &mut SmallVec<[u32; INT_STR_ARRAY_SIZE]>,
  value: u32
) -> Result<(), FormatError> {
  if buf.try_reserve_exact(1).is_err() {
    return Err(FormatError::InvalidArg);
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
  if ptr.is_null() && !arg.suppress {
    return Err(FormatError::InvalidArg);
  }

  let width = if arg.width == 0 || arg.width > INT_STR_ARRAY_SIZE {
    usize::MAX
  } else {
    arg.width
  };
  let spec = char::from_u32((ctype.casemap.tolower)(arg.specifier.into()))
    .unwrap_or('\0');

  let mut buf: SmallVec<[u32; INT_STR_ARRAY_SIZE]> = SmallVec::new();

  loop {
    let c = consumer.consume_u32()?;
    if !(ctype.casemap.isspace)(c) {
      consumer.vomit_u32(c)?;
      break;
    }
  }

  let (mut base, is_signed): (u32, bool) = match spec {
    | 'i' => (0, true),
    | 'd' => (10, true),
    | 'b' => (2, false),
    | 'o' => (8, false),
    | 'x' | 'p' => (16, false),
    | _ => (10, false)
  };

  let mut k = 0usize;
  let mut got_sign = false;
  let mut got_number = false;
  let mut pfx: Option<char> = None;

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

    if (ch == '-' || ch == '+') && !got_sign && k == 0 {
      got_sign = true;
      try_push_into_slice(&mut buf, c_cur)?;
      k += 1;
      continue;
    }

    if base == 0 || base == 2 || base == 16 {
      if ch == '0' {
        got_number = true;

        if ch.to_ascii_lowercase() == 'b' {
          got_number = false;
          base = 2;
        } else if ch.to_ascii_lowercase() == 'x' {
          got_number = false;
          base = 16;
        } else {
          if base == 0 {
            base = 8;
          }
        }
      } else if base == 0 {
        if ch.is_ascii_digit() {
          base = 10;
        } else {
          consumer.vomit_u32(c_cur)?;
          if is_signed {
            write_signed_integer(0, ptr, arg)?;
          } else {
            write_unsigned_integer(0, ptr, arg)?;
          }
          return Err(FormatError::BadMatch);
        }
      }
    }

    let pfx_base = match ch.to_ascii_lowercase() {
      | 'b' if matches!(spec, 'i' | 'b') => 2,
      | 'x' if matches!(spec, 'i' | 'x' | 'p') => 16,
      | _ => 0
    };
    let may_have_pfx = pfx_base != 0 &&
      pfx.is_none() &&
      buf.len() == usize::from(got_sign) + 1 &&
      buf.last() == Some(&('0' as u32));

    if ch.is_digit(base) {
      try_push_into_slice(&mut buf, c_cur)?;
      got_number = true;
    } else if may_have_pfx {
      try_push_into_slice(&mut buf, c_cur)?;
      got_number = false;
      pfx = Some(ch);
      base = pfx_base;
    } else {
      consumer.vomit_u32(c_cur)?;
      break;
    }

    k += 1;
  }

  if !got_number && pfx.is_some() {
    if let Some(r) = buf.pop() {
      consumer.vomit_u32(r)?;
      k -= 1;
    }
    got_number =
      buf.last().map_or(false, |&c| c != '+' as u32 && c != '-' as u32);
  }

  if !got_number {
    for _ in (0..k).rev() {
      if let Some(r) = buf.pop() {
        consumer.vomit_u32(r)?;
      }
    }
  }

  let store_ptr = arg.specifier == 'p';
  if is_signed {
    let result: strtoint::StrToIntResult<intmax_t> =
      strtoint::strtoint(&buf, base as i32, ctype);
    if result.error == Some(StrToError::InvalidNumber) {
      return Err(FormatError::BadMatch);
    } else if !arg.suppress {
      consumer.increase_converted();
      if store_ptr {
        let outptr: *const u8 = result.value as isize as *const u8;
        unsafe { *(ptr as *mut *mut u8) = outptr.cast_mut() };
        return Ok(());
      } else {
        return write_signed_integer(result.value, ptr, arg);
      }
    } else {
      Ok(())
    }
  } else {
    let result: strtoint::StrToIntResult<uintmax_t> =
      strtoint::strtoint(&buf, base as i32, ctype);
    if result.error == Some(StrToError::InvalidNumber) {
      return Err(FormatError::BadMatch);
    } else if !arg.suppress {
      consumer.increase_converted();
      if store_ptr {
        let outptr: *const u8 = result.value as usize as *const u8;
        unsafe { *(ptr as *mut *mut u8) = outptr.cast_mut() };
        return Ok(());
      } else {
        return write_unsigned_integer(result.value, ptr, arg);
      }
    } else {
      Ok(())
    }
  }
}
