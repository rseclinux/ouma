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
    support::locale
  },
  core::{ffi, ffi::c_void}
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

  let mut left = unsafe { ffi::CStr::from_ptr(left).to_bytes_with_nul() };
  let mut right = unsafe { ffi::CStr::from_ptr(right).to_bytes_with_nul() };

  let mut d: c_int;

  loop {
    let mut ps1 = MBState::new();
    let mut ps2 = MBState::new();
    let mut c32_1: char32_t = 0;
    let mut c32_2: char32_t = 0;

    let n1 = (ctype.converter.mbtoc32)(&mut c32_1, left, &mut ps1);
    let n2 = (ctype.converter.mbtoc32)(&mut c32_2, right, &mut ps2);

    let (c1, adv1) = match n1 {
      | 0 => (0, 1),
      | 1.. => (c32_1 as c_int, n1 as usize),
      | _ => (left[0] as c_int, 1)
    };

    let (c2, adv2) = match n2 {
      | 0 => (0, 1),
      | 1.. => (c32_2 as c_int, n2 as usize),
      | _ => (right[0] as c_int, 1)
    };

    let c1 = (ctype.casemap.tolower)(c1 as u32) as c_int;
    let c2 = (ctype.casemap.tolower)(c2 as u32) as c_int;

    d = c1.wrapping_sub(c2);
    if d != 0 || c2 as u32 == '\0' as u32 {
      break;
    }

    left = &left[adv1..];
    right = &right[adv2..];
  }

  d
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

  let mut left = unsafe { ffi::CStr::from_ptr(left).to_bytes_with_nul() };
  let mut right = unsafe { ffi::CStr::from_ptr(right).to_bytes_with_nul() };

  let mut n = n;
  let mut d: c_int = 0;

  while n != 0 {
    let mut ps1 = MBState::new();
    let mut ps2 = MBState::new();
    let mut c32_1: char32_t = 0;
    let mut c32_2: char32_t = 0;

    let n1 = (ctype.converter.mbtoc32)(&mut c32_1, left, &mut ps1);
    let n2 = (ctype.converter.mbtoc32)(&mut c32_2, right, &mut ps2);

    let (c1, adv1) = match n1 {
      | 0 => (0, 1),
      | 1.. => (c32_1 as c_int, n1 as usize),
      | _ => (left[0] as c_int, 1)
    };

    let (c2, adv2) = match n2 {
      | 0 => (0, 1),
      | 1.. => (c32_2 as c_int, n2 as usize),
      | _ => (right[0] as c_int, 1)
    };

    let c1 = (ctype.casemap.tolower)(c1 as u32) as c_int;
    let c2 = (ctype.casemap.tolower)(c2 as u32) as c_int;

    d = c1.wrapping_sub(c2);
    if d != 0 || c2 as u32 == '\0' as u32 {
      break;
    }

    left = &left[adv1..];
    right = &right[adv2..];
    n -= 1;
  }

  d
}
