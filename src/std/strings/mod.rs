use {
  crate::{
    c_char,
    c_int,
    c_long,
    c_longlong,
    locale_t,
    size_t,
    std::string,
    support::locale
  },
  core::{
    cmp::Ordering,
    ffi::{self, c_void},
    str
  }
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_bcmp(
  lhs: *const c_void,
  rhs: *const c_void,
  n: size_t
) -> c_int {
  string::rs_memcmp(lhs, rhs, n)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_bcopy(
  src: *const c_void,
  dst: *mut c_void,
  n: size_t
) {
  string::rs_memcpy(dst, src, n);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_bzero(
  p: *mut c_void,
  n: size_t
) {
  string::rs_memset(p, 0, n);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_explicit_bzero(
  p: *mut c_void,
  n: size_t
) {
  string::rs_memset(p, 0, n);

  core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_ffs(mask: c_int) -> c_int {
  if mask == 0 { 0 } else { (mask.trailing_zeros() as c_int) + 1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_ffsl(mask: c_long) -> c_long {
  if mask == 0 { 0 } else { (mask.trailing_zeros() as c_long) + 1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_ffsll(mask: c_longlong) -> c_longlong {
  if mask == 0 { 0 } else { (mask.trailing_zeros() as c_longlong) + 1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_index(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  string::rs_strchr(s, c)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_rindex(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  string::rs_strrchr(s, c)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcasecmp(
  left: *const c_char,
  right: *const c_char
) -> c_int {
  rs_strcasecmp_l(left, right, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcasecmp_l(
  left: *const c_char,
  right: *const c_char,
  locale: locale_t<'static>
) -> c_int {
  if left.is_null() || right.is_null() {
    return 0;
  }

  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  // we do not care about malformed characters since there is no requirement
  // for that in POSIX aka UB
  let left =
    unsafe { str::from_utf8_unchecked(ffi::CStr::from_ptr(left).to_bytes()) };
  let right =
    unsafe { str::from_utf8_unchecked(ffi::CStr::from_ptr(right).to_bytes()) };

  let result =
    left.chars().into_iter().cmp_by(right.chars().into_iter(), |x, y| {
      (ctype.casemap.tolower)(x as u32).cmp(&(ctype.casemap.tolower)(y as u32))
    });

  match result {
    | Ordering::Less => -1,
    | Ordering::Equal => 0,
    | Ordering::Greater => 1
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strncasecmp(
  left: *const c_char,
  right: *const c_char,
  n: size_t
) -> c_int {
  rs_strncasecmp_l(left, right, n, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strncasecmp_l(
  left: *const c_char,
  right: *const c_char,
  n: size_t,
  locale: locale_t<'static>
) -> c_int {
  if left.is_null() || right.is_null() {
    return 0;
  }

  if n == 0 {
    return 0;
  }

  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  // we do not care about malformed characters since there is no requirement
  // for that in POSIX aka UB
  let left =
    unsafe { str::from_utf8_unchecked(ffi::CStr::from_ptr(left).to_bytes()) };
  let right =
    unsafe { str::from_utf8_unchecked(ffi::CStr::from_ptr(right).to_bytes()) };

  let result = left.chars().into_iter().take(n).cmp_by(
    right.chars().into_iter().take(n),
    |x, y| {
      (ctype.casemap.tolower)(x as u32).cmp(&(ctype.casemap.tolower)(y as u32))
    }
  );

  match result {
    | Ordering::Less => -1,
    | Ordering::Equal => 0,
    | Ordering::Greater => 1
  }
}
