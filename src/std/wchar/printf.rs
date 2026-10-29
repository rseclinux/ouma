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
  core::{ffi::VaList, slice, str}
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
  pub fn terminate(&mut self) {
    self.buffer[self.written] = '\0' as u32;
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
