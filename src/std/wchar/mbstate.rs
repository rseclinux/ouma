use {
  crate::{
    MBState,
    MBStateLock,
    c_char,
    c_int,
    char32_t,
    mbstate_t,
    size_t,
    ssize_t,
    std::{stdio, stdlib, string, uchar},
    support::{locale, sync::SpinLock},
    wchar_t,
    wint_t
  },
  core::{ptr, slice}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_btowc(c: c_int) -> wint_t {
  static GLOBAL: SpinLock<MBState> = SpinLock::new(MBState::new());

  let mut ps = MBStateLock::Owned(GLOBAL.lock());
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  if c == stdio::constants::EOF {
    return super::constants::WEOF;
  }

  let buf: &[u8] = &[c as u8];
  let mut c32: char32_t = 0;

  if (ctype.converter.mbtoc32)(&mut c32, buf, &mut ps) != 1 {
    return super::constants::WEOF;
  }

  c32 as wint_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbrlen(
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  static GLOBAL: SpinLock<MBState> = SpinLock::new(MBState::new());
  let mut ps = if !ps.is_null() {
    MBStateLock::Borrowed(unsafe { &mut *ps })
  } else {
    MBStateLock::Owned(GLOBAL.lock())
  };
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();
  let s = unsafe { slice::from_raw_parts(s as *const u8, n) };
  let mut c32: char32_t = 0;

  let l = (ctype.converter.mbtoc32)(&mut c32, s, &mut ps);
  if l >= 0 && c32 == '\0' as char32_t {
    return 0;
  }
  l as usize
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbrtowc(
  pwc: *mut wchar_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  uchar::rs_mbrtoc32(pwc as *mut char32_t, s, n, ps)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbsinit(ps: *const mbstate_t) -> c_int {
  if ps.is_null() {
    c_int::from(true)
  } else {
    let ps = unsafe { *ps as MBState };
    c_int::from(ps.is_initial())
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbsrtowcs(
  dst: *mut wchar_t,
  src: *mut *const c_char,
  nms: size_t,
  ps: *mut mbstate_t
) -> size_t {
  rs_mbsnrtowcs(dst, src, size_t::MAX, nms, ps)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbsnrtowcs(
  dst: *mut wchar_t,
  src: *mut *const c_char,
  nmc: size_t,
  nms: size_t,
  ps: *mut mbstate_t
) -> size_t {
  static GLOBAL: SpinLock<MBState> = SpinLock::new(MBState::new());
  let mut ps = if !ps.is_null() {
    MBStateLock::Borrowed(unsafe { &mut *ps })
  } else {
    MBStateLock::Owned(GLOBAL.lock())
  };
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  let nmc = unsafe {
    let max = string::rs_strlen(*src) + 1;
    nmc.min(max)
  };

  let s = unsafe { slice::from_raw_parts(*src as *const u8, nmc) };
  let (mut s_converted, mut d_converted) = (0usize, 0usize);

  while s_converted < nmc && (dst.is_null() || d_converted < nms) {
    let (result, c32) = if dst.is_null() {
      let mut c32: char32_t = 0;
      let l =
        (ctype.converter.mbtoc32)(&mut c32, &s[s_converted..nmc], &mut ps);
      (l, c32)
    } else {
      let d = unsafe { slice::from_raw_parts_mut(dst as *mut u32, nms) };
      let l = (ctype.converter.mbtoc32)(
        &mut d[d_converted],
        &s[s_converted..nmc],
        &mut ps
      );
      (l, d[d_converted])
    };

    if result == -1 {
      if !dst.is_null() {
        unsafe { *src = s[s_converted..].as_ptr().cast() };
      }
      return -1isize as usize;
    }

    if result == -2 {
      if !dst.is_null() {
        unsafe { *src = s[nmc..].as_ptr().cast() };
      }
      break;
    }

    if c32 == '\0' as char32_t {
      if !dst.is_null() {
        unsafe { *src = ptr::null_mut() };
      }
      return d_converted;
    }

    d_converted += 1;
    s_converted += result as usize;
  }

  if !dst.is_null() {
    unsafe { *src = s[s_converted..].as_ptr().cast() };
  }

  d_converted
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcrtomb(
  s: *mut c_char,
  wc: wchar_t,
  ps: *mut mbstate_t
) -> size_t {
  uchar::rs_c32rtomb(s, wc as char32_t, ps)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsrtombs(
  dst: *mut c_char,
  src: *mut *const wchar_t,
  nms: size_t,
  ps: *mut mbstate_t
) -> size_t {
  rs_wcsnrtombs(dst, src, size_t::MAX, nms, ps)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsnrtombs(
  dst: *mut c_char,
  src: *mut *const wchar_t,
  nmc: size_t,
  nms: size_t,
  ps: *mut mbstate_t
) -> size_t {
  static GLOBAL: SpinLock<MBState> = SpinLock::new(MBState::new());
  let mut ps = if !ps.is_null() {
    MBStateLock::Borrowed(unsafe { &mut *ps })
  } else {
    MBStateLock::Owned(GLOBAL.lock())
  };
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  let nmc = unsafe {
    let max = super::rs_wcslen(*src) + 1;
    nmc.min(max)
  };

  let s = unsafe { slice::from_raw_parts(*src as *const u32, nmc) };
  let (mut s_converted, mut d_converted) = (0usize, 0usize);
  let mut result: ssize_t;

  while s_converted != nmc && (dst.is_null() || d_converted < nms) {
    let c32: char32_t = s[s_converted];
    let rem = nms.wrapping_sub(d_converted);

    result = if dst.is_null() {
      let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
        [0; stdlib::constants::MB_LEN_MAX];

      (ctype.converter.c32tomb)(buf.as_mut_slice(), c32)
    } else {
      let d = unsafe { slice::from_raw_parts_mut(dst as *mut u8, nms) };

      if rem > ctype.converter.mb_cur_max {
        (ctype.converter.c32tomb)(&mut d[d_converted..], c32)
      } else {
        let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
          [0; stdlib::constants::MB_LEN_MAX];

        let l = (ctype.converter.c32tomb)(buf.as_mut_slice(), c32);
        if l > 0 && l as usize > rem {
          ps.reset();
          break;
        }
        if l >= 0 {
          d[d_converted..].copy_from_slice(&buf[..l as usize]);
        }

        l
      }
    };

    if result < 0 {
      if !dst.is_null() {
        unsafe { *src = s[s_converted..].as_ptr().cast() };
      }
      return -1isize as size_t;
    }

    if c32 == '\0' as u32 {
      if !dst.is_null() {
        unsafe {
          *src = ptr::null();
        };
      }
      return d_converted;
    }

    d_converted += result as usize;
    s_converted += 1;
  }

  if !dst.is_null() {
    unsafe { *src = s[s_converted..].as_ptr().cast() };
  }

  d_converted
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wctob(c: wint_t) -> c_int {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();
  let mut buf = [0 as u8; stdlib::constants::MB_LEN_MAX];

  if (ctype.converter.c32tomb)(&mut buf, c as char32_t) != 1 {
    return stdio::constants::EOF;
  }

  buf[0] as c_int
}
