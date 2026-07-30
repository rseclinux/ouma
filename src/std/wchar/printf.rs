use {
  crate::{
    c_int,
    size_t,
    std::{stdlib::constants, wchar},
    stdio::{
      format::FormatError,
      printf::{Emitter, printf_inner}
    },
    support::{ffi::va_list::ExtVaList, locale},
    wchar_t
  },
  core::{ffi::VaList, slice, str}
};

struct BufferWriter<'a> {
  buffer: &'a mut [u32],
  pos: usize,
  ctype: locale::ctype::CtypeObject<'a>
}

impl<'a> BufferWriter<'a> {
  #[inline]
  pub fn new(
    buffer: *mut u32,
    bufsz: usize,
    ctype: locale::ctype::CtypeObject<'a>
  ) -> Self {
    let slice = unsafe { slice::from_raw_parts_mut(buffer, bufsz) };
    Self { buffer: slice, pos: 0, ctype: ctype }
  }

  #[inline]
  fn is_valid_char(
    &self,
    v: u32
  ) -> bool {
    let mut buffer = [0u8; constants::MB_LEN_MAX];
    (self.ctype.converter.c32tomb)(&mut buffer, v) != -1
  }
}

impl<'a> Emitter for BufferWriter<'a> {
  type FormatChar = u32;

  #[inline]
  fn get_written(&self) -> usize {
    self.pos
  }

  #[inline]
  fn get_unicode_char_len(_: char) -> usize {
    1
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    if self.pos + s.len() > self.buffer.len() {
      return Err(FormatError::Overflow);
    }
    let s = str::from_utf8(s).map_err(|_| FormatError::InvalidSequence)?;
    for c in s.chars() {
      if self.is_valid_char(c as u32) {
        self.buffer[self.pos] = c as u32;
        self.pos += 1;
      } else {
        return Err(FormatError::InvalidSequence);
      }
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
      self.buffer[self.pos] = *c as u32;
      self.pos += 1;
    }
    Ok(())
  }

  #[inline]
  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError> {
    self.emit_u32_slice(s)
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
      self.buffer[self.pos] = *c;
      self.pos += 1;
    }
    Ok(())
  }

  #[inline]
  fn emit_unicode_char(
    &mut self,
    c: char
  ) -> Result<(), FormatError> {
    self.emit_u32_slice(&[c as u32])
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vswprintf(
  buffer: *mut wchar_t,
  bufsz: size_t,
  format: *const wchar_t,
  list: VaList
) -> c_int {
  let fmtlen = wchar::rs_wcslen(format);
  let fmt = unsafe { slice::from_raw_parts(format as *const u32, fmtlen) };
  let locale = locale::get_thread_locale();

  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default().clone();

  let mut emitter = BufferWriter::new(buffer.cast(), bufsz, ctype);
  let mut ap = unsafe { ExtVaList::from_va_list(list) };

  let ret = printf_inner(locale, &mut emitter, fmt, &mut ap);

  if bufsz > 0 {
    emitter.buffer[emitter.pos] = '\0' as u32;
  }

  match ret {
    | Ok(r) => r as c_int,
    | Err(_) => -1 // TODO: set errno
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_swprintf(
  buffer: *mut wchar_t,
  bufsz: size_t,
  format: *const wchar_t,
  list: ...
) -> c_int {
  rs_vswprintf(buffer, bufsz, format, list)
}
