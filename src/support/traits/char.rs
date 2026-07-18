use core::{ascii::Char, convert::Into};

pub enum CharToAscii {
  Narrow(u8),
  Wide(u32)
}

impl Into<CharToAscii> for u8 {
  fn into(self) -> CharToAscii {
    CharToAscii::Narrow(self)
  }
}

impl Into<CharToAscii> for u32 {
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
pub fn get_char_with_index<T: Into<CharToAscii> + Copy>(
  src: &[T],
  index: usize
) -> Option<char> {
  src.get(index).map(|&c| get_ascii_char(c).to_char())
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

    let b = if len > 1 {
      let off = len + index;
      &b[index..off]
    } else {
      &b[index..]
    };

    let mut buf = [0u8; 4];
    let encoded = a.encode_utf8(&mut buf);
    let encoded = encoded.as_bytes();

    b.windows(encoded.len()).position(|window| window == encoded).is_some()
  }
}

impl MatchChar for u32 {
  #[inline]
  fn char_matches(
    a: char,
    b: &[Self],
    index: usize
  ) -> bool {
    let c = a as u32;
    let b = &b[index..];
    b.contains(&c)
  }
}
