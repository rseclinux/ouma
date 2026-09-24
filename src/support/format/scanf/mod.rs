use {
  super::{error::FormatError, length::LengthModifier},
  crate::{
    std::wchar::UnicodeBitset,
    support::{
      locale::{self, ctype::CtypeObject},
      traits::char::{CharToAscii, MatchChar, get_ascii_char}
    }
  },
  core::ffi::VaList,
  num_traits::ConstZero
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
  type FormatChar: Into<CharToAscii>
    + MatchChar
    + num_traits::ConstZero
    + PartialEq
    + Copy;

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
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();
  let _numeric = locale::get_slot(&locale.numeric).unwrap_or_default();
  let _numargs = vlist.clone();

  let mut index = 0usize;

  while index < format.len() {
    let ch = match format.get(index).copied() {
      | None => T::FormatChar::ZERO,
      | Some(c) => c
    };

    if get_ascii_char(ch).to_char() == '%' {
      eprintln!("format specifier");
    } else {
      if (ctype.casemap.isspace)(get_ascii_char(ch).into()) {
        loop {
          if !consumer.consume_whitespace(&ctype) {
            break;
          }
        }
        index += 1;
        continue;
      }

      let c_cur = consumer.consume()?;
      if c_cur != ch {
        consumer.vomit(c_cur)?;
        return Err(FormatError::BadMatch);
      }
    }

    index += 1;
  }

  Ok(())
}
