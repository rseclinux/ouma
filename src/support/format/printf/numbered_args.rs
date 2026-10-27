use {
  super::values::*,
  crate::support::{
    ffi::va_list::ExtVaList,
    format::{
      get_number,
      get_numbered_arg,
      length::{self, LengthModifier}
    },
    locale::ctype::CtypeObject,
    traits::char::{CharToAscii, get_ascii_char, get_ascii_char_with_index}
  },
  num_traits::ConstZero
};

const NUMBERED_ARG_MAX: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumArgKind {
  Signed,
  Unsigned,
  Float,
  Char,
  String,
  Pointer,
  NotSpecified
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumArgType {
  pub kind: NumArgKind,
  pub lm: LengthModifier
}

impl Default for NumArgType {
  #[inline]
  fn default() -> Self {
    Self { kind: NumArgKind::NotSpecified, lm: LengthModifier::Default }
  }
}

#[inline]
fn safe_strchr<T: Into<CharToAscii> + Copy>(
  s: &str,
  key: T
) -> Option<usize> {
  s.chars().position(|b| b == get_ascii_char(key).to_char())
}

#[inline]
fn safe_strspn<T: Into<CharToAscii> + Copy>(
  s: &[T],
  key: &str
) -> usize {
  s.iter().take_while(|&&c| key.contains(get_ascii_char(c).to_char())).count()
}

#[inline]
pub fn get_numarg_length<T: Into<CharToAscii> + Copy + ConstZero>(
  format: &[T],
  ctype: &CtypeObject
) -> usize {
  let mut result = 0usize;
  let mut index = 0usize;

  while index < format.len() {
    let ch = format.get(index).copied().unwrap_or(T::ZERO);
    index += 1;

    if get_ascii_char(ch).to_char() == '%' {
      if result >= NUMBERED_ARG_MAX {
        return result;
      }

      let mut got_numarg = false;
      {
        let numarg = get_numbered_arg(format, &mut index, ctype);
        if numarg > NUMBERED_ARG_MAX {
          return 0;
        }
        if numarg > 0 {
          got_numarg = true;
        }
        if result < numarg {
          result = numarg;
        }
      }

      while let Some(c) = format.get(index).copied() {
        if safe_strchr("dibBouxXfFeEgGaAcspCS%m", c).is_some() {
          break;
        }
        index += 1;

        if get_ascii_char(c).to_char() == '*' {
          let numarg = get_numbered_arg(format, &mut index, ctype);
          if numarg == 0 || numarg > NUMBERED_ARG_MAX {
            return 0;
          }
          if numarg == 0 {
            return 0;
          }
          if result < numarg {
            result = numarg;
          }
        }
      }

      let spec = get_ascii_char_with_index(format, index).unwrap_or('\0');
      index += 1;

      if spec != '%' && spec != 'm' && !got_numarg {
        return 0;
      }
    }
  }

  result
}

fn get_numarg_types<'a, T: Into<CharToAscii> + Copy + ConstZero>(
  format: &[T],
  arg_num_len: usize,
  buffer: &'a mut [NumArgType; NUMBERED_ARG_MAX],
  ctype: &CtypeObject
) -> &'a [NumArgType] {
  let mut index = 0usize;

  while index < format.len() {
    let ch = format.get(index).copied().unwrap_or(T::ZERO);
    index += 1;

    if get_ascii_char(ch).to_char() == '%' {
      let mut arg_v = get_numbered_arg(format, &mut index, ctype);
      if arg_v > arg_num_len {
        return &[];
      }

      index += safe_strspn(&format[index..], "'-+ #0");

      if get_ascii_char_with_index(format, index) == Some('*') {
        index += 1;
        let numarg = get_numbered_arg(format, &mut index, ctype);
        if numarg == 0 || numarg > arg_num_len {
          return &[];
        }
        let arg_field_width = numarg - 1;

        buffer[arg_field_width].lm = LengthModifier::Default;
        buffer[arg_field_width].kind = NumArgKind::Signed;
      } else {
        get_number(format, &mut index, ctype);
      }

      if get_ascii_char_with_index(format, index) == Some('.') {
        index += 1;
        if get_ascii_char_with_index(format, index) == Some('*') {
          index += 1;
          let numarg = get_numbered_arg(format, &mut index, ctype);
          if numarg == 0 || numarg > arg_num_len {
            return &[];
          }
          let arg_field_width = numarg - 1;

          buffer[arg_field_width].lm = LengthModifier::Default;
          buffer[arg_field_width].kind = NumArgKind::Signed;
        } else {
          get_number(format, &mut index, ctype);
        }
      }

      let lm = length::parse_length_modifier(format, &mut index, ctype);
      buffer[arg_v].lm = lm;

      let specifier = get_ascii_char_with_index(format, index).unwrap_or('\0');
      index += 1;

      if arg_v == 0 {
        continue;
      }
      arg_v -= 1;

      match specifier {
        | 'd' | 'i' => {
          buffer[arg_v].kind = NumArgKind::Signed;
        },
        | 'b' | 'B' | 'o' | 'u' | 'x' | 'X' => {
          buffer[arg_v].kind = NumArgKind::Unsigned;
        },
        | 'a' | 'A' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G' => {
          buffer[arg_v].kind = NumArgKind::Float;
        },
        | 'c' => {
          buffer[arg_v].kind = NumArgKind::Char;
        },
        | 'C' => {
          buffer[arg_v].lm = LengthModifier::Long;
          buffer[arg_v].kind = NumArgKind::Char;
        },
        | 's' => {
          buffer[arg_v].kind = NumArgKind::String;
        },
        | 'S' => {
          buffer[arg_v].lm = LengthModifier::Long;
          buffer[arg_v].kind = NumArgKind::String;
        },
        | 'p' => {
          buffer[arg_v].lm = LengthModifier::Ptrdiff;
          buffer[arg_v].kind = NumArgKind::Pointer;
        },
        | _ => {}
      };
    }
  }

  &buffer[..arg_num_len]
}

#[inline]
pub unsafe fn get_numarg_values<'a>(
  len: usize,
  types: &[NumArgType],
  buffer: &'a mut [ArgumentValues; NUMBERED_ARG_MAX],
  args: &mut ExtVaList
) -> usize {
  if len == 0 || types.is_empty() {
    return 0;
  }

  let mut idx = 0usize;
  while let Some(t) = types.get(idx).copied() &&
    idx < len
  {
    match t.kind {
      | NumArgKind::Char => {
        let v = unsafe { CChar::from_raw(t.lm, args) };
        match v {
          | Some(x) => buffer[idx] = ArgumentValues::Char(x),
          | None => buffer[idx] = ArgumentValues::NotSpecified
        };
      },
      | NumArgKind::Float => {
        let v = unsafe { FloatValue::from_raw(t.lm, args) };
        match v {
          | Some(x) => buffer[idx] = ArgumentValues::Float(x),
          | None => buffer[idx] = ArgumentValues::NotSpecified
        };
      },
      | NumArgKind::Pointer => {
        let v = unsafe { CPointer::from_raw(args) };
        buffer[idx] = ArgumentValues::Pointer(v);
      },
      | NumArgKind::Signed => {
        let v = unsafe { Signed::from_raw(t.lm, args) };
        match v {
          | Some(x) => buffer[idx] = ArgumentValues::Signed(x),
          | None => buffer[idx] = ArgumentValues::NotSpecified
        };
      },
      | NumArgKind::String => {
        let v = unsafe { CString::from_raw(t.lm, args) };
        match v {
          | Some(x) => buffer[idx] = ArgumentValues::String(x),
          | None => buffer[idx] = ArgumentValues::NotSpecified
        };
      },
      | NumArgKind::Unsigned => {
        let v = unsafe { Unsigned::from_raw(t.lm, args) };
        match v {
          | Some(x) => buffer[idx] = ArgumentValues::Unsigned(x),
          | None => buffer[idx] = ArgumentValues::NotSpecified
        };
      },
      | _ => buffer[idx] = ArgumentValues::NotSpecified
    }

    idx += 1;
  }

  idx
}
