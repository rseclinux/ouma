use {
  crate::{
    MBState,
    c_char,
    c_int,
    c_long,
    c_longlong,
    char32_t,
    locale_t,
    size_t,
    std::string,
    support::{locale, locale::ctype::CtypeObject}
  },
  core::{ffi, ffi::c_void}
};

#[inline]
fn fetchchar(
  s: &mut &[u8],
  ctype: &CtypeObject,
  mb: &mut MBState
) -> char32_t {
  let mut c32: char32_t = 0;
  let ret = (ctype.converter.mbtoc32)(&mut c32, s, mb);
  if ret < 0 {
    return 0;
  }

  *s = &s[ret as usize..];

  c32
}

#[inline]
fn fetchchar_with_size(
  s: &mut &[u8],
  n: &mut size_t,
  ctype: &CtypeObject,
  mb: &mut MBState
) -> char32_t {
  let mut c32: char32_t = 0;
  let limit = s.len().min(*n as usize);

  let ret = (ctype.converter.mbtoc32)(&mut c32, &s[..limit], mb);
  if ret < 0 {
    return 0;
  }

  let ret = ret as usize;
  *s = &s[ret..];
  *n -= ret;

  c32
}

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

  let mut left = unsafe { ffi::CStr::from_ptr(left).to_bytes() };
  let mut right = unsafe { ffi::CStr::from_ptr(right).to_bytes() };

  let mut mbl = MBState::new();
  let mut mbr = MBState::new();

  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  loop {
    let l = (ctype.casemap.tolower)(fetchchar(&mut left, &ctype, &mut mbl));
    let r = (ctype.casemap.tolower)(fetchchar(&mut right, &ctype, &mut mbr));

    if l != r {
      return if l < r { -1 } else { 1 };
    }

    if l == 0 {
      return 0;
    }
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

  let mut left = unsafe { ffi::CStr::from_ptr(left).to_bytes() };
  let mut right = unsafe { ffi::CStr::from_ptr(right).to_bytes() };
  let mut nl = n;
  let mut nr = n;

  let mut mbl = MBState::new();
  let mut mbr = MBState::new();

  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  loop {
    let l = (ctype.casemap.tolower)(fetchchar_with_size(
      &mut left, &mut nl, &ctype, &mut mbl
    ));
    let r = (ctype.casemap.tolower)(fetchchar_with_size(
      &mut right, &mut nr, &ctype, &mut mbr
    ));

    if l != r {
      return if l < r { -1 } else { 1 };
    }

    if l == 0 {
      return 0;
    }
  }
}
