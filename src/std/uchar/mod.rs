use {
  crate::{
    MBState,
    c_char,
    char8_t,
    char16_t,
    char32_t,
    mbstate_t,
    size_t,
    ssize_t,
    std::{errno, stdlib},
    support::locale
  },
  core::{cell::UnsafeCell, slice, str},
  critical_section::Mutex
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_c8rtomb(
  s: *mut c_char,
  c8: char8_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
    [0; stdlib::constants::MB_LEN_MAX];
  let (s, c8) = if s.is_null() {
    (buf.as_mut_slice(), 0)
  } else {
    unsafe {
      (slice::from_raw_parts_mut(s as *mut u8, ctype.converter.mb_cur_max), c8)
    }
  };

  if ps.u8_position == 0 {
    if (c8 >= 0x80 && c8 <= 0xc1) || c8 >= 0xf5 {
      errno::set_errno(errno::EILSEQ);
      return -1isize as size_t;
    }
    if c8 >= 0xc2 {
      ps.u8_position = 1;
      ps.u8_buffer[0] = c8;
      return 0;
    }

    ps.reset();
    (ctype.converter.c32tomb)(s, c8 as char32_t) as size_t
  } else {
    if ps.u8_position == 1 {
      if (c8 < 0x80 || c8 > 0xbf) ||
        (ps.u8_buffer[0] == 0xe0 && c8 < 0xa0) ||
        (ps.u8_buffer[0] == 0xed && c8 > 0x9f) ||
        (ps.u8_buffer[0] == 0xf0 && c8 < 0x90) ||
        (ps.u8_buffer[0] == 0xf4 && c8 > 0xbf)
      {
        errno::set_errno(errno::EILSEQ);
        return -1isize as size_t;
      }

      if ps.u8_buffer[0] >= 0xe0 {
        ps.u8_buffer[ps.u8_position] = c8;
        ps.u8_position += 1;
        return 0;
      }
    } else {
      if c8 < 0x80 || c8 > 0xbf {
        errno::set_errno(errno::EILSEQ);
        return -1isize as size_t;
      }

      if ps.u8_position == 2 && ps.u8_buffer[0] >= 0xf0 {
        ps.u8_buffer[ps.u8_position] = c8;
        ps.u8_position += 1;
        return 0;
      }
    }

    ps.u8_buffer[ps.u8_position] = c8;
    ps.u8_position += 1;

    match str::from_utf8(&ps.u8_buffer[..ps.u8_position]) {
      | Ok(decoded) => {
        if let Some(c32) = decoded.chars().next() {
          ps.reset();
          return (ctype.converter.c32tomb)(s, c32 as char32_t) as size_t;
        }
        decoded.len()
      },
      | Err(_) => {
        errno::set_errno(errno::EILSEQ);
        -1isize as size_t
      }
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_c16rtomb(
  s: *mut c_char,
  c16: char16_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
    [0; stdlib::constants::MB_LEN_MAX];
  let (s, c16) = if s.is_null() {
    (buf.as_mut_slice(), 0)
  } else {
    unsafe {
      (slice::from_raw_parts_mut(s as *mut u8, ctype.converter.mb_cur_max), c16)
    }
  };

  if ps.u16_surrogate != 0 {
    let units = [ps.u16_surrogate, c16];
    let mut decoder = char::decode_utf16(units.iter().copied());

    match decoder.next() {
      | Some(Ok(c)) => {
        ps.reset();
        return (ctype.converter.c32tomb)(s, c as char32_t) as size_t;
      },
      | _ => {
        errno::set_errno(errno::EILSEQ);
        return -1isize as size_t;
      }
    }
  } else {
    let units = [c16];
    let mut decoder = char::decode_utf16(units.iter().copied());

    if let Some(next) = decoder.next() {
      match next {
        | Ok(c) => {
          ps.reset();
          return (ctype.converter.c32tomb)(s, c as char32_t) as size_t;
        },
        | Err(e) => {
          if (0xd800..=0xdbff).contains(&e.unpaired_surrogate()) {
            ps.u16_surrogate = e.unpaired_surrogate();
            return 0;
          }
        },
      }
    }
  }

  errno::set_errno(errno::EILSEQ);
  -1isize as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_c32rtomb(
  s: *mut c_char,
  c32: char32_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
    [0; stdlib::constants::MB_LEN_MAX];
  let (s, c32) = if s.is_null() {
    (buf.as_mut_slice(), 0)
  } else {
    unsafe {
      (slice::from_raw_parts_mut(s as *mut u8, ctype.converter.mb_cur_max), c32)
    }
  };

  ps.reset();
  (ctype.converter.c32tomb)(s, c32) as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbrtoc8(
  pc8: *mut char8_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let rc8 = pc8;
  let mut c8: char8_t = 0;
  let (pc8, buffer): (&mut char8_t, &[u8]) = if s.is_null() {
    unsafe { (&mut *pc8, [0u8; 1].as_slice()) }
  } else if pc8.is_null() {
    unsafe { (&mut c8, core::slice::from_raw_parts(s as *const u8, n)) }
  } else {
    unsafe { (&mut *pc8, core::slice::from_raw_parts(s as *const u8, n)) }
  };

  if ps.u8_position != 0 {
    if !rc8.is_null() {
      let total = ps.u8_buffer.iter().position(|&b| b == 0).unwrap_or(4);
      let index = total - ps.u8_position;

      *pc8 = ps.u8_buffer[index];
    }
    ps.u8_position -= 1;
    return -3isize as usize;
  }

  let mut c32: char32_t = 0;
  let l: ssize_t = (ctype.converter.mbtoc32)(&mut c32, buffer, ps);
  if l >= 0 {
    match l {
      | 0 => {
        if !rc8.is_null() {
          *pc8 = 0;
        }
        return 0;
      },
      | -1 | -2 => return l as size_t,
      | _ => {}
    }

    let decoded = match char::from_u32(c32) {
      | Some(d) => d,
      | None => {
        errno::set_errno(errno::EILSEQ);
        return -1isize as usize;
      }
    };

    let mut buffer = [0u8; 4];
    let result = decoded.encode_utf8(&mut buffer).as_bytes();

    ps.u8_buffer[..result.len()].copy_from_slice(result);
    ps.u8_position = result.len() - 1;

    if !rc8.is_null() {
      *pc8 = ps.u8_buffer[0];
    }

    if *pc8 == b'\0' {
      return 0;
    }
  }

  l as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbrtoc16(
  pc16: *mut char16_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let rc16 = pc16;
  let mut c16: char16_t = 0;
  let (pc16, buffer): (&mut char16_t, &[u8]) = if s.is_null() {
    unsafe { (&mut *pc16, [0u8; 1].as_slice()) }
  } else if pc16.is_null() {
    unsafe { (&mut c16, core::slice::from_raw_parts(s as *const u8, n)) }
  } else {
    unsafe { (&mut *pc16, core::slice::from_raw_parts(s as *const u8, n)) }
  };

  if ps.u16_surrogate != 0 {
    if !rc16.is_null() {
      *pc16 = ps.u16_surrogate;
    }
    ps.u16_surrogate = 0;
    return -3isize as size_t;
  }

  let mut c32: char32_t = 0;
  let l: ssize_t = (ctype.converter.mbtoc32)(&mut c32, buffer, ps);
  if l >= 0 {
    match l {
      | 0 => {
        if !rc16.is_null() {
          *pc16 = 0;
        }
        return 0;
      },
      | -1 | -2 => return l as size_t,
      | _ => {}
    }

    let decoded = match char::from_u32(c32) {
      | Some(d) => d,
      | None => {
        errno::set_errno(errno::EILSEQ);
        return -1isize as usize;
      }
    };

    let mut buffer = [0u16; 16];
    let result = decoded.encode_utf16(&mut buffer);

    ps.u16_buffer[..result.len()].copy_from_slice(result);

    if result.len() == 2 {
      let leading = ps.u16_buffer[0];
      let trailing = ps.u16_buffer[1];

      ps.u16_surrogate = trailing;
      if !rc16.is_null() {
        *pc16 = leading;
      }
    } else {
      if !rc16.is_null() {
        *pc16 = ps.u16_buffer[0];
      }
    }

    if *pc16 == '\0' as char16_t {
      return 0;
    }
  }

  l as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_mbrtoc32(
  pc32: *mut char32_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> size_t {
  let ctype =
    locale::get_slot(&locale::get_thread_locale().ctype).unwrap_or_default();

  static GLOBAL: Mutex<UnsafeCell<MBState>> =
    Mutex::new(UnsafeCell::new(MBState::new()));
  let ps: &mut MBState = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    critical_section::with(|cs| {
      let cell = GLOBAL.borrow(cs);
      unsafe { &mut *cell.get() }
    })
  };

  let mut c32: char32_t = 0;
  let (pc32, buffer): (&mut char32_t, &[u8]) = if s.is_null() {
    unsafe { (&mut *pc32, [0u8; 1].as_slice()) }
  } else if pc32.is_null() {
    unsafe { (&mut c32, core::slice::from_raw_parts(s as *const u8, n)) }
  } else {
    unsafe { (&mut *pc32, core::slice::from_raw_parts(s as *const u8, n)) }
  };

  let l: ssize_t = (ctype.converter.mbtoc32)(pc32, buffer, ps);
  if l >= 0 && *pc32 == '\0' as char32_t {
    return 0;
  }
  l as size_t
}
