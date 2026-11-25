use {
  crate::{
    std::{errno, string, wchar},
    support::{
      locale,
      string::conversion::{StrToError, strtoint}
    },
    types::{c_char, c_int, intmax_t, locale_t, uintmax_t, wchar_t}
  },
  core::{ops::Neg, slice}
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct imaxdiv_t {
  pub quot: intmax_t,
  pub rem: intmax_t
}

#[unsafe(no_mangle)]
extern "C" fn rs_imaxdiv(
  num: intmax_t,
  denom: intmax_t
) -> imaxdiv_t {
  imaxdiv_t { quot: num / denom, rem: num % denom }
}

#[unsafe(no_mangle)]
extern "C" fn rs_imaxabs(num: intmax_t) -> intmax_t {
  return if num < 0 { num.neg() } else { num };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoimax_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> intmax_t {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  get_slot!(ctype, locale.ctype, locale::ctype::CtypeObject);

  let result: strtoint::StrToIntResult<intmax_t> =
    strtoint::strtoint(src, base, ctype);

  if let Some(e) = result.error {
    errno::set_errno(e.into());
  }

  if !endptr.is_null() {
    if result.error != Some(StrToError::InvalidNumber) {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoimax(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> intmax_t {
  rs_strtoimax_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoumax_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> uintmax_t {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  get_slot!(ctype, locale.ctype, locale::ctype::CtypeObject);

  let result: strtoint::StrToIntResult<uintmax_t> =
    strtoint::strtoint(src, base, ctype);

  if let Some(e) = result.error {
    errno::set_errno(e.into());
  }

  if !endptr.is_null() {
    if result.error != Some(StrToError::InvalidNumber) {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoumax(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> uintmax_t {
  rs_strtoumax_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoimax_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> intmax_t {
  let slen = wchar::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
  let locale = locale::get_real_locale(locale);
  get_slot!(ctype, locale.ctype, locale::ctype::CtypeObject);

  let result: strtoint::StrToIntResult<intmax_t> =
    strtoint::strtoint(src, base, ctype);

  if let Some(e) = result.error {
    errno::set_errno(e.into());
  }

  if !endptr.is_null() {
    if result.error != Some(StrToError::InvalidNumber) {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoimax(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> intmax_t {
  rs_wcstoimax_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoumax_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> uintmax_t {
  let slen = wchar::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
  let locale = locale::get_real_locale(locale);
  get_slot!(ctype, locale.ctype, locale::ctype::CtypeObject);

  let result: strtoint::StrToIntResult<uintmax_t> =
    strtoint::strtoint(src, base, ctype);

  if let Some(e) = result.error {
    errno::set_errno(e.into());
  }

  if !endptr.is_null() {
    if result.error != Some(StrToError::InvalidNumber) {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoumax(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> uintmax_t {
  rs_wcstoumax_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}
