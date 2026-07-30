pub mod char_format;
pub mod error_format;
pub mod float_format;
pub mod integer_format;
pub mod pointer_format;
pub mod string_format;

use {
  super::format::{FormatError, LengthModifier},
  crate::{
    c_int,
    stdio::format::{CChar, CString},
    support::{
      ffi::va_list::ExtVaList,
      locale::{self, Locale},
      traits::char::{CharToAscii, get_char_with_index}
    },
    types::wchar_t
  },
  core::{ascii, ffi::c_void},
  float_format::FloatConv
};

const PAD_CHUNK_SIZE: usize = 16;

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
    c: ascii::Char,
    n: usize
  ) -> Result<(), FormatError> {
    let current = self.get_written();
    if n > current {
      let mut count = n - current;
      if count == 0 {
        return Ok(());
      }
      let buf = [c; PAD_CHUNK_SIZE];
      while count > PAD_CHUNK_SIZE {
        self.emit_ascii_slice(&buf)?;
        count -= PAD_CHUNK_SIZE;
      }
      if count > 0 {
        self.emit_ascii_slice(&buf[..count])?;
      }
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
pub struct Argument {
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
  while let Some(c) = get_char_with_index(fmt, *index) {
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

// https://github.com/lights0123/printf-compat/blob/184776b9ad7e166928ed2d001e95881bbbfbaae1/src/parser.rs#L29
#[inline]
fn parse_width<T: Copy + Into<CharToAscii>>(
  fmt: &[T],
  index: &mut usize,
  va: &mut ExtVaList
) -> (c_int, bool) {
  let mut width: c_int = 0;
  if get_char_with_index(fmt, *index) == Some('*') {
    *index += 1;
    let arg: c_int = unsafe { va.next_arg() };
    return match arg.checked_neg() {
      | Some(magnitude) if arg < 0 => (magnitude, true),
      | _ if arg >= 0 => (arg, false),
      // arg == c_int::MIN: checked_neg() returns None, unrepresentable
      | _ => (c_int::MAX, true)
    };
  }
  while let Some(ch) = get_char_with_index(fmt, *index) {
    match ch {
      // https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html#the-bytes-solution
      | '0'..='9' => width = width * 10 + (ch as u8 & 0x0f) as c_int,
      | _ => break
    }
    *index += 1;
  }
  (width, false)
}

#[inline]
fn parse_precision<T: Copy + Into<CharToAscii>>(
  fmt: &[T],
  index: &mut usize,
  va: &mut ExtVaList
) -> Option<u32> {
  let mut precision: Option<u32> = None;
  if get_char_with_index(fmt, *index) == Some('.') {
    *index += 1;
    if get_char_with_index(fmt, *index) == Some('*') {
      *index += 1;
      let prec: c_int = unsafe { va.next_arg() };
      precision = if prec < 0 { None } else { Some(prec as u32) };
    } else {
      let (parsed, _) = parse_width(fmt, index, va);
      precision = if parsed < 0 { None } else { Some(parsed as u32) };
    }
  }
  precision
}

#[inline]
pub fn printf_inner<T: Emitter>(
  locale: &Locale,
  emitter: &mut T,
  fmt: &[T::FormatChar],
  ap: &mut ExtVaList
) -> Result<usize, FormatError> {
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();
  let messages = locale::get_slot(&locale.messages).unwrap_or_default();
  let numeric = locale::get_slot(&locale.numeric).unwrap_or_default();

  let mut index = 0usize;

  while index < fmt.len() {
    if get_char_with_index(fmt, index) == Some('%') {
      index += 1;

      // Parse flags
      let mut flags = parse_flags(fmt, &mut index);

      // Parse width
      let (width, force_left_align) = parse_width(fmt, &mut index, ap);
      flags.left_align = force_left_align;

      // Parse precision
      let precision = parse_precision(fmt, &mut index, ap);

      // Parse length modifier and bit width
      let lm = super::format::parse_length_modifier(fmt, &mut index, &ctype);

      let specifier: char = get_char_with_index(fmt, index).unwrap_or('\0');

      // Construct argument struct
      let arg = Argument {
        flags,
        width: width as usize,
        precision,
        modifier: lm,
        specifier
      };

      match arg.specifier {
        | '%' => emitter.emit_ascii_char(ascii::Char::PercentSign)?,
        | 'i' | 'd' => integer_format::format_signed(
          emitter,
          unsafe { lm.parse_signed(ap)? },
          &arg,
          &ctype,
          &numeric
        )?,
        | 'b' | 'B' | 'o' | 'x' | 'X' | 'u' => integer_format::format_unsigned(
          emitter,
          unsafe { lm.parse_unsigned(ap)? },
          &arg,
          &ctype,
          &numeric
        )?,
        | 'a' | 'A' => todo!("hexadecimal fmt n/a"),
        | 'e' | 'E' => {
          let val = unsafe { lm.parse_float(ap)? };
          float_format::format_float(
            emitter,
            val,
            FloatConv::E,
            &arg,
            &ctype,
            &numeric
          )?
        },
        | 'f' | 'F' => {
          let val = unsafe { lm.parse_float(ap)? };
          float_format::format_float(
            emitter,
            val,
            FloatConv::F,
            &arg,
            &ctype,
            &numeric
          )?
        },
        | 'g' | 'G' => {
          let val = unsafe { lm.parse_float(ap)? };
          float_format::format_float(
            emitter,
            val,
            FloatConv::G,
            &arg,
            &ctype,
            &numeric
          )?
        },
        | 'p' => {
          let value: *const () = unsafe { ap.next_arg() };
          pointer_format::format_pointer(
            emitter,
            value as *const c_void,
            &arg,
            &ctype,
            &numeric
          )?;
        },
        | 'c' => {
          let val = unsafe { lm.parse_cchar(ap)? };
          char_format::format_char(emitter, val, &arg)?;
        },
        | 'C' => {
          let val: u32 = unsafe { ap.next_arg() };
          char_format::format_char(emitter, CChar::Wide(val), &arg)?;
        },
        | 's' => {
          let val = unsafe { lm.parse_cstr(ap)? };
          string_format::format_string(emitter, val, &arg)?;
        },
        | 'S' => {
          let ptr: *const wchar_t = unsafe { ap.next_arg() };
          let slice = if ptr.is_null() {
            u32str_from_ascii!(b"(null)")
          } else {
            unsafe {
              core::slice::from_raw_parts(
                ptr as *const u32,
                crate::std::wchar::rs_wcslen(ptr)
              )
            }
          };
          let v = CString::Wide(slice);
          string_format::format_string(emitter, v, &arg)?;
        },
        | 'm' => error_format::format_error(
          &arg, emitter, &messages, &ctype, &numeric
        )?,
        | 'n' => panic!("Usage of %n has been detected. Aborting."),
        | _ => emitter.emit_unicode_char(arg.specifier)?
      }

      index += 1;
    } else {
      let start = index;
      while index < fmt.len() && get_char_with_index(fmt, index) != Some('%') {
        index += 1;
      }
      T::emit_format_string(emitter, &fmt[start..index])?;
    }
  }

  Ok(emitter.get_written())
}
