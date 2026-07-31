use {
  crate::{
    c_char,
    c_int,
    char32_t,
    mbstate_t,
    size_t,
    std::{errno, wchar::mbstate},
    support::locale,
    wchar_t
  },
  core::{ptr, slice}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_mblen(
  s: *const c_char,
  n: size_t
) -> c_int {
  rs_mbtowc(ptr::null_mut(), s, n)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbtowc(
  pwc: *mut wchar_t,
  s: *const c_char,
  n: size_t
) -> c_int {
  if s.is_null() {
    return 0;
  }

  let locale = locale::get_thread_locale();
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();
  let s = unsafe { slice::from_raw_parts(s as *const u8, n) };

  let mut ps = mbstate_t::new();
  let mut c32: char32_t = 0;

  let result = (ctype.converter.mbtoc32)(&mut c32, s, &mut ps);
  if result < 0 {
    errno::set_errno(errno::EILSEQ);
    return -1;
  }

  if !pwc.is_null() {
    unsafe { *pwc = c32 as wchar_t };
  }

  if c32 == '\0' as char32_t { 0 } else { result as c_int }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbstowcs(
  pwcs: *mut wchar_t,
  s: *const c_char,
  n: size_t
) -> size_t {
  let mut s = s;
  mbstate::rs_mbsrtowcs(pwcs, &mut s, n, ptr::null_mut())
}
