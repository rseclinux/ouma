use crate::{
  c_char,
  char8_t,
  char16_t,
  char32_t,
  mbstate_t,
  size_t,
  std::{errno, stdlib},
  support::{locale, mbstate}
};

#[no_mangle]
pub extern "C" fn rs_c8rtomb(
  s: *mut c_char,
  c8: char8_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  let (s, c8) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c8) };

  let c32: char32_t;
  unsafe {
    if ((*ps).count & 0xff00) != 0xc800 {
      if (c8 >= 0x80 && c8 <= 0xc1) || c8 >= 0xf5 {
        (*ps).count = 0;
        errno::set_errno(errno::EILSEQ);
        return -1isize as usize;
      }
      if c8 >= 0xc2 {
        (*ps).count = 0xc801;
        mbstate::mbstate_set_codeunit(ps, c8, 0);
        mbstate::mbstate_set_codeunit(ps, 1, 3);
        return 0;
      }
      (*ps).count = 0;
      c32 = c8 as char32_t;
    } else {
      let cu1 = mbstate::mbstate_get_codeunit(ps, 0);
      if mbstate::mbstate_get_codeunit(ps, 3) == 1 {
        if (c8 < 0x80 || c8 > 0xbf) ||
          (cu1 == 0xe0 && c8 < 0xa0) ||
          (cu1 == 0xed && c8 > 0x9f) ||
          (cu1 == 0xf0 && c8 < 0x90) ||
          (cu1 == 0xf4 && c8 > 0x8f)
        {
          (*ps).count = 0;
          errno::set_errno(errno::EILSEQ);
          return -1isize as usize;
        }
        if cu1 >= 0xe0 {
          (*ps).count = 0xc802;
          mbstate::mbstate_set_codeunit(ps, c8, 1);
          mbstate::mbstate_set_codeunit(
            ps,
            mbstate::mbstate_get_codeunit(ps, 3) + 1,
            3
          );
          return 0;
        }
        c32 = (((cu1 & 0x1f).wrapping_shl(6)) + (c8 & 0x3f)) as char32_t;
      } else {
        let cu2 = mbstate::mbstate_get_codeunit(ps, 1);
        if c8 < 0x80 || c8 > 0xbf {
          (*ps).count = 0;
          errno::set_errno(errno::EILSEQ);
          return -1isize as usize;
        }
        if mbstate::mbstate_get_codeunit(ps, 3) == 2 && cu1 >= 0xf0 {
          (*ps).count = 0xc803;
          mbstate::mbstate_set_codeunit(ps, c8, 2);
          mbstate::mbstate_set_codeunit(
            ps,
            mbstate::mbstate_get_codeunit(ps, 3) + 1,
            3
          );
          return 0;
        }
        if cu1 < 0xf0 {
          c32 = (((cu1 & 0x0f).wrapping_shl(12)) +
            ((cu2 & 0x3f).wrapping_shl(6)) +
            (c8 & 0x3f)) as char32_t;
        } else {
          let cu3 = mbstate::mbstate_get_codeunit(ps, 2);
          c32 = (((cu1 & 0x07).wrapping_shl(18)) +
            ((cu2 & 0x3f).wrapping_shl(12)) +
            ((cu3 & 0x3f).wrapping_shl(6)) +
            (c8 & 0x3f)) as char32_t;
        }
      }
    }
  }

  let l = (ctype.c32tomb)(s, c32, ps);
  if l >= 0 {
    mbstate::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_c16rtomb(
  s: *mut c_char,
  c16: char16_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  let (s, c16) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c16) };

  let c32: char32_t;
  let mut c16l: char16_t = 0;
  if mbstate::mbstate_get_surrogate(ps, &mut c16l) {
    if c16 < 0xdc00 || c16 > 0xdfff {
      errno::set_errno(errno::EILSEQ);
      return -1isize as usize;
    }
    c32 = 0x10000 + ((c16l & 0x3ff) << 10 | (c16 & 0x3ff)) as char32_t;
  } else if c16 >= 0xd800 && c16 <= 0xdbff {
    mbstate::mbstate_set_surrogate(ps, c16);
    return 0;
  } else {
    c32 = c16 as char32_t;
  }

  let l = (ctype.c32tomb)(s, c32, ps);
  if l >= 0 {
    mbstate::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_c32rtomb(
  s: *mut c_char,
  c32: char32_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  let (s, c32) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c32) };
  let l = (ctype.c32tomb)(s, c32, ps);
  if l >= 0 {
    mbstate::mbstate_set_init(ps);
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_mbrtoc8(
  pc8: *mut char8_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut c8: char8_t = 0;
  let (pc8, s, n) = if s.is_null() {
    (&mut c8 as *mut char8_t, 0 as *const c_char, 1 as size_t)
  } else if pc8.is_null() {
    (&mut c8 as *mut char8_t, s, n)
  } else {
    (pc8, s, n)
  };
  unsafe {
    if ((*ps).count & 0xff00) == 0xc800 {
      let i: usize = (*ps).codeunits[3] as usize;
      if !pc8.is_null() {
        *pc8 = (*ps).codeunits[i];
      }
      if i == 0 {
        (*ps).count = 0;
      } else {
        (*ps).codeunits[3] -= 1;
      }
      return -3isize as usize;
    }
  }
  let mut c32: char32_t = 0;
  let l = (ctype.mbtoc32)(&mut c32, s, n, ps);
  if l > 0 {
    if c32 <= 0x7F {
      if !pc8.is_null() {
        unsafe { *pc8 = c32 as char8_t };
      }
      unsafe { (*ps).count = 0 };
    } else if c32 <= 0x7FF {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xC0 + (c32.wrapping_shr(6) as char8_t & 0x1F) };
      }
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3F), 0);
      mbstate::mbstate_set_codeunit(ps, 0, 3);
      unsafe { (*ps).count = 0xc800 | 1 };
    } else if c32 <= 0xFFFF {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xE0 + (c32.wrapping_shr(12) as char8_t & 0x0F) };
      }
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(6) as char8_t & 0x3F),
        1
      );
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3F), 0);
      mbstate::mbstate_set_codeunit(ps, 1, 3);
      unsafe { (*ps).count = 0xc800 | 2 };
    } else if c32 <= 0x10FFFF {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xF0 + (c32.wrapping_shr(18) as char8_t & 0x07) };
      }
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(12) as char8_t & 0x3F),
        2
      );
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(6) as char8_t & 0x3F),
        1
      );
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3F), 0);
      mbstate::mbstate_set_codeunit(ps, 2, 3);
      unsafe { (*ps).count = 0xc800 | 3 };
    } else {
      unsafe { (*ps).count = 0 };
      errno::set_errno(errno::EILSEQ);
      return -1isize as usize;
    }
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_mbrtoc16(
  pc16: *mut char16_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut c16: char16_t = 0;
  let (pc16, s, n) = if s.is_null() {
    (&mut c16 as *mut char16_t, 0 as *const c_char, 1 as size_t)
  } else if pc16.is_null() {
    (&mut c16 as *mut char16_t, s, n)
  } else {
    (pc16, s, n)
  };
  if mbstate::mbstate_get_surrogate(ps, pc16) == true {
    mbstate::mbstate_set_init(ps);
    return -3isize as usize;
  }
  if n == 0 {
    mbstate::mbstate_set_init(ps);
    return -2isize as usize;
  }
  let mut c32: char32_t = 0;
  let l = (ctype.mbtoc32)(&mut c32, s, n, ps);
  if l >= 0 {
    if c32 < 0x10000 {
      unsafe { *pc16 = c32 as char16_t };
    } else {
      c32 -= 0x10000;
      unsafe { *pc16 = 0xd800 | (c32 >> 10) as char16_t };
      mbstate::mbstate_set_surrogate(ps, 0xdc00 | (c32 & 0x3ff) as char16_t);
    }
    unsafe {
      if *pc16 == 0 {
        return 0;
      }
    }
  }
  l as size_t
}

#[no_mangle]
pub extern "C" fn rs_mbrtoc32(
  pc32: *mut char32_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut c32: char32_t = 0;
  let (pc32, s, n) = if s.is_null() {
    (&mut c32 as *mut char32_t, 0 as *const c_char, 1 as size_t)
  } else if pc32.is_null() {
    (&mut c32 as *mut char32_t, s, n)
  } else {
    (pc32, s, n)
  };
  let l = (ctype.mbtoc32)(pc32, s, n, ps);
  unsafe {
    if l >= 0 && *pc32 == '\0' as char32_t {
      return 0;
    }
  }
  l as size_t
}
