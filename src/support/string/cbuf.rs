use core::fmt::Write;

pub struct CBufWriter<'a> {
  buf: &'a mut [u8],
  pos: usize
}

impl<'a> CBufWriter<'a> {
  pub fn new(buf: &'a mut [u8]) -> Self {
    Self { buf, pos: 0 }
  }
}

impl<'a> Write for CBufWriter<'a> {
  fn write_str(
    &mut self,
    s: &str
  ) -> core::fmt::Result {
    let b = s.as_bytes();
    let rem = &mut self.buf[self.pos..];
    if b.len() > rem.len() {
      return Err(core::fmt::Error);
    }
    rem[..b.len()].copy_from_slice(b);
    self.pos += b.len();
    Ok(())
  }
}
