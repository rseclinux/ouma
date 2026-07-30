use {
  crate::{
    c_char,
    c_int,
    size_t,
    std::string,
    stdio::{
      format::FormatError,
      printf::{Emitter, printf_inner}
    },
    support::{ffi::va_list::ExtVaList, locale}
  },
  core::{ffi::VaList, slice}
};

struct BufferWriter<'a> {
  buffer: &'a mut [u8],
  pos: usize,
  ctype: locale::ctype::CtypeObject<'a>
}

impl<'a> BufferWriter<'a> {
  #[inline]
  pub fn new(
    buffer: *mut u8,
    bufsz: usize,
    ctype: locale::ctype::CtypeObject<'a>
  ) -> Self {
    let slice = unsafe { slice::from_raw_parts_mut(buffer, bufsz) };
    Self { buffer: slice, pos: 0, ctype: ctype }
  }

  #[inline]
  fn conv_u32(
    &mut self,
    v: u32
  ) -> Result<(), FormatError> {
    let ret = (self.ctype.converter.c32tomb)(&mut self.buffer[self.pos..], v);
    if ret == -1 {
      return Err(FormatError::InvalidSequence);
    }
    self.pos += ret as usize;
    Ok(())
  }
}

impl<'a> Emitter for BufferWriter<'a> {
  type FormatChar = u8;

  #[inline]
  fn get_written(&self) -> usize {
    self.pos
  }

  #[inline]
  fn get_unicode_char_len(c: char) -> usize {
    c.len_utf8()
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    if self.pos + s.len() > self.buffer.len() {
      return Err(FormatError::Overflow);
    }
    for c in s {
      self.buffer[self.pos] = *c;
      self.pos += 1;
    }
    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    if self.pos + s.len() > self.buffer.len() {
      return Err(FormatError::Overflow);
    }
    for c in s {
      self.buffer[self.pos] = *c as u8;
      self.pos += 1;
    }
    Ok(())
  }

  #[inline]
  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError> {
    self.emit_u8_slice(s)
  }

  #[inline]
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError> {
    if self.pos + s.len() > self.buffer.len() {
      return Err(FormatError::Overflow);
    }
    for c in s {
      self.conv_u32(*c)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_unicode_char(
    &mut self,
    c: char
  ) -> Result<(), FormatError> {
    let narrow = c as u8;
    if c.is_ascii() {
      self.emit_u8_slice(&[narrow])
    } else {
      self.emit_u32_slice(&[c as u32])
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vsnprintf(
  buffer: *mut c_char,
  bufsz: size_t,
  format: *const c_char,
  list: VaList
) -> c_int {
  let fmtlen = string::rs_strlen(format);
  let fmt = unsafe { slice::from_raw_parts(format as *const u8, fmtlen) };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default().clone();

  let mut emitter = BufferWriter::new(buffer.cast(), bufsz, ctype);
  let mut ap = unsafe { ExtVaList::from_va_list(list) };

  let ret = printf_inner(locale, &mut emitter, fmt, &mut ap);

  if bufsz > 0 {
    emitter.buffer[emitter.pos] = b'\0';
  }

  match ret {
    | Ok(r) => r as c_int,
    | Err(_) => -1 // TODO: set errno
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vsprintf(
  buffer: *mut c_char,
  format: *const c_char,
  list: VaList
) -> c_int {
  let fmtlen = string::rs_strlen(format);
  let fmt = unsafe { slice::from_raw_parts(format as *const u8, fmtlen) };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default().clone();

  let mut emitter = BufferWriter::new(buffer.cast(), usize::MAX, ctype);
  let mut ap = unsafe { ExtVaList::from_va_list(list) };

  let ret = printf_inner(locale, &mut emitter, fmt, &mut ap);

  emitter.buffer[emitter.pos] = b'\0';

  match ret {
    | Ok(r) => r as c_int,
    | Err(_) => -1 // TODO: set errno
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_snprintf(
  buffer: *mut c_char,
  bufsz: size_t,
  format: *const c_char,
  list: ...
) -> c_int {
  rs_vsnprintf(buffer, bufsz, format, list)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_sprintf(
  buffer: *mut c_char,
  format: *const c_char,
  list: ...
) -> c_int {
  rs_vsprintf(buffer, format, list)
}
