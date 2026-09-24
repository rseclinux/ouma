use {
  super::constants::EOF,
  crate::{
    std::{errno, string},
    support::{
      format::{
        error::FormatError,
        scanf::{self, Consumer}
      },
      locale::{self, ctype::CtypeObject}
    },
    types::{c_char, c_int}
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
    if self.buffer.get(self.consumed).is_none() {
      return Err(FormatError::EndOfFile);
    }
    let ch = self.buffer[self.consumed];
    self.consumed += 1;
    Ok(ch)
  }

  #[inline]
  fn vomit_u8(
    &mut self,
    _: u8
  ) -> Result<(), FormatError> {
    debug_assert!(self.consumed > 0);
    debug_assert!(self.consumed <= self.buffer.len());
    self.buffer = &self.buffer[1..];
    self.consumed = self.consumed.saturating_sub(1);
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
  fn consume_u32(&mut self) -> Result<u32, FormatError> {
    let u = self.consume_u8()?;
    println!("consume u32 is {}", u.escape_ascii());
    Ok(u.into())
  }

  #[inline]
  fn vomit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    if ch <= 0x7f {
      return self.vomit_u8(ch as u8);
    }
    eprintln!("non ascii!");
    Err(FormatError::InvalidSequence)
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
