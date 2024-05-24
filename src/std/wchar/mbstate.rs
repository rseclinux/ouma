use {
  crate::{
    c_char,
    c_int,
    c_uchar,
    char32_t,
    mbstate_t,
    size_t,
    std::{stdio, stdlib, string},
    support::{locale, mbstate},
    wchar_t,
    wint_t
  },
  core::{ffi::c_void, ptr}
};

#[no_mangle]
pub extern "C" fn rs_btowc(c: c_int) -> wint_t {
  if c == stdio::constants::EOF {
    return super::constants::WEOF;
  }
  let buf = c as c_char;
  // TODO: mutex lock
  static mut PRIV: mbstate_t = mbstate_t::new();
  let mut wc: wchar_t = 0;
  let status = unsafe {
    rs_mbrtowc(&mut wc, &buf as *const c_char, 1, ptr::addr_of_mut!(PRIV))
  };
  if status == usize::max_value() || status == usize::max_value() - 1 {
    return super::constants::WEOF;
  }
  wc as wint_t
}

#[no_mangle]
pub extern "C" fn rs_mbrlen(
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let mut wc: wchar_t = 0;
  rs_mbrtowc(&mut wc, s, n, ps)
}

#[no_mangle]
pub extern "C" fn rs_mbrtowc(
  pwc: *mut wchar_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut wc: wchar_t = 0;
  let (pwc, s, n) = if s.is_null() {
    (&mut wc as *mut wchar_t, 0 as *const c_char, 1 as size_t)
  } else if pwc.is_null() {
    (&mut wc as *mut wchar_t, s, n)
  } else {
    (pwc, s, n)
  };
  let l = (ctype.mbtoc32)(pwc as *mut char32_t, s, n, ps);
  unsafe {
    if l >= 0 && *pwc == '\0' as wchar_t {
      return 0;
    }
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_mbsinit(ps: *const mbstate_t) -> c_int {
  c_int::from(mbstate::mbstate_get_init(ps))
}

#[no_mangle]
extern "C" fn rs_mbsnrtowcs(
  dst: *mut wchar_t,
  src: *mut *const c_char,
  nmc: size_t,
  len: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut sb: *const c_char = unsafe { *src };
  let mut nms = nmc;
  let mut i = len;
  if dst.is_null() {
    let mut ret = 0;
    loop {
      let mut c32: char32_t = 0;
      let l = (ctype.mbtoc32)(&mut c32, sb, nms, ps);
      match l {
        | -1 => {
          return -1isize as usize;
        },
        | -2 => {
          return ret;
        },
        | _ => {
          if c32 == 0 {
            return ret;
          }
          sb = sb.wrapping_add(l as usize);
          nms = nms.wrapping_sub(l as usize);
          ret += 1;
        }
      }
    }
  } else {
    let mut db = dst;
    while i > 0 {
      let l = (ctype.mbtoc32)(db as *mut char32_t, sb, nms, ps);
      match l {
        | -1 => {
          unsafe { *src = sb };
          return -1isize as usize;
        },
        | -2 => unsafe {
          *src = sb.wrapping_add(nms);
          return db.offset_from(dst) as size_t;
        },
        | _ => {
          unsafe {
            if *db == 0 {
              *src = ptr::null_mut();
              return db.offset_from(dst) as size_t;
            }
          }
          sb = sb.wrapping_add(l as usize);
          nms = nms.wrapping_sub(l as usize);
          db = db.wrapping_add(1);
        }
      }
      i -= 1;
    }
    unsafe {
      *src = sb;
      return db.offset_from(dst) as size_t;
    }
  }
}

#[no_mangle]
extern "C" fn rs_mbsrtowcs(
  dst: *mut wchar_t,
  src: *mut *const c_char,
  len: size_t,
  ps: *mut mbstate_t
) -> size_t {
  rs_mbsnrtowcs(dst, src, size_t::MAX, len, ps)
}

#[no_mangle]
pub extern "C" fn rs_wcrtomb(
  s: *mut c_char,
  wc: wchar_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  let (s, wc) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, wc) };
  let l = (ctype.c32tomb)(s, wc as char32_t, ps);
  if l >= 0 {
    mbstate::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_wcsnrtombs(
  dst: *mut c_char,
  src: *mut *const wchar_t,
  nwc: size_t,
  len: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let mut r: usize;
  let mut dsto: usize = 0;
  let mut srco: usize = 0;
  let s1 = unsafe { *src };
  let buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];

  if dst.is_null() {
    while srco < nwc {
      let wc: wchar_t = unsafe { *s1.wrapping_offset(dsto as isize) };

      if wc < 0x80 {
        if wc == 0 {
          return srco;
        }

        r = 1;
      } else {
        r = rs_wcrtomb(buf.as_ptr() as *mut c_char, wc, ps);
        if r == -1isize as usize {
          return r;
        }
      }

      dsto += 1;
      srco += r;
    }
  }

  while dsto < len && srco < nwc {
    let wc: wchar_t = unsafe { *s1.wrapping_offset(dsto as isize) };

    if wc < 0x80 {
      unsafe { *dst.wrapping_offset(srco as isize) = wc as c_char };

      if wc == 0 {
        unsafe { *src = ptr::null_mut() };
        return srco;
      }

      r = 1;
    } else if (len - srco) >= buf.len() {
      r = rs_wcrtomb(dst.wrapping_add(srco), wc, ps);
      if r == -1isize as usize {
        unsafe {
          *src = s1.wrapping_add(srco);
        }
        return r;
      }
    } else {
      r = rs_wcrtomb(buf.as_ptr() as *mut c_char, wc, ps);
      if r == -1isize as usize {
        unsafe {
          *src = s1.wrapping_add(srco);
        }
        return r;
      }
      if r > len - srco {
        break;
      }

      string::rs_memcpy(
        dst.wrapping_add(srco) as *mut c_void,
        buf.as_ptr() as *const c_void,
        r
      );
    }

    dsto += 1;
    srco += r;
  }

  unsafe {
    *src = s1.wrapping_add(srco);
  }
  dsto
}

#[no_mangle]
pub extern "C" fn rs_wcsrtombs(
  dst: *mut c_char,
  src: *mut *const wchar_t,
  len: size_t,
  ps: *mut mbstate_t
) -> size_t {
  rs_wcsnrtombs(dst, src, size_t::MAX, len, ps)
}

#[no_mangle]
pub extern "C" fn rs_wctob(c: wint_t) -> c_int {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  // TODO: mutex lock
  static mut PRIV: mbstate_t = mbstate_t::new();
  unsafe {
    if (ctype.c32tomb)(buf.as_ptr().cast_mut(), c, ptr::addr_of_mut!(PRIV)) != 1
    {
      return stdio::constants::EOF;
    }
  }
  unsafe { *buf.as_ptr() as c_uchar as c_int }
}
