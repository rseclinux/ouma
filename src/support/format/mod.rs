use crate::support::{
  locale::ctype::CtypeObject,
  traits::char::{CharToAscii, get_ascii_char_with_index}
};

pub mod error;
pub mod grouping;
pub mod length;

#[inline]
fn get_number<T: Into<CharToAscii> + Copy>(
  s: &[T],
  index: &mut usize,
  ctype: &CtypeObject
) -> usize {
  let mut value = 0usize;
  while let Some(c) = get_ascii_char_with_index(s, *index) &&
    (ctype.casemap.isdigit)(c as u32)
  {
    *index += 1;
    value = value * 10 + (c as u8 & 0x0f) as usize;
  }
  value
}

#[inline]
fn get_numbered_arg<T: Into<CharToAscii> + Copy>(
  s: &[T],
  index: &mut usize,
  ctype: &CtypeObject
) -> usize {
  let start = *index;
  let value = get_number(s, index, ctype);
  if get_ascii_char_with_index(s, *index) != Some('$') {
    *index = start;
    return 0;
  }
  *index += 1;
  value
}
