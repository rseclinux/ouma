use {
  crate::{
    c_int,
    c_long,
    c_longlong,
    c_ulong,
    c_ulonglong,
    locale_t,
    std::errno,
    support::{locale, string::conversion::strtoint},
    wchar_t
  },
  core::slice
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstol_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> c_long {
  let slen = super::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
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
pub extern "C" fn rs_wcstol(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> c_long {
  rs_wcstol_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoll_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> c_longlong {
  let slen = super::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
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
pub extern "C" fn rs_wcstoll(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> c_longlong {
  rs_wcstoll_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoul_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> c_ulong {
  let slen = super::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
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
pub extern "C" fn rs_wcstoul(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> c_ulong {
  rs_wcstoul_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstoull_l(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int,
  locale: locale_t<'static>
) -> c_ulonglong {
  let slen = super::rs_wcslen(nptr);
  let src = unsafe { slice::from_raw_parts(nptr as *const u32, slen) };
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
pub extern "C" fn rs_wcstoull(
  nptr: *const wchar_t,
  endptr: *mut *mut wchar_t,
  base: c_int
) -> c_ulonglong {
  rs_wcstoull_l(nptr, endptr, base, locale::get_thread_locale_ptr())
}
