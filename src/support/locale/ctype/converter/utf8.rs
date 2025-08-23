use {
  super::ConverterObject,
  crate::{c_char, char32_t, mbstate_t, ssize_t, std::errno}
};

fn c32tomb(
  s: *mut c_char,
  c32: char32_t
) -> ssize_t {
  let mut s = s;
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
  pc32: &mut char32_t,
  s: &[u8],
  ps: &mut mbstate_t
) -> ssize_t {
  let mut n = s.len();
  let mut offset = 0;

  if n < 1 {
    return -2;
  }

  let mut bytesleft = ps.bytesleft;
  let mut partial = ps.partial;
  let mut lowerbound = ps.lowerbound;

  if bytesleft == 0 {
    if (s[offset] & 0x80) == 0 {
      *pc32 = s[offset] as char32_t;
      ps.reset();
      return 1;
    } else if (s[offset] & 0xe0) == 0xc0 {
      bytesleft = 1;
      partial = s[offset] as char32_t & 0x1f;
      lowerbound = 0x80;
      offset += 1;
    } else if (s[offset] & 0xf0) == 0xe0 {
      bytesleft = 2;
      partial = s[offset] as char32_t & 0xf;
      lowerbound = 0x800;
      offset += 1;
    } else if (s[offset] & 0xf8) == 0xf0 {
      bytesleft = 3;
      partial = s[offset] as char32_t & 0x7;
      lowerbound = 0x10000;
      offset += 1;
    } else {
      errno::set_errno(errno::EILSEQ);
      return -1;
    }

    n -= 1;
  }

  while n > 0 {
    if (s[offset] & 0xc0) != 0x80 {
      errno::set_errno(errno::EILSEQ);
      return -1;
    }

    partial <<= 6;
    partial |= s[offset] as char32_t & 0x3f;
    offset += 1;
    bytesleft -= 1;

    if bytesleft == 0 {
      if partial < lowerbound ||
        (partial >= 0xd800 && partial <= 0xdfff) ||
        partial > 0x10ffff
      {
        errno::set_errno(errno::EILSEQ);
        return -1;
      }

      *pc32 = partial;
      ps.reset();
      return offset as ssize_t;
    }

    n -= 1;
  }

  ps.bytesleft = bytesleft;
  ps.lowerbound = lowerbound;
  ps.partial = partial;

  -2
}

pub const CONVERTER_UTF8: ConverterObject = ConverterObject {
  codeset: c"UTF-8",
  mb_cur_max: 4,
  mbtoc32: mbtoc32,
  c32tomb: c32tomb
};
