use {
  crate::{
    std::{errno, stdlib},
    support::{
      ffi::va_list::ExtVaList,
      format::{
        error::FormatError,
        printf::{self, Emitter}
      },
      locale::{self, ctype::CtypeObject}
    },
    types::{c_int, size_t, wchar_t}
  },
  core::{ffi::VaList, mem, slice, str}
};

#[derive(Debug)]
struct BufferedEmitter<'a> {
  pub buffer: &'a mut [u32],
  written: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> BufferedEmitter<'a> {
  #[inline]
  pub const fn new(
    buffer: *mut wchar_t,
    bufsz: usize,
    ctype: &'a CtypeObject<'a>
  ) -> Self {
    let buffer: &mut [u32] = if bufsz == 0 {
      &mut []
    } else {
      unsafe { slice::from_raw_parts_mut(buffer as *mut u32, bufsz) }
    };
    Self { buffer, written: 0, ctype }
  }

  #[inline]
  pub fn emit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    if self.written + 1 < self.buffer.len() {
      self.buffer[self.written] = ch;
    }
    self.written += 1;
    Ok(())
  }
}

impl<'a> Emitter for BufferedEmitter<'a> {
  type FormatChar = u32;

  #[inline]
  fn get_written(&self) -> usize {
    self.written
  }

  #[inline]
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.emit_u32(c)
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    let Ok(encoded) = str::from_utf8(s) else {
      return Err(FormatError::InvalidSequence);
    };

    for c in encoded.chars() {
      let mut buffer = [0u8; stdlib::constants::MB_LEN_MAX];
      let ret = (self.ctype.converter.c32tomb)(&mut buffer, c as u32);
      if ret >= 0 {
        self.emit_u32(c as u32)?;
      } else {
        self.emit_u32(char::REPLACEMENT_CHARACTER as u32)?;
      }
    }

    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u32(*i as u32)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u32(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn get_unicode_char_len(_: char) -> usize {
    1
  }
}

#[derive(Debug)]
struct AllocatedEmitter<'a> {
  pub buffer: &'a mut [u32],
  written: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> AllocatedEmitter<'a> {
  #[inline]
  pub fn new(ctype: &'a CtypeObject<'a>) -> Option<Self> {
    let size = 16 / mem::size_of::<wchar_t>();
    let p = stdlib::alloc::rs_malloc(size * mem::size_of::<wchar_t>());
    if p.is_null() {
      return None;
    }
    let buffer = unsafe { slice::from_raw_parts_mut(p as *mut u32, size) };
    Some(Self { buffer, written: 0, ctype })
  }

  #[inline]
  pub fn terminate(&mut self) {
    self.buffer[self.written] = 0;
  }

  #[inline]
  pub fn get_buffer(&mut self) -> *mut wchar_t {
    self.buffer.as_mut_ptr().cast()
  }

  #[inline]
  pub fn emit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    if self.written + 1 >= self.buffer.len() {
      let new_len =
        self.buffer.len().checked_mul(2).ok_or(FormatError::Allocation)?;
      let new_bytes = new_len
        .checked_mul(mem::size_of::<wchar_t>())
        .ok_or(FormatError::Allocation)?;
      let p =
        stdlib::alloc::rs_realloc(self.buffer.as_mut_ptr().cast(), new_bytes);
      if p.is_null() {
        return Err(FormatError::Allocation);
      }
      self.buffer =
        unsafe { slice::from_raw_parts_mut(p as *mut u32, new_len) };
    }
    self.buffer[self.written] = ch;
    self.written += 1;
    Ok(())
  }
}

impl<'a> Emitter for AllocatedEmitter<'a> {
  type FormatChar = u32;

  #[inline]
  fn get_written(&self) -> usize {
    self.written
  }

  #[inline]
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.emit_u32(c)
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    let Ok(encoded) = str::from_utf8(s) else {
      return Err(FormatError::InvalidSequence);
    };

    for c in encoded.chars() {
      let mut buffer = [0u8; stdlib::constants::MB_LEN_MAX];
      let ret = (self.ctype.converter.c32tomb)(&mut buffer, c as u32);
      if ret < 0 {
        return Err(FormatError::InvalidSequence);
      }
      self.emit_u32(c as u32)?;
    }

    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u32(*i as u32)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u32(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn get_unicode_char_len(_: char) -> usize {
    1
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vswprintf(
  buffer: *mut wchar_t,
  count: size_t,
  format: *const wchar_t,
  vlist: VaList
) -> c_int {
  let format = unsafe {
    slice::from_raw_parts(format as *const u32, super::rs_wcslen(format))
  };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let mut emitter = BufferedEmitter::new(buffer, count, &ctype);
  let vlist = unsafe { ExtVaList::from_va_list(vlist) };

  let result = printf::printf_inner(&locale, &mut emitter, format, vlist);

  if count > 0 {
    let written = emitter.get_written();
    let terminate = if written < count - 1 { written } else { count - 1 };
    emitter.buffer[terminate] = '\0' as u32;
  }

  match result {
    | Err(x) => {
      if matches!(x, FormatError::Overflow | FormatError::InvalidSequence) {
        errno::set_errno(x.to_errno());
      }
      -1
    },
    | Ok(x) => x as c_int
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_swprintf(
  buffer: *mut wchar_t,
  count: size_t,
  format: *const wchar_t,
  args: ...
) -> c_int {
  rs_vswprintf(buffer, count, format, args.clone())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vaswprintf(
  strp: *mut *mut wchar_t,
  format: *const wchar_t,
  vlist: VaList
) -> c_int {
  let format = unsafe {
    slice::from_raw_parts(format as *const u32, super::rs_wcslen(format))
  };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let emitter = AllocatedEmitter::new(&ctype);
  let mut emitter = match emitter {
    | Some(e) => e,
    | None => {
      errno::set_errno(errno::ENOMEM);
      return -1;
    }
  };

  let vlist = unsafe { ExtVaList::from_va_list(vlist) };

  let result = printf::printf_inner(&locale, &mut emitter, format, vlist);

  match result {
    | Err(x) => {
      if matches!(x, FormatError::Overflow | FormatError::InvalidSequence) {
        errno::set_errno(x.to_errno());
      }
      let p = emitter.get_buffer();
      stdlib::alloc::rs_free(p.cast());
      -1
    },
    | Ok(x) => {
      emitter.terminate();
      unsafe { *strp = emitter.get_buffer() };
      x as c_int
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_aswprintf(
  strp: *mut *mut wchar_t,
  format: *const wchar_t,
  args: ...
) -> c_int {
  rs_vaswprintf(strp, format, args.clone())
}
