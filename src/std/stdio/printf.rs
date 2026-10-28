use {
  crate::{
    std::{errno, stdlib, string},
    support::{
      ffi::va_list::ExtVaList,
      format::{
        error::FormatError,
        printf::{self, Emitter}
      },
      locale::{self, ctype::CtypeObject}
    },
    types::{c_char, c_int, size_t}
  },
  core::{ffi::VaList, slice}
};

#[derive(Debug)]
struct BufferedEmitter<'a> {
  pub buffer: &'a mut [u8],
  written: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> BufferedEmitter<'a> {
  #[inline]
  pub const fn new(
    buffer: *mut c_char,
    bufsz: usize,
    ctype: &'a CtypeObject<'a>
  ) -> Self {
    let buffer: &mut [u8] = if bufsz == 0 {
      &mut []
    } else {
      unsafe { slice::from_raw_parts_mut(buffer as *mut u8, bufsz) }
    };
    Self { buffer, written: 0, ctype }
  }

  #[inline]
  pub fn terminate(&mut self) {
    self.buffer[self.written] = b'\0';
  }

  #[inline]
  pub fn emit_u8(
    &mut self,
    ch: u8
  ) -> Result<(), FormatError> {
    if self.written + 1 < self.buffer.len() {
      self.buffer[self.written] = ch;
    }
    self.written += 1;
    Ok(())
  }

  #[inline]
  pub fn emit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    let mut buffer = [0u8; stdlib::constants::MB_LEN_MAX];
    let result = (self.ctype.converter.c32tomb)(&mut buffer, ch);
    if result < 0 {
      self.emit_u8(b'?')?;
      return Ok(());
    }
    for i in 0..(result as usize) {
      self.emit_u8(buffer[i])?;
    }
    Ok(())
  }
}

impl<'a> Emitter for BufferedEmitter<'a> {
  type FormatChar = u8;

  #[inline]
  fn get_written(&self) -> usize {
    self.written
  }

  #[inline]
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.emit_u8(c)
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u8(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u8(*i as u8)?;
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
  fn get_unicode_char_len(c: char) -> usize {
    c.len_utf8()
  }
}

#[derive(Debug)]
struct UnboundedEmitter<'a> {
  ptr: *mut u8,
  written: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> UnboundedEmitter<'a> {
  #[inline]
  pub const fn new(
    ptr: *mut c_char,
    ctype: &'a CtypeObject<'a>
  ) -> Self {
    Self { ptr: ptr as *mut u8, written: 0, ctype }
  }

  #[inline]
  pub unsafe fn terminate(&mut self) {
    unsafe { *self.ptr.offset(self.written as isize) = b'\0' };
  }

  #[inline]
  pub fn emit_u8(
    &mut self,
    ch: u8
  ) -> Result<(), FormatError> {
    if !self.ptr.is_null() {
      unsafe { *self.ptr.offset(self.written as isize) = ch };
    }
    self.written += 1;
    Ok(())
  }

  #[inline]
  pub fn emit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    let mut buffer = [0u8; stdlib::constants::MB_LEN_MAX];
    let result = (self.ctype.converter.c32tomb)(&mut buffer, ch);
    if result < 0 {
      self.emit_u8(b'?')?;
      return Ok(());
    }
    for i in 0..(result as usize) {
      self.emit_u8(buffer[i])?;
    }
    Ok(())
  }
}

impl<'a> Emitter for UnboundedEmitter<'a> {
  type FormatChar = u8;

  #[inline]
  fn get_written(&self) -> usize {
    self.written
  }

  #[inline]
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.emit_u8(c)
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u8(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u8(*i as u8)?;
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
  fn get_unicode_char_len(c: char) -> usize {
    c.len_utf8()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vsnprintf(
  buffer: *mut c_char,
  count: size_t,
  format: *const c_char,
  vlist: VaList
) -> c_int {
  let format = unsafe {
    slice::from_raw_parts(format as *const u8, string::rs_strlen(format))
  };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let mut emitter = BufferedEmitter::new(buffer, count, &ctype);
  let vlist = unsafe { ExtVaList::from_va_list(vlist) };

  let result = printf::printf_inner(&locale, &mut emitter, format, vlist);

  if count > 0 {
    let written = emitter.get_written();
    let terminate = if written < count - 1 { written } else { count - 1 };
    emitter.buffer[terminate] = b'\0';
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
pub extern "C" fn rs_vsprintf(
  buffer: *mut c_char,
  format: *const c_char,
  vlist: VaList
) -> c_int {
  let format = unsafe {
    slice::from_raw_parts(format as *const u8, string::rs_strlen(format))
  };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let mut emitter = UnboundedEmitter::new(buffer, &ctype);
  let vlist = unsafe { ExtVaList::from_va_list(vlist) };

  let result = printf::printf_inner(&locale, &mut emitter, format, vlist);

  unsafe { emitter.terminate() };

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
pub unsafe extern "C" fn rs_snprintf(
  buffer: *mut c_char,
  count: size_t,
  format: *const c_char,
  args: ...
) -> c_int {
  rs_vsnprintf(buffer, count, format, args.clone())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_sprintf(
  buffer: *mut c_char,
  format: *const c_char,
  args: ...
) -> c_int {
  rs_vsprintf(buffer, format, args.clone())
}
