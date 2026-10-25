use {
  crate::{
    std::string,
    types::{c_char, c_int, c_long}
  },
  core::slice
};

#[inline]
fn b64_char_to_int(ch: u8) -> i32 {
  let ch = ch as char;
  if ch == '.' {
    return 0;
  }
  if ch == '/' {
    return 1;
  }

  if !ch.is_ascii_alphanumeric() {
    return -1;
  }

  let is_lower = ch.is_ascii_lowercase();

  ch.to_digit(36).and_then(|f| Some(f as i32)).unwrap_or(0) +
    2 +
    if is_lower { 26 } else { 0 }
}

#[inline]
fn b64_int_to_char(value: u32) -> Option<u8> {
  debug_assert!(value < 64);
  if value == 0 {
    return Some(b'.');
  }
  if value == 1 {
    return Some(b'/');
  }
  if value < 38 {
    return char::from_digit(value - 2, 36)
      .and_then(|f| Some(f.to_ascii_uppercase() as u8));
  }
  char::from_digit(value - 2 - 26, 36)
    .and_then(|f| Some(f.to_ascii_lowercase() as u8))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_a64l(s: *const c_char) -> c_long {
  let s =
    unsafe { slice::from_raw_parts(s as *const u8, string::rs_strlen(s)) };
  let mut result: i32 = 0;
  let mut offset = 0usize;

  while let Some(s) = s.get(offset).copied() &&
    offset < 6
  {
    let value = b64_char_to_int(s);
    if value == -1 {
      break;
    }
    let mul = (6 * offset) as i32;
    result += value << mul;
    offset += 1;
  }

  result as c_long
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_l64a_r(
  i: c_long,
  buf: *mut c_char,
  len: c_int
) -> c_int {
  if buf.is_null() || len < 1 {
    return -1;
  }

  let slice =
    unsafe { slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
  let mut i = i as u32;
  let mut offset = 0usize;

  while i != 0 {
    if offset + 1 >= slice.len() {
      return -1;
    }
    let Some(c) = b64_int_to_char(i % 64) else {
      return -1;
    };
    slice[offset] = c;
    i /= 64;
    offset += 1;
  }
  slice[offset] = 0;

  0
}
