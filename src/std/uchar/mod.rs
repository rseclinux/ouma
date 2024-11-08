use {
  crate::{
    c_char,
    char8_t,
    char16_t,
    char32_t,
    mbstate_t,
    size_t,
    std::{errno, stdlib},
    support::{
      locale,
      manipulation::{bit, shift},
      mbstate
    }
  },
  core::ptr
};

const UTF8_ACCEPT: char8_t = 0;
const UTF8_REJECT: char8_t = 96;

const CLASSTAB: [char8_t; 256] = [
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8,
  8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
  9, 9, 9, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 11, 3, 3, 3, 3, 3, 3, 3, 3,
  3, 3, 3, 3, 4, 3, 3, 7, 6, 6, 6, 5, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8
];

const STATETAB: [char8_t; 108] = [
  0, 96, 12, 36, 48, 84, 72, 60, 96, 96, 96, 24, 96, 0, 96, 96, 96, 96, 96, 96,
  0, 0, 96, 96, 96, 12, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 12, 96, 96,
  96, 96, 96, 96, 12, 12, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 12, 12, 96,
  96, 96, 36, 96, 96, 96, 96, 96, 96, 96, 36, 96, 96, 96, 36, 96, 96, 96, 96,
  96, 96, 36, 36, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 36, 96, 96, 96, 96,
  96, 96, 96, 96, 96, 96, 96, 96, 96, 96, 96
];

#[inline(always)]
fn decode_step(
  state: char8_t,
  c8: char8_t,
  pc32: *mut char32_t
) -> char8_t {
  let class = CLASSTAB[c8 as usize];
  unsafe {
    *pc32 = if state == UTF8_ACCEPT {
      (c8 & (char8_t::from(0xff).wrapping_shr(class as u32))) as char32_t
    } else {
      (c8 & 0x3f) as char32_t | (*pc32 << 6)
    }
  }
  STATETAB[(state + class) as usize]
}

#[no_mangle]
pub extern "C" fn rs_c8rtomb(
  s: *mut c_char,
  c8: char8_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype = locale::get_thread_locale().ctype.expect("Malformed locale data");
  let mut buf: [c_char; stdlib::constants::MB_LEN_MAX as usize] =
    [0; stdlib::constants::MB_LEN_MAX as usize];
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
  let (s, c8) = if s.is_null() { (buf.as_mut_ptr(), 0) } else { (s, c8) };

  let mut c32: char32_t = unsafe {
    shift::shiftout((*ps).codeunit as usize, bit::bits(23, 0) as usize)
      as char32_t
  };
  let mut state: char8_t = unsafe {
    shift::shiftout((*ps).codeunit as usize, bit::bits(31, 24) as usize)
      as char8_t
  };
  state = decode_step(state, c8, &mut c32);
  match state {
    | UTF8_ACCEPT => {
      let l = (ctype.c32tomb)(s, c32, ps);
      if l >= 0 {
        mbstate::mbstate_set_init(ps);
      }
      return l as size_t;
    },
    | UTF8_REJECT => {
      errno::set_errno(errno::EILSEQ);
      return -1isize as size_t;
    },
    | _ => {
      unsafe {
        (*ps).codeunit =
          (shift::shiftin(state as usize, bit::bits(31, 24) as usize) |
            shift::shiftin(c32 as usize, bit::bits(23, 0) as usize))
            as char32_t
      };
      return 0;
    }
  };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
  let (pc8, s, n) = if s.is_null() {
    (&mut c8 as *mut char8_t, 0 as *const c_char, 1 as size_t)
  } else if pc8.is_null() {
    (&mut c8 as *mut char8_t, s, n)
  } else {
    (pc8, s, n)
  };
  unsafe {
    if ((*ps).count & 0x80000000) != 0 {
      let i: usize = (*ps).codeunits[3] as usize;
      if !pc8.is_null() {
        *pc8 = (*ps).codeunits[i];
      }
      if i == 0 {
        (*ps).count &= 0x7fffffff;
      } else {
        (*ps).codeunits[3] -= 1;
      }
      return -3isize as usize;
    }
  }
  let mut c32: char32_t = 0;
  let l = (ctype.mbtoc32)(&mut c32, s, n, ps);
  if l > 0 {
    if c32 <= 0x7f {
      if !pc8.is_null() {
        unsafe { *pc8 = c32 as char8_t };
      }
    } else if c32 <= 0x7ff {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xc0 + (c32.wrapping_shr(6) as char8_t & 0x1f) };
      }
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3f), 0);
      mbstate::mbstate_set_codeunit(ps, 0, 3);
      unsafe {
        (*ps).count |= 0x80000000;
      }
    } else if c32 <= 0xffff {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xe0 + (c32.wrapping_shr(12) as char8_t & 0x0f) };
      }
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(6) as char8_t & 0x3f),
        1
      );
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3f), 0);
      mbstate::mbstate_set_codeunit(ps, 1, 3);
      unsafe {
        (*ps).count |= 0x80000000;
      }
    } else if c32 <= 0x10ffff {
      if !pc8.is_null() {
        unsafe { *pc8 = 0xf0 + (c32.wrapping_shr(18) as char8_t & 0x07) };
      }
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(12) as char8_t & 0x3f),
        2
      );
      mbstate::mbstate_set_codeunit(
        ps,
        0x80 + (c32.wrapping_shr(6) as char8_t & 0x3f),
        1
      );
      mbstate::mbstate_set_codeunit(ps, 0x80 + (c32 as char8_t & 0x3f), 0);
      mbstate::mbstate_set_codeunit(ps, 2, 3);
      unsafe {
        (*ps).count |= 0x80000000;
      }
    } else {
      errno::set_errno(errno::EILSEQ);
      return -1isize as size_t;
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
  static mut PRIV: mbstate_t = mbstate_t::new();
  let ps =
    if !ps.is_null() { unsafe { &mut *ps } } else { ptr::addr_of_mut!(PRIV) };
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
