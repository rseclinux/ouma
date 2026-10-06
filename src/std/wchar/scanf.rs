use {
  crate::{
    std::{errno, stdio::constants::EOF, stdlib},
    support::{
      format::{
        error::FormatError,
        scanf::{self, Consumer}
      },
      locale::{self, ctype::CtypeObject}
    },
    types::{c_int, wchar_t}
  },
  core::{ffi::VaList, slice}
};

struct StreamConsumer<'a> {
  buffer: &'a [u32],
  multibyte: [u8; stdlib::constants::MB_LEN_MAX],
  consumed: usize,
  converted: usize,
  pending: usize,
  pending_len: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> StreamConsumer<'a> {
  #[inline]
  pub fn new(
    ptr: *const wchar_t,
    ctype: &'a CtypeObject
  ) -> Self {
    let len = super::rs_wcslen(ptr);
    let buffer = unsafe { slice::from_raw_parts(ptr as *const u32, len) };
    let multibyte = [0u8; stdlib::constants::MB_LEN_MAX];
    Self {
      buffer,
      multibyte,
      consumed: 0,
      converted: 0,
      pending: 0,
      pending_len: 0,
      ctype
    }
  }
}

impl<'a> Consumer for StreamConsumer<'a> {
  type FormatChar = u32;

  #[inline]
  fn consume_u32(&mut self) -> Result<u32, FormatError> {
    if self.consumed < self.buffer.len() {
      let ch = self.buffer[self.consumed];
      self.consumed += 1;
      return Ok(ch);
    }
    Err(FormatError::EndOfFile)
  }

  #[inline]
  fn vomit_u32(
    &mut self,
    _: u32
  ) -> Result<(), FormatError> {
    debug_assert!(self.consumed > 0);
    debug_assert!(self.consumed <= self.buffer.len());
    self.consumed -= 1;
    Ok(())
  }

  #[inline]
  fn consume_u8(&mut self) -> Result<u8, FormatError> {
    if self.pending < self.pending_len {
      let b = self.multibyte[self.pending];
      self.pending += 1;
      return Ok(b);
    }
    if self.consumed >= self.buffer.len() {
      return Err(FormatError::EndOfFile);
    }
    let c32 = self.buffer[self.consumed];
    let ret = (self.ctype.converter.c32tomb)(&mut self.multibyte, c32);
    if ret <= 0 {
      return Err(FormatError::InvalidSequence);
    }
    self.consumed += 1;
    self.pending_len = ret as usize;
    self.pending = 1;
    Ok(self.multibyte[0])
  }

  #[inline]
  fn vomit_u8(
    &mut self,
    _ch: u8
  ) -> Result<(), FormatError> {
    if self.pending > 0 {
      self.pending -= 1;
      return Ok(());
    }
    debug_assert!(self.pending_len > 0);
    debug_assert!(self.consumed > 0);
    if self.pending_len > 0 {
      self.consumed -= 1;
      self.pending_len = 0;
    }
    Ok(())
  }

  #[inline]
  fn consume(&mut self) -> Result<Self::FormatChar, FormatError> {
    self.consume_u32()
  }

  #[inline]
  fn vomit(
    &mut self,
    ch: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.vomit_u32(ch)
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
pub extern "C" fn rs_vswscanf(
  buffer: *const wchar_t,
  format: *const wchar_t,
  mut vlist: VaList
) -> c_int {
  if buffer.is_null() || format.is_null() {
    return EOF;
  }

  let locale = locale::get_thread_locale();
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let format = unsafe {
    slice::from_raw_parts(format as *const u32, super::rs_wcslen(format))
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
pub unsafe extern "C" fn rs_swscanf(
  buffer: *const wchar_t,
  format: *const wchar_t,
  args: ...
) -> c_int {
  rs_vswscanf(buffer, format, args.clone())
}
