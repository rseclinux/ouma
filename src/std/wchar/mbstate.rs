use {
  crate::{
    c_char,
    c_int,
    c_uchar,
    char32_t,
    mbstate_t,
    size_t,
    std::{stdio, stdlib},
    support::{locale, mbstate},
    wchar_t,
    wint_t
  },
  core::ptr
};

#[no_mangle]
pub extern "C" fn rs_btowc(c: c_int) -> wint_t {
  if c == stdio::constants::EOF {
    return super::constants::WEOF;
  }
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let buf: c_char = c as c_char;
  let mut c32: char32_t = 0;
  static mut PRIV: mbstate_t = mbstate_t::new();
  mbstate::mbstate_set_init(ptr::addr_of_mut!(PRIV));
  if (ctype.mbtoc32)(&mut c32, &buf, 1, ptr::addr_of_mut!(PRIV)) != 1 {
    return super::constants::WEOF;
  }
  c32 as wint_t
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
  let mut len = len;
  if dst.is_null() {
    let mut sb: *const char32_t = unsafe { *src as *const char32_t };
    let mut ret: size_t = 0;
    let mut nwc = nwc;
    while nwc != 0 {
      let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
        [0; stdlib::constants::MB_LEN_MAX as usize];
      let l =
        unsafe { (ctype.c32tomb)(buf.as_mut_ptr(), (**src) as char32_t, ps) };
      if l < 0 {
        return -1isize as size_t;
      }
      ret = ret.wrapping_add(l as usize);
      unsafe {
        let s1 = sb;
        sb = sb.offset(1);
        if *s1 == '\0' as char32_t {
          return ret.wrapping_sub(1);
        }
      }
      nwc -= 1;
    }
    return ret;
  } else {
    let mut db = dst;
    let mut i = 0;
    while i < nwc && len > 0 {
      let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
        [0; stdlib::constants::MB_LEN_MAX as usize];
      let l =
        unsafe { (ctype.c32tomb)(buf.as_mut_ptr(), (**src) as char32_t, ps) };
      if l < 0 {
        return -1isize as size_t;
      }
      if l as usize > len {
        return unsafe { db.offset_from(dst) as size_t };
      }
      let mut k: size_t = 0;
      while k < l as size_t {
        unsafe { *db.wrapping_offset(k as isize) = buf[k] };
        k = k.wrapping_add(1);
      }
      unsafe {
        let s1 = *src;
        *src = (*src).wrapping_offset(1);
        if *s1 == 0 {
          *src = ptr::null();
          return db.offset_from(dst) as size_t;
        }
      }
      db = db.wrapping_offset(l);
      len = len.wrapping_sub(l as usize);
      i += 1;
    }
    return unsafe { db.offset_from(dst) as size_t };
  }
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  if (ctype.c32tomb)(buf.as_ptr().cast_mut(), c, ptr::addr_of_mut!(PRIV)) != 1 {
    return stdio::constants::EOF;
  }
  unsafe { *buf.as_ptr() as c_uchar as c_int }
}
