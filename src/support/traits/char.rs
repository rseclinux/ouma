use core::{ascii::Char, convert::Into};

pub enum CharToAscii {
  Narrow(u8),
  Wide(u32)
}

impl Into<CharToAscii> for u8 {
  #[inline]
  fn into(self) -> CharToAscii {
    CharToAscii::Narrow(self)
  }
}

impl Into<CharToAscii> for u32 {
  #[inline]
  fn into(self) -> CharToAscii {
    CharToAscii::Wide(self)
  }
}

#[inline]
pub fn get_ascii_char(c: impl Into<CharToAscii>) -> Char {
  match c.into() {
    | CharToAscii::Narrow(c) => Char::from_u8(c).unwrap_or(Char::Null),
    | CharToAscii::Wide(wc) => {
      let Some(c) = char::from_u32(wc) else {
        return Char::Null;
      };
      c.as_ascii().unwrap_or(Char::Null)
    }
  }
}

#[inline]
pub fn get_ascii_char_with_index<T: Into<CharToAscii> + Copy>(
  src: &[T],
  index: usize
) -> Option<char> {
  src.get(index).map(|&c| get_ascii_char(c).to_char())
}

pub trait CharToUnicode: Sized {
  fn get_unicode_char(
    s: &[Self],
    index: usize
  ) -> Option<char>;
}

impl CharToUnicode for u8 {
  #[inline]
  fn get_unicode_char(
    s: &[Self],
    index: usize
  ) -> Option<char> {
    if s.is_empty() || index >= s.len() {
      return None;
    }

    let s = &s[index..];

    if (s[0] & 0x80) == 0 {
      let ch = s[0] as char;
      return Some(ch);
    }

    let mut bytes = 1usize;

    let offset = 0usize;
    while let Some(ch) = s.get(offset) &&
      offset < 4
    {
      if (ch & 0xe0) == 0xc0 {
        bytes += 1;
        break;
      } else if (ch & 0xf0) == 0xe0 {
        bytes += 2;
        break;
      } else if (ch & 0xf8) == 0xf0 {
        bytes += 3;
        break;
      } else {
        return None;
      }
    }

    let encoded = core::str::from_utf8(&s[..bytes]).ok()?;

    encoded.chars().nth(0)
  }
}

impl CharToUnicode for u32 {
  #[inline]
  fn get_unicode_char(
    s: &[Self],
    index: usize
  ) -> Option<char> {
    char::from_u32(s.get(index).copied()?)
  }
}

pub trait MatchChar: Sized {
  fn char_matches(
    a: char,
    b: &[Self],
    index: usize
  ) -> bool;
}

impl MatchChar for u8 {
  #[inline]
  fn char_matches(
    a: char,
    b: &[Self],
    index: usize
  ) -> bool {
    let len = a.len_utf8();
    let offset = index + len;

    if offset > b.len() {
      return false;
    }

    let slice = &b[index..offset];

    let mut buf = [0u8; 4];
    let encoded = a.encode_utf8(&mut buf);
    let encoded = encoded.as_bytes();

    slice.windows(encoded.len()).position(|window| window == encoded).is_some()
  }
}

impl MatchChar for u32 {
  #[inline]
  fn char_matches(
    a: char,
    b: &[Self],
    index: usize
  ) -> bool {
    let offset = index + 1;

    if offset > b.len() {
      return false;
    }

    let slice = &b[index..offset];

    let c = a as u32;
    slice.contains(&c)
  }
}
