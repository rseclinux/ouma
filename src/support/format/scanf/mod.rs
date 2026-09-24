use {
  super::{error::FormatError, length::LengthModifier},
  crate::{
    std::wchar::UnicodeBitset,
    support::{
      locale::{self, ctype::CtypeObject},
      traits::char::{CharToAscii, MatchChar}
    }
  },
  core::ffi::VaList
};

pub mod utils;

#[derive(Default, Clone, Copy)]
pub struct ScanfArgument {
  pub suppress: bool,
  pub allocate: bool,
  pub width: usize,
  pub modifier: LengthModifier,
  pub scan_set: Option<UnicodeBitset>,
  pub specifier: char
}

pub trait Consumer {
  type FormatChar: Into<CharToAscii> + MatchChar + Copy;

  fn get_read(&self) -> usize;
  fn get_converted(&self) -> usize;
  fn increase_converted(&mut self);

  fn consume(&mut self) -> Result<Self::FormatChar, FormatError>;
  fn vomit(
    &mut self,
    ch: Self::FormatChar
  ) -> Result<(), FormatError>;

  fn consume_u32(&mut self) -> Result<u32, FormatError>;
  fn vomit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError>;
  fn consume_u8(&mut self) -> Result<u8, FormatError>;
  fn vomit_u8(
    &mut self,
    ch: u8
  ) -> Result<(), FormatError>;

  #[inline]
  fn consume_unicode_char(&mut self) -> Option<char> {
    let u32ch = self.consume_u32().ok()?;

    char::from_u32(u32ch)
  }

  #[inline]
  fn consume_whitespace(
    &mut self,
    ctype: &CtypeObject
  ) -> bool {
    let Some(ch) = self.consume_unicode_char() else {
      return false;
    };
    (ctype.casemap.isspace)(ch.into())
  }
}

#[inline]
pub fn scanf_inner<T: Consumer>(
  locale: &locale::Locale,
  consumer: &mut T,
  format: &[T::FormatChar],
  vlist: &mut VaList
) -> Result<(), FormatError> {
  Ok(())
}
