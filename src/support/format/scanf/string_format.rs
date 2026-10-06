use {
  super::{Consumer, ScanfArgument},
  crate::{
    std::stdlib::alloc,
    support::{
      format::{error::FormatError, length::LengthModifier},
      locale::ctype::CtypeObject
    },
    wchar_t
  },
  core::{mem, slice}
};

#[inline]
fn format_narrow_string<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject
) -> Result<(), FormatError> {
  let width = if arg.width > 0 { arg.width } else { usize::MAX };
  let need_append = (arg.allocate || !arg.suppress) && !ptr.is_null();

  let mut outptr: *mut u8;
  let mut outlen;

  if arg.allocate {
    let alloc_len = if arg.width == 0 {
      16 * mem::size_of::<u8>()
    } else {
      arg
        .width
        .checked_add(1)
        .and_then(|c| c.checked_mul(mem::size_of::<u8>()))
        .ok_or(FormatError::Overflow)?
    };

    let p = alloc::rs_malloc(alloc_len);
    if p.is_null() {
      return Err(FormatError::Allocation);
    }

    outptr = p.cast();
    outlen = alloc_len / mem::size_of::<u8>();

    if need_append {
      unsafe {
        *(ptr as *mut *mut u8) = outptr;
      }
    }
  } else {
    outptr = ptr;
    outlen = width;
  }

  if arg.scan_set.is_none() {
    loop {
      let c = consumer.consume_u8()?;
      if !(ctype.casemap.isspace)(c.into()) {
        consumer.vomit_u8(c)?;
        break;
      }
    }
  }

  let mut k = 0usize;
  while k < width {
    let c = match consumer.consume_u8() {
      | Ok(c) => c,
      | Err(FormatError::EndOfFile) if k > 0 => break,
      | Err(e) => {
        if arg.allocate {
          alloc::rs_free(outptr.cast());
        }
        return Err(e);
      }
    };

    let skip = if let Some(set) = arg.scan_set {
      !set.contains(c.into())
    } else {
      (ctype.casemap.isspace)(c.into())
    };

    if skip {
      let r = consumer.vomit_u8(c);
      if r.is_err() && arg.allocate {
        alloc::rs_free(outptr.cast());
      }
      r?;
      break;
    }

    if need_append {
      if arg.allocate && k + 1 >= outlen {
        let len = mem::size_of::<u8>();
        let newlen = match outlen.checked_mul(2) {
          | Some(n) => n,
          | None => {
            alloc::rs_free(outptr.cast());
            return Err(FormatError::Overflow);
          }
        };
        let new = alloc::rs_reallocarray(outptr.cast(), newlen, len);
        if new.is_null() {
          alloc::rs_free(outptr.cast());
          return Err(FormatError::Allocation);
        }
        outptr = new.cast();
        outlen = newlen;
        unsafe {
          *(ptr as *mut *mut u8) = outptr;
        }
      }
      let out = unsafe { slice::from_raw_parts_mut(outptr, k + 1) };
      out[k] = c;
    }

    k += 1;
  }

  if k == 0 {
    if arg.allocate {
      alloc::rs_free(outptr.cast());
    }
    return Err(FormatError::BadMatch);
  }

  if need_append {
    let out = unsafe { slice::from_raw_parts_mut(outptr, k + 1) };
    out[k] = b'\0';
    if arg.allocate {
      unsafe {
        *(ptr as *mut *mut u8) = outptr;
      }
    }
    consumer.increase_converted();
  }

  if arg.allocate && !need_append {
    alloc::rs_free(outptr.cast());
  }

  Ok(())
}

#[inline]
fn format_wide_string<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject
) -> Result<(), FormatError> {
  let width = if arg.width > 0 { arg.width } else { usize::MAX };
  let need_append = (arg.allocate || !arg.suppress) && !ptr.is_null();

  let mut outptr: *mut u8;
  let mut outlen;

  if arg.allocate {
    let alloc_len = if arg.width == 0 {
      16 * mem::size_of::<wchar_t>()
    } else {
      arg
        .width
        .checked_add(1)
        .and_then(|c| c.checked_mul(mem::size_of::<wchar_t>()))
        .ok_or(FormatError::Overflow)?
    };

    let p = alloc::rs_malloc(alloc_len);
    if p.is_null() {
      return Err(FormatError::Allocation);
    }

    outptr = p.cast();
    outlen = alloc_len / mem::size_of::<wchar_t>();

    if need_append {
      unsafe {
        *(ptr as *mut *mut u8) = outptr;
      }
    }
  } else {
    outptr = ptr;
    outlen = width;
  }

  if arg.scan_set.is_none() {
    loop {
      let c = consumer.consume_u32()?;
      if !(ctype.casemap.isspace)(c) {
        consumer.vomit_u32(c)?;
        break;
      }
    }
  }

  let mut k = 0usize;
  while k < width {
    let c = match consumer.consume_u32() {
      | Ok(c) => c,
      | Err(FormatError::EndOfFile) if k > 0 => break,
      | Err(e) => {
        if arg.allocate {
          alloc::rs_free(outptr.cast());
        }
        return Err(e);
      }
    };

    let skip = if let Some(set) = arg.scan_set {
      !set.contains(c as usize)
    } else {
      (ctype.casemap.isspace)(c)
    };

    if skip {
      let r = consumer.vomit_u32(c);
      if r.is_err() && arg.allocate {
        alloc::rs_free(outptr.cast());
      }
      r?;
      break;
    }

    if need_append {
      if arg.allocate && k + 1 >= outlen {
        let len = mem::size_of::<wchar_t>();
        let newlen = match outlen.checked_mul(2) {
          | Some(n) => n,
          | None => {
            alloc::rs_free(outptr.cast());
            return Err(FormatError::Overflow);
          }
        };
        let new = alloc::rs_reallocarray(outptr.cast(), newlen, len);
        if new.is_null() {
          alloc::rs_free(outptr.cast());
          return Err(FormatError::Allocation);
        }
        outptr = new.cast();
        outlen = newlen;
        unsafe {
          *(ptr as *mut *mut u8) = outptr;
        }
      }
      let out =
        unsafe { slice::from_raw_parts_mut(outptr.cast::<u32>(), k + 1) };
      out[k] = c;
    }

    k += 1;
  }

  if k == 0 {
    if arg.allocate {
      alloc::rs_free(outptr.cast());
    }
    return Err(FormatError::BadMatch);
  }

  if need_append {
    let out = unsafe { slice::from_raw_parts_mut(outptr.cast::<u32>(), k + 1) };
    out[k] = '\0'.into();
    if arg.allocate {
      unsafe {
        *(ptr as *mut *mut u8) = outptr;
      }
    }
    consumer.increase_converted();
  }

  if arg.allocate && !need_append {
    alloc::rs_free(outptr.cast());
  }

  Ok(())
}

#[inline]
pub fn format_string<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject
) -> Result<(), FormatError> {
  if arg.modifier == LengthModifier::Long {
    format_wide_string(consumer, ptr, arg, ctype)
  } else {
    format_narrow_string(consumer, ptr, arg, ctype)
  }
}
