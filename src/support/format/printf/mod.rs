use {
  super::{error::FormatError, length::LengthModifier},
  crate::support::{
    float::f128::F128,
    traits::{char::CharToAscii, float::Float}
  },
  core::ascii
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;

pub mod float_format;
pub mod numbered_args;
pub mod strfrom;
pub mod values;

pub trait CastToDouble: Float {
  fn cast_to_double(self) -> f64;
}

impl CastToDouble for f64 {
  #[inline]
  fn cast_to_double(self) -> f64 {
    self
  }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl CastToDouble for F80 {
  #[inline]
  fn cast_to_double(self) -> f64 {
    self.as_f64()
  }
}

impl CastToDouble for F128 {
  #[inline]
  fn cast_to_double(self) -> f64 {
    self.as_f64()
  }
}

pub trait Emitter {
  type FormatChar: Copy + Into<CharToAscii>;

  fn get_written(&self) -> usize;
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError>;
  fn emit_ascii_slice(
    &mut self,
    s: &[ascii::Char]
  ) -> Result<(), FormatError>;
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError>;
  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError>;
  fn get_unicode_char_len(c: char) -> usize;

  #[inline]
  fn emit_ascii_char(
    &mut self,
    c: ascii::Char
  ) -> Result<(), FormatError> {
    self.emit_ascii_slice(&[c])
  }

  #[inline]
  fn emit_unicode_char(
    &mut self,
    c: char
  ) -> Result<(), FormatError> {
    self.emit_u32_slice(&[c as u32])
  }

  #[inline]
  fn pad_to(
    &mut self,
    arg: &PrintfArgument,
    c: ascii::Char,
    n: usize
  ) -> Result<(), FormatError> {
    let mut width = arg.width;
    while width > n {
      self.emit_ascii_char(c)?;
      width -= 1;
    }
    Ok(())
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PrintfFlags {
  pub alternate_form: bool,
  pub leading_zeroes: bool,
  pub left_align: bool,
  pub space_prefix: bool,
  pub prepend_plus: bool,
  pub group_decimals: bool
}

#[derive(Debug, Default, Clone)]
pub struct PrintfArgument {
  pub flags: PrintfFlags,
  pub width: usize,
  pub precision: Option<u32>,
  pub modifier: LengthModifier,
  pub specifier: char
}
