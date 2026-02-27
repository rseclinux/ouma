use {
  crate::{
    allocation::{borrow::Cow, string::String, vec::Vec},
    c_char,
    c_int,
    size_t,
    std::{errno, string}
  },
  core::{ffi, fmt, slice, str}
};

pub struct StringStream<'a> {
  data: &'a mut [c_char],
  writeptr: size_t,
  err: bool
}

impl<'a> StringStream<'a> {
  fn write(
    &mut self,
    bytes: *const c_char,
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

  pub fn new(buf: &'a mut [c_char]) -> Self {
    Self { data: &mut buf[..], writeptr: 0, err: false }
  }

  pub fn from_cchar(
    &mut self,
    c: c_char
  ) {
    self.write(&c, 1);
  }

  pub fn from_cstr(
    &mut self,
    cstr: *const c_char
  ) {
    self.write(cstr, string::rs_strlen(cstr));
  }

  pub fn from_str(
    &mut self,
    s: &str
  ) {
    self.write(s.as_ptr().cast(), s.len());
  }

  pub fn as_str(&mut self) -> Result<&str, c_int> {
    let s = unsafe {
      slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), self.data.len())
    };
    Ok(str::from_utf8(s).map_err(|_| errno::ENOENT)?)
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

pub fn strtocstr<'a>(s: &'a str) -> Cow<'a, [u8]> {
  let mut buf: Vec<u8> = Vec::new();

  buf.extend_from_slice(&s.as_bytes());

  buf.push(b'\0');

  Cow::Owned(buf)
}

pub fn strtowcstr<'a>(s: &'a str) -> Cow<'a, [u32]> {
  let mut buf: Vec<u32> = s.chars().into_iter().map(|c| c as u32).collect();

  buf.push('\0' as u32);

  Cow::Owned(buf)
}

pub fn cstrtostr<'a>(cs: &'a [u8]) -> Result<Cow<'a, str>, c_int> {
  let c = ffi::CStr::from_bytes_with_nul(cs).map_err(|_| errno::EILSEQ)?;

  Ok(Cow::Borrowed(c.to_str().map_err(|_| errno::EILSEQ)?))
}

pub fn wcstrtostr<'a>(wcs: &'a [u32]) -> Result<Cow<'a, str>, c_int> {
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
