use {
  super::EncodedLDBLReturn,
  crate::{
    c_char,
    c_double,
    c_float,
    c_int,
    c_long,
    c_longdouble,
    c_longlong,
    c_ulong,
    c_ulonglong,
    locale_t,
    std::{errno, string},
    support::{
      locale,
      string::conversion::{strtofloat, strtoint}
    }
  },
  core::{ptr, slice}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtol_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> c_long {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let result: strtoint::StrToIntResult<c_long> =
    strtoint::strtoint(src, base, &ctype);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtol(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> c_long {
  rs_strtol_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoll_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> c_longlong {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let result: strtoint::StrToIntResult<c_longlong> =
    strtoint::strtoint(src, base, &ctype);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoll(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> c_longlong {
  rs_strtoll_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoul_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> c_ulong {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let result: strtoint::StrToIntResult<c_ulong> =
    strtoint::strtoint(src, base, &ctype);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoul(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> c_ulong {
  rs_strtoul_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoull_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int,
  locale: locale_t<'static>
) -> c_ulonglong {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let result: strtoint::StrToIntResult<c_ulonglong> =
    strtoint::strtoint(src, base, &ctype);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtoull(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  base: c_int
) -> c_ulonglong {
  rs_strtoull_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtof_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  locale: locale_t<'static>
) -> c_float {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);

  let result: strtofloat::StrToFloatResult<c_float> =
    strtofloat::strtofloat(src, &locale);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtof(
  nptr: *const c_char,
  endptr: *mut *mut c_char
) -> c_float {
  rs_strtof_l(nptr, endptr, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtod_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  locale: locale_t<'static>
) -> c_double {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);

  let result: strtofloat::StrToFloatResult<c_double> =
    strtofloat::strtofloat(src, &locale);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  result.value
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtod(
  nptr: *const c_char,
  endptr: *mut *mut c_char
) -> c_double {
  rs_strtod_l(nptr, endptr, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
extern "C" fn __oumainternal_strtofloatenc_l(
  nptr: *const c_char,
  endptr: *mut *mut c_char,
  locale: locale_t<'static>
) -> EncodedLDBLReturn {
  let slen = string::rs_strlen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u8, slen) };
  let locale = locale::get_real_locale(locale);

  let result: strtofloat::StrToFloatResult<c_longdouble> =
    strtofloat::strtofloat(src, &locale);

  if result.error != 0 {
    errno::set_errno(result.error);
  }

  if !endptr.is_null() {
    if result.error != errno::EINVAL {
      unsafe { *endptr = nptr.offset(result.len as isize).cast_mut() };
    } else {
      unsafe { *endptr = nptr.cast_mut() };
    }
  }

  let enc = EncodedLDBLReturn { bytes: result.value.to_ne_bytes() };
  enc
}

#[unsafe(no_mangle)]
extern "C" fn __oumainternal_strtofloatenc(
  nptr: *const c_char,
  endptr: *mut *mut c_char
) -> EncodedLDBLReturn {
  __oumainternal_strtofloatenc_l(nptr, endptr, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_atof(s: *const c_char) -> c_double {
  rs_strtod(s, ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_atoi(s: *const c_char) -> c_int {
  rs_strtol(s, ptr::null_mut(), 10) as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_atol(s: *const c_char) -> c_long {
  rs_strtol(s, ptr::null_mut(), 10)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_atoll(s: *const c_char) -> c_longlong {
  rs_strtoll(s, ptr::null_mut(), 10)
}
