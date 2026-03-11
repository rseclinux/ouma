use {
  crate::{
    MBState,
    c_char,
    c_int,
    char32_t,
    locale_t,
    size_t,
    support::{locale, locale::ctype::CtypeObject}
  },
  core::ffi
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
