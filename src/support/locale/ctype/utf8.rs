use {
  super::LCCtype,
  crate::{
    c_char,
    c_uchar,
    c_uint,
    char32_t,
    mbstate_t,
    size_t,
    ssize_t,
    std::errno,
    support::mbstate
  }
};

fn c32tomb(
  src: *mut c_char,
  c32: char32_t,
  _: *mut mbstate_t
) -> ssize_t {
  let mut s = src;
  unsafe {
    if c32 <= 0x7f {
      *s = c32 as c_char;
      return 1;
    } else if c32 <= 0x7ff {
      *s = 0xc0u8 as c_char | (c32.wrapping_shr(6)) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | (c32 & 0x3f) as c_char;
      return 2;
    } else if c32 <= 0xffff {
      if c32 >= 0xd800 && c32 <= 0xdfff {
        errno::set_errno(errno::EILSEQ);
        return -1;
      }
      *s = 0xe0u8 as c_char | (c32.wrapping_shr(12)) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | ((c32.wrapping_shr(6)) & 0x3f) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | (c32 & 0x3f) as c_char;
      return 3;
    } else if c32 <= 0x10ffff {
      *s = 0xf0u8 as c_char | (c32.wrapping_shr(18)) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | ((c32.wrapping_shr(12)) & 0x3f) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | ((c32.wrapping_shr(6)) & 0x3f) as c_char;
      s = s.wrapping_offset(1);
      *s = 0x80u8 as c_char | (c32 & 0x3f) as c_char;
      return 4;
    } else {
      errno::set_errno(errno::EILSEQ);
      return -1;
    }
  }
}

fn mbtoc32(
  pc32: *mut char32_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> ssize_t {
  let mut sb: *const c_uchar = s as *const c_uchar;
  let mut i = n;
  if i < 1 {
    return -2;
  }

  let mut bytesleft: c_uint = 0;
  let mut partial: char32_t = 0;
  let mut lowerbound: char32_t = 0;
  mbstate::mbstate_get_multibyte(
    ps,
    &mut bytesleft,
    &mut partial,
    &mut lowerbound
  );

  if bytesleft == 0 {
    unsafe {
      if (*sb & 0x80) == 0 {
        mbstate::mbstate_set_init(ps);
        if !pc32.is_null() {
          *pc32 = *sb as char32_t;
        }
        if *sb != b'\0' {
          return 1;
        } else {
          return 0;
        }
      } else if (*sb & 0xe0) == 0xc0 {
        bytesleft = 1;
        partial = *sb as char32_t & 0x1f;
        lowerbound = 0x80;
        sb = sb.offset(1);
      } else if (*sb & 0xf0) == 0xe0 {
        bytesleft = 2;
        partial = *sb as char32_t & 0xf;
        lowerbound = 0x800;
        sb = sb.offset(1);
      } else if (*sb & 0xf8) == 0xf0 {
        bytesleft = 3;
        partial = *sb as char32_t & 0x7;
        lowerbound = 0x10000;
        sb = sb.offset(1);
      } else {
        errno::set_errno(errno::EILSEQ);
        return -1;
      }
    }
    i = i.wrapping_sub(1);
  }

  while i > 0 {
    unsafe {
      if (*sb & 0xc0) != 0x80 {
        errno::set_errno(errno::EILSEQ);
        return -1;
      }
    }

    partial <<= 6;
    partial |= unsafe { *sb as char32_t & 0x3f };
    sb = sb.wrapping_offset(1);

    bytesleft = bytesleft.wrapping_sub(1);
    if bytesleft == 0 {
      if partial < lowerbound ||
        (partial >= 0xd800 && partial <= 0xdfff) ||
        partial > 0x10ffff
      {
        errno::set_errno(errno::EILSEQ);
        return -1;
      }
      unsafe {
        *pc32 = partial;
      }
      mbstate::mbstate_set_init(ps);
      return unsafe { sb.offset_from(s as *const c_uchar) };
    }

    i = i.wrapping_sub(1);
  }
  mbstate::mbstate_set_multibyte(ps, bytesleft, partial, lowerbound);
  -2
}

pub const CTYPE_UTF8: LCCtype = LCCtype {
  codeset: c"UTF-8",
  mbtoc32: mbtoc32,
  c32tomb: c32tomb,
  mb_cur_max: 4
};
