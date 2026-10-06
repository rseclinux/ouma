use {
  super::constants::EOF,
  crate::{
    std::{errno, stdlib::constants, string},
    support::{
      format::{
        error::FormatError,
        scanf::{self, Consumer}
      },
      locale::{self, ctype::CtypeObject}
    },
    types::{MBState, c_char, c_int, char32_t}
  },
  core::{ffi::VaList, slice}
};

struct StreamConsumer<'a> {
  buffer: &'a [u8],
  consumed: usize,
  converted: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> StreamConsumer<'a> {
  #[inline]
  pub fn new(
    ptr: *const c_char,
    ctype: &'a CtypeObject
  ) -> Self {
    let len = string::rs_strlen(ptr);
    let buffer = unsafe { slice::from_raw_parts(ptr as *const u8, len) };
    Self { buffer, consumed: 0, converted: 0, ctype }
  }
}

impl<'a> Consumer for StreamConsumer<'a> {
  type FormatChar = u8;

  #[inline]
  fn consume_u8(&mut self) -> Result<u8, FormatError> {
    if self.consumed < self.buffer.len() {
      let ch = self.buffer[self.consumed];
      self.consumed += 1;
      return Ok(ch);
    }
    Err(FormatError::EndOfFile)
  }

  #[inline]
  fn vomit_u8(
    &mut self,
    _: u8
  ) -> Result<(), FormatError> {
    debug_assert!(self.consumed > 0);
    debug_assert!(self.consumed <= self.buffer.len());
    self.consumed -= 1;
    Ok(())
  }

  #[inline]
  fn consume_u32(&mut self) -> Result<u32, FormatError> {
    if self.consumed < self.buffer.len() {
      let s = &self.buffer[self.consumed..];

      if (s[0] & 0x80) == 0 {
        let ch = s[0] as u32;
        self.consumed += 1;
        return Ok(ch);
      }

      let mut bytes = 1usize;

      let offset = 0usize;
      while let Some(ch) = s.get(offset) &&
        offset < 4
      {
        if (ch & 0xe0) == 0xc0 {
          bytes += 1;
          break;
        } else if (ch & 0xf0) == 0xe0 {
          bytes += 2;
          break;
        } else if (ch & 0xf8) == 0xf0 {
          bytes += 3;
          break;
        } else {
          return Err(FormatError::InvalidSequence);
        }
      }

      let off = self.consumed + bytes;

      if off > self.buffer.len() {
        return Err(FormatError::EndOfFile);
      }

      let mb = &self.buffer[self.consumed..off];

      let mut c32: char32_t = 0;
      let mut st = MBState::new();
      let len = (self.ctype.converter.mbtoc32)(&mut c32, mb, &mut st);
      if len < 0 {
        return Err(FormatError::InvalidSequence);
      }

      self.consumed += len as usize;
      return Ok(c32);
    }
    Err(FormatError::EndOfFile)
  }

  #[inline]
  fn vomit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    let mut buf = [0u8; constants::MB_LEN_MAX];
    let ret = (self.ctype.converter.c32tomb)(&mut buf, ch);
    if ret < 0 {
      return Err(FormatError::InvalidSequence);
    }
    if self.consumed < (ret as usize) {
      return Err(FormatError::EndOfFile);
    }
    self.consumed -= ret as usize;
    Ok(())
  }

  #[inline]
  fn consume(&mut self) -> Result<Self::FormatChar, FormatError> {
    self.consume_u8()
  }

  #[inline]
  fn vomit(
    &mut self,
    ch: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.vomit_u8(ch)
  }

  #[inline]
  fn get_converted(&self) -> usize {
    self.converted
  }

  #[inline]
  fn increase_converted(&mut self) {
    self.converted += 1;
  }

  #[inline]
  fn get_read(&self) -> usize {
    self.consumed
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_vsscanf(
  buffer: *const c_char,
  format: *const c_char,
  mut vlist: VaList
) -> c_int {
  if buffer.is_null() || format.is_null() {
    return EOF;
  }

  let locale = locale::get_thread_locale();
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let format = unsafe {
    slice::from_raw_parts(format as *const u8, string::rs_strlen(format))
  };

  let mut consumer = StreamConsumer::new(buffer, &ctype);

  let result = scanf::scanf_inner(&locale, &mut consumer, format, &mut vlist);

  match result {
    | Ok(_) => consumer.get_converted() as c_int,
    | Err(e) => {
      if e.eligible_for_errno() {
        errno::set_errno(e.to_errno());
      }
      match e {
        | FormatError::EndOfFile if consumer.get_converted() == 0 => EOF,
        | _ => consumer.get_converted() as c_int
      }
    }
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_sscanf(
  buffer: *const c_char,
  format: *const c_char,
  args: ...
) -> c_int {
  rs_vsscanf(buffer, format, args.clone())
}
