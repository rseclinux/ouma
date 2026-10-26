use {
  crate::{
    std::string,
    types::{c_char, c_int, c_long}
  },
  core::{cell::SyncUnsafeCell, ptr, slice}
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
  value: c_long,
  buf: *mut c_char,
  len: c_int
) -> c_int {
  const B64_CHARS: &[u8] =
    b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

  let slice =
    unsafe { slice::from_raw_parts_mut(buf as *mut u8, len as usize) };
  let mut buflen = slice.len();
  let mut offset = 0usize;
  let mut value = value as u32;

  while buflen != 0 {
    if value == 0 {
      slice[offset] = 0;
      return 0;
    }

    slice[offset] = B64_CHARS[(value & 0x3f) as usize];
    value >>= 6;
    offset += 1;
    buflen -= 1;
  }

  -1
}

const L64_BUF_LEN: usize = 7;

#[thread_local]
static L64_BUF: SyncUnsafeCell<[u8; L64_BUF_LEN]> =
  SyncUnsafeCell::new([0u8; L64_BUF_LEN]);

#[unsafe(no_mangle)]
pub extern "C" fn rs_l64a(i: c_long) -> *mut c_char {
  let ptr = L64_BUF.get();
  let buf = unsafe { &mut *ptr };
  if rs_l64a_r(i, buf.as_mut_ptr().cast(), L64_BUF_LEN as i32) != 0 {
    return ptr::null_mut();
  }
  buf.as_mut_ptr().cast()
}
