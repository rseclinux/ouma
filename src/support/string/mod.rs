use {
  crate::{
    allocation::{borrow::Cow, ffi::CString, string::String, vec::Vec},
    c_char,
    c_int,
    size_t,
    std::{errno, string}
  },
  core::{ffi::CStr, fmt, slice, str}
};

pub struct StringStream<'a> {
  data: &'a mut [u8],
  writeptr: size_t,
  err: bool
}

impl<'a> StringStream<'a> {
  fn write(
    &mut self,
    bytes: *const u8,
    size: size_t
  ) {
    let b = unsafe { slice::from_raw_parts(bytes, size) };
    let ds = self.data.len();
    let mut i = 0;
    while self.writeptr < ds && i < size {
      self.data[self.writeptr] = b[i];
      i += 1;
      self.writeptr += 1;
    }
    if i < size {
      self.err = true;
    }
  }

  pub fn new(buf: &'a mut [u8]) -> Self {
    Self { data: &mut buf[..], writeptr: 0, err: false }
  }

  pub fn from_cstr(
    &mut self,
    cstr: *const u8
  ) {
    self.write(cstr, string::rs_strlen(cstr as *const c_char));
  }

  pub fn from_str(
    &mut self,
    s: &str
  ) {
    self.write(s.as_ptr().cast(), s.len());
  }

  pub fn to_str(&mut self) -> Result<&str, c_int> {
    let s = unsafe {
      slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), self.data.len())
    };
    Ok(str::from_utf8(s).map_err(|_| errno::EINVAL)?)
  }

  pub fn has_overflow(&self) -> bool {
    self.err
  }
}

impl<'a> fmt::Write for StringStream<'a> {
  fn write_str(
    &mut self,
    s: &str
  ) -> fmt::Result {
    self.from_str(s);
    Ok(())
  }
}

#[inline]
pub fn strtocstr(s: &str) -> Cow<'static, CStr> {
  let bytes: Vec<u8> = s.bytes().take_while(|&b| b != 0).collect();

  unsafe { Cow::Owned(CString::from_vec_unchecked(bytes)) }
}

#[inline]
pub fn cstrtostr<'a>(cs: &'a CStr) -> Cow<'a, str> {
  cs.to_string_lossy()
}

#[inline]
pub fn strtowcstr(s: &str) -> Cow<'static, [u32]> {
  let mut buf: Vec<u32> = s.chars().into_iter().map(|c| c as u32).collect();

  buf.push('\0' as u32);

  Cow::Owned(buf)
}

#[inline]
pub fn wcstrtostr(wcs: &[u32]) -> Result<Cow<'static, str>, c_int> {
  let position =
    wcs.iter().position(|&c| c == '\0' as u32).ok_or(errno::EILSEQ)?;

  if position + 1 != wcs.len() {
    return Err(errno::EILSEQ);
  }

  let content = &wcs[..position];

  let mut result: String = String::with_capacity(content.len());

  for &c in content {
    let ch = char::from_u32(c).ok_or(errno::EILSEQ)?;

    result.push(ch);
  }

  Ok(Cow::Owned(result))
}
