use {
  super::{error::FormatError, length::LengthModifier},
  crate::support::{
    ffi::va_list::ExtVaList,
    float::f128::F128,
    format::length,
    locale::{
      ctype::CtypeObject,
      messages::MessagesObject,
      numeric::NumericObject
    },
    traits::{
      char::{CharToAscii, get_ascii_char, get_ascii_char_with_index},
      float::Float
    }
  },
  core::ascii,
  num_traits::ConstZero,
  numbered_args::{NUMBERED_ARG_MAX, NumArgType},
  values::ArgumentValues
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::support::float::intel_extended::F80;

pub mod float_format;
pub mod integer_format;
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
  type FormatChar: Copy + Into<CharToAscii> + num_traits::ConstZero;

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
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
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
  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_format_char(*i)?;
    }
    Ok(())
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

#[inline]
fn parse_flags<T: Copy + Into<CharToAscii>>(
  fmt: &[T],
  index: &mut usize
) -> PrintfFlags {
  let mut result = PrintfFlags::default();
  while let Some(c) = get_ascii_char_with_index(fmt, *index) {
    match c.into() {
      | '#' => result.alternate_form = true,
      | '0' => result.leading_zeroes = true,
      | '+' => result.prepend_plus = true,
      | '-' => result.left_align = true,
      | '\'' => result.group_decimals = true,
      | ' ' => result.space_prefix = true,
      | _ => break
    }
    *index += 1;
  }
  result
}

#[inline]
pub fn printf_inner<'a, E: Emitter>(
  emitter: &mut E,
  ctype: &CtypeObject,
  numeric: &NumericObject,
  _messages: &MessagesObject,
  format: &[E::FormatChar],
  args: ExtVaList<'a>
) -> Result<usize, FormatError> {
  let mut index = 0usize;

  let mut start_args = args.clone();
  let mut type_buffer = [NumArgType::default(); NUMBERED_ARG_MAX];
  let mut value_buffer = [ArgumentValues::NotSpecified; NUMBERED_ARG_MAX];

  let numarg_len = numbered_args::get_numarg_length(format, ctype);
  let numarg_types = numbered_args::get_numarg_types(
    format,
    numarg_len,
    &mut type_buffer,
    ctype
  );
  let numarg_values_len = unsafe {
    numbered_args::get_numarg_values(
      numarg_len,
      numarg_types,
      &mut value_buffer,
      &mut start_args
    )
  };
  let use_numbered_args = numarg_values_len > 0;

  let mut args_cache = numbered_args::VaListCache::new(args.clone());
  if use_numbered_args {
    args_cache.set_numered_args(&value_buffer[..numarg_values_len]);
  }

  while index < format.len() {
    let ch = format.get(index).copied().unwrap_or(E::FormatChar::ZERO);
    index += 1;

    if get_ascii_char(ch).to_char() == '%' {
      let pos_idx = if !use_numbered_args {
        0
      } else {
        let n = super::get_numbered_arg(format, &mut index, ctype);
        if n == 0 || n > numarg_values_len {
          return Err(FormatError::InvalidArg);
        }
        n - 1
      };

      let mut flags = parse_flags(format, &mut index);
      if flags.left_align {
        flags.leading_zeroes = false;
      }

      let width = if get_ascii_char_with_index(format, index) == Some('*') {
        index += 1;
        let n = super::get_numbered_arg(format, &mut index, ctype);
        let arg_field_width = if !use_numbered_args {
          0
        } else if n == 0 || n > numarg_values_len {
          return Err(FormatError::InvalidArg);
        } else {
          n - 1
        };
        let arg = unsafe {
          args_cache.get_signed(LengthModifier::Default, arg_field_width)?
        };
        match arg {
          | values::Signed::Int(v) if v < 0 => {
            flags.left_align = true;
            flags.leading_zeroes = false;
            (v as isize).unsigned_abs() as usize
          },
          | values::Signed::Int(v) => v as usize,
          | _ => 0
        }
      } else {
        super::get_number(format, &mut index, ctype)
      };

      let mut precision: Option<u32> = None;
      if get_ascii_char_with_index(format, index) == Some('.') {
        index += 1;
        if get_ascii_char_with_index(format, index) == Some('*') {
          index += 1;
          let n = super::get_numbered_arg(format, &mut index, ctype);
          let arg_precision = if !use_numbered_args {
            0
          } else if n == 0 || n > numarg_values_len {
            return Err(FormatError::InvalidArg);
          } else {
            n - 1
          };
          let arg = unsafe {
            args_cache.get_signed(LengthModifier::Default, arg_precision)?
          };
          precision = match arg {
            | values::Signed::Int(v) if v >= 0 => Some(v as u32),
            | _ => None
          };
        } else {
          precision = Some(super::get_number(format, &mut index, ctype) as u32);
        }
      }

      let lm = length::parse_length_modifier(format, &mut index, ctype);

      let specifier = get_ascii_char_with_index(format, index).unwrap_or('\0');
      index += 1;

      let arg =
        PrintfArgument { flags, width, precision, modifier: lm, specifier };

      match arg.specifier {
        | 'd' | 'i' => {
          let value = unsafe { args_cache.get_signed(arg.modifier, pos_idx)? };
          integer_format::format_signed(emitter, value, &arg, ctype, numeric)?;
        },
        | 'b' | 'B' | 'o' | 'u' | 'x' | 'X' => {
          let value =
            unsafe { args_cache.get_unsigned(arg.modifier, pos_idx)? };
          integer_format::format_unsigned(
            emitter, value, &arg, ctype, numeric
          )?;
        },
        | _ => {
          if let Some(c) = format.get(index - 1).copied() {
            emitter.emit_format_char(c)?;
          }
        },
      }
    } else {
      emitter.emit_format_char(ch)?;
    }
  }

  Ok(emitter.get_written())
}
