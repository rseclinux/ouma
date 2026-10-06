use {
  super::{Consumer, ScanfArgument},
  crate::{
    std::stdlib::alloc,
    support::format::{error::FormatError, length::LengthModifier}
  },
  core::{mem, slice}
};

#[inline]
fn format_narrow_char<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  let width = if arg.width > 0 { arg.width } else { 1 };
  let need_append = (arg.allocate || !arg.suppress) && !ptr.is_null();

  let out: *mut u8;

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

    out = p.cast();

    if need_append {
      unsafe {
        *(ptr as *mut *mut u8) = out;
      }
    }
  } else {
    out = ptr;
  }

  let oslice = if need_append {
    unsafe { slice::from_raw_parts_mut(out, width) }
  } else {
    &mut [][..]
  };

  let mut k = 0usize;
  while k < width {
    let c = match consumer.consume_u8() {
      | Ok(c) => c,
      | Err(e) => {
        if arg.allocate {
          alloc::rs_free(out.cast());
        }
        return Err(e);
      }
    };

    if need_append {
      oslice[k] = c;
    }

    k += 1;
  }

  if need_append {
    consumer.increase_converted();
  }

  Ok(())
}

#[inline]
fn format_wide_char<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  let width = if arg.width > 0 { arg.width } else { 1 };
  let need_append = (arg.allocate || !arg.suppress) && !ptr.is_null();

  let out: *mut u8;

  if arg.allocate {
    let alloc_len = if arg.width == 0 {
      16 * mem::size_of::<u32>()
    } else {
      arg
        .width
        .checked_add(1)
        .and_then(|c| c.checked_mul(mem::size_of::<u32>()))
        .ok_or(FormatError::Overflow)?
    };

    let p = alloc::rs_malloc(alloc_len);
    if p.is_null() {
      return Err(FormatError::Allocation);
    }

    out = p.cast();

    if need_append {
      unsafe {
        *(ptr as *mut *mut u8) = out;
      }
    }
  } else {
    out = ptr;
  }

  let oslice = if need_append {
    unsafe { slice::from_raw_parts_mut(out.cast::<u32>(), width) }
  } else {
    &mut [][..]
  };

  let mut k = 0usize;
  while k < width {
    let c = match consumer.consume_u32() {
      | Ok(c) => c,
      | Err(e) => {
        if arg.allocate {
          alloc::rs_free(out.cast());
        }
        return Err(e);
      }
    };

    if need_append {
      oslice[k] = c;
    }

    k += 1;
  }

  if need_append {
    consumer.increase_converted();
  }

  Ok(())
}

#[inline]
pub fn format_char<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  if arg.modifier == LengthModifier::Long {
    format_wide_char(consumer, ptr, arg)
  } else {
    format_narrow_char(consumer, ptr, arg)
  }
}
