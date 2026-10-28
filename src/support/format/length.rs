use crate::support::{
  locale::ctype::CtypeObject,
  string::conversion::strtoint,
  traits::char::{CharToAscii, get_ascii_char_with_index}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthModifier {
  Byte,
  Short,
  Default,
  Long,
  LongLong,
  LongFloat,
  Size,
  Intmax,
  Ptrdiff,
  Bit(usize),
  BitFast(usize)
}

impl Default for LengthModifier {
  #[inline]
  fn default() -> Self {
    Self::Default
  }
}

#[inline]
pub fn parse_length_modifier<'a, T: Copy + Into<CharToAscii>>(
  fmt: &[T],
  index: &mut usize,
  ctype: &CtypeObject<'a>
) -> LengthModifier {
  let one = get_ascii_char_with_index(fmt, *index);
  let two = get_ascii_char_with_index(fmt, *index + 1);
  let lm = match (one, two) {
    | (Some('h'), Some('h')) => {
      *index += 2;
      LengthModifier::Byte
    },
    | (Some('l'), Some('l')) => {
      *index += 2;
      LengthModifier::LongLong
    },
    | (Some('h'), _) => {
      *index += 1;
      LengthModifier::Short
    },
    | (Some('l'), _) => {
      *index += 1;
      LengthModifier::Long
    },
    | (Some('j'), _) => {
      *index += 1;
      LengthModifier::Intmax
    },
    | (Some('z'), _) => {
      *index += 1;
      LengthModifier::Size
    },
    | (Some('t'), _) => {
      *index += 1;
      LengthModifier::Ptrdiff
    },
    | (Some('L'), _) => {
      *index += 1;
      LengthModifier::LongFloat
    },
    | _ => LengthModifier::Default
  };
  if get_ascii_char_with_index(fmt, *index) == Some('w') {
    let start = *index;
    let fast = if get_ascii_char_with_index(fmt, *index + 1) == Some('f') {
      *index += 2;
      true
    } else {
      *index += 1;
      false
    };
    if get_ascii_char_with_index(fmt, *index)
      .and_then(|f| Some((ctype.casemap.isdigit)(f as u32)))
      .unwrap_or(false)
    {
      let result: strtoint::StrToIntResult<usize> =
        strtoint::strtoint(&fmt[*index..], 10, ctype);
      if !matches!(result.value, 8 | 16 | 32 | 64) {
        *index = start;
        return LengthModifier::Default;
      }

      *index += result.len;

      let value = result.value.max(0);
      if fast {
        return LengthModifier::BitFast(value);
      } else {
        return LengthModifier::Bit(value);
      }
    } else {
      if fast {
        *index -= 2;
      } else {
        *index -= 1;
      }
      return LengthModifier::Default;
    }
  }
  lm
}
