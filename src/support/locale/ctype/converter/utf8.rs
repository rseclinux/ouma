use {
  super::ConverterObject,
  crate::{char32_t, mbstate_t, ssize_t, std::errno},
  icu_properties::{
    CodePointMapData,
    CodePointSetData,
    props::{
      DefaultIgnorableCodePoint,
      EastAsianWidth,
      Emoji,
      GeneralCategory,
      HangulSyllableType
    }
  }
};

fn c32tomb(
  s: &mut [u8],
  c32: char32_t
) -> ssize_t {
  if c32 <= 0x7f {
    s[0] = c32 as u8;
    return 1;
  } else if c32 <= 0x7ff {
    s[0] = 0xc0u8 | (c32.wrapping_shr(6)) as u8;
    s[1] = 0x80u8 | (c32 & 0x3f) as u8;
    return 2;
  } else if c32 <= 0xffff {
    if c32 >= 0xd800 && c32 <= 0xdfff {
      errno::set_errno(errno::EILSEQ);
      return -1;
    }
    s[0] = 0xe0u8 | (c32.wrapping_shr(12)) as u8;
    s[1] = 0x80u8 | ((c32.wrapping_shr(6)) & 0x3f) as u8;
    s[2] = 0x80u8 | (c32 & 0x3f) as u8;
    return 3;
  } else if c32 <= 0x10ffff {
    s[0] = 0xf0u8 | (c32.wrapping_shr(18)) as u8;
    s[1] = 0x80u8 | ((c32.wrapping_shr(12)) & 0x3f) as u8;
    s[2] = 0x80u8 | ((c32.wrapping_shr(6)) & 0x3f) as u8;
    s[3] = 0x80u8 | (c32 & 0x3f) as u8;
    return 4;
  } else {
    errno::set_errno(errno::EILSEQ);
    return -1;
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

fn wcwidth(c: u32) -> i32 {
  let Some(c) = char::from_u32(c) else {
    return -1;
  };

  if (c as u32) >= 0x20 && (c as u32) < 0x7f {
    return 1;
  }
  if (c as u32) == 0 {
    return 0;
  }
  if c < ' ' || ((c as u32) >= 0x7f && (c as u32) <= 0xa0) {
    return -1;
  }

  let gc = CodePointMapData::<GeneralCategory>::new();
  match gc.get(c) {
    | GeneralCategory::Control => return -1,
    | GeneralCategory::NonspacingMark | GeneralCategory::EnclosingMark => {
      return 0;
    },
    | GeneralCategory::Format => {
      if (c as u32) == 0x00ad {
        return 1;
      } else {
        return 0;
      }
    },
    | _ => ()
  }

  let hangul_jamo = CodePointMapData::<HangulSyllableType>::new();
  match hangul_jamo.get(c) {
    | HangulSyllableType::VowelJamo | HangulSyllableType::TrailingJamo => {
      return 1;
    },
    | HangulSyllableType::LeadingJamo |
    HangulSyllableType::LeadingVowelSyllable |
    HangulSyllableType::LeadingVowelTrailingSyllable => {
      return 2;
    },
    | _ => ()
  }

  if CodePointSetData::new::<DefaultIgnorableCodePoint>().contains(c) {
    return 0;
  }

  if (c as u32) >= 0x3248 && (c as u32) <= 0x4dff {
    if (c as u32) <= 0x324f {
      return 2;
    };
    if (c as u32) >= 0x4dc0 {
      return 2;
    };
  }

  let ea = CodePointMapData::<EastAsianWidth>::new();
  match ea.get(c) {
    | EastAsianWidth::Ambiguous |
    EastAsianWidth::Halfwidth |
    EastAsianWidth::Narrow |
    EastAsianWidth::Neutral => return 1,
    | EastAsianWidth::Fullwidth | EastAsianWidth::Wide => return 2,
    | _ => ()
  }

  if CodePointSetData::new::<Emoji>().contains(c) {
    return 2;
  }

  -1
}

pub const CONVERTER_UTF8: ConverterObject = ConverterObject {
  codeset: c"UTF-8",
  mb_cur_max: 4,
  mbtoc32: mbtoc32,
  c32tomb: c32tomb,
  wcwidth: wcwidth
};
