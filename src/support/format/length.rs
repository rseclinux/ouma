use crate::support::{
  locale::ctype::CtypeObject,
  string::conversion::strtoint::strtoint,
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
  let mut lm = match (one, two) {
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
    let is_fast = get_ascii_char_with_index(fmt, *index + 1) == Some('f');
    *index += if is_fast { 2 } else { 1 };
    let mut valid = false;
    if let Some(ch) = get_ascii_char_with_index(fmt, *index) &&
      (ctype.casemap.isdigit)(ch as u32)
    {
      let result = strtoint::<T, usize>(&fmt[*index..], 10, ctype);
      if matches!(result.value, 8 | 16 | 32 | 64) {
        *index += result.len;
        lm = if is_fast {
          LengthModifier::BitFast(result.value)
        } else {
          LengthModifier::Bit(result.value)
        };
        valid = true;
      }
    }
    if !valid {
      *index = start;
    }
  }
  lm
}
