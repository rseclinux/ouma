use {
  super::{Emitter, FormatError, printf_inner},
  crate::{
    c_char,
    c_int,
    support::{ffi::va_list::ExtVaList, locale::get_thread_locale}
  },
  core::{
    ascii,
    ffi::{CStr, VaList, c_void}
  },
  syscalls::{Sysno, raw_syscall}
};

unsafe fn write(
  fd: i32,
  buf: *const c_void,
  len: usize
) {
  let _ = unsafe { raw_syscall!(Sysno::write, fd, buf, len) };
}

struct LOL(usize);

impl LOL {
  fn emit_u8(
    &mut self,
    v: u8
  ) -> Result<(), FormatError> {
    unsafe { write(1, &v as *const u8 as *const c_void, 1) };
    self.0 += 1;
    Ok(())
  }

  fn emit_ascii(
    &mut self,
    v: ascii::Char
  ) -> Result<(), FormatError> {
    self.emit_u8(v as u8)
  }

  fn emit_u32(
    &mut self,
    v: u32
  ) -> Result<(), FormatError> {
    let c = char::from_u32(v).unwrap_or_default();
    let mut buf = [0u8; 4];
    let conv = c.encode_utf8(&mut buf);
    let slice = conv.as_bytes();
    unsafe {
      write(1, slice.as_ptr() as *const c_void, slice.len());
    };
    Ok(())
  }
}

impl Emitter for LOL {
  type FormatChar = u8;

  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    for i in s.iter().copied() {
      self.emit_u8(i)?;
    }
    Ok(())
  }

  fn emit_ascii_slice(
    &mut self,
    s: &[ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s.iter().copied() {
      self.emit_ascii(i)?;
    }
    Ok(())
  }

  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError> {
    for i in s.iter().copied() {
      self.emit_u32(i)?;
    }
    Ok(())
  }

  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError> {
    self.emit_u8_slice(s)
  }

  fn get_written(&self) -> usize {
    self.0
  }
}

extern "C" fn my_vprintf(
  fmt: *const c_char,
  ap: VaList
) -> c_int {
  let mut va = unsafe { ExtVaList::from_va_list(ap) };
  let locale = get_thread_locale();
  let mut e = LOL(0);
  let cstr = unsafe { CStr::from_ptr(fmt) };
  let ret = printf_inner(locale, &mut e, cstr.to_bytes(), &mut va);
  match ret {
    | Ok(x) => x as c_int,
    | Err(_) => -1
  }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn my_printf(
  fmt: *const c_char,
  ap: ...
) -> c_int {
  my_vprintf(fmt, ap.clone())
}
