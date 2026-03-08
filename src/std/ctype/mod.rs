use crate::{
  MBState,
  c_char,
  c_int,
  c_uchar,
  char32_t,
  locale_t,
  std::wctype,
  support::locale,
  wctrans_t,
  wctype_t,
  wint_t
};

#[inline]
fn inner_isctype(
  c: c_int,
  cc: wctype_t,
  locale: locale_t<'static>
) -> c_int {
  let locale_real: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale_real.ctype).unwrap_or_default();
  let mut ps = MBState::new();

  if c < 0 || c > c_uchar::MAX as c_int {
    return 0;
  }

  let c = c as c_char;
  let buf = [c as u8];
  let mut c32: char32_t = 0;

  if (ctype.converter.mbtoc32)(&mut c32, &buf, &mut ps) != 1 {
    return 0;
  }

  match cc {
    | wctype::WCTYPE_ALNUM => c_int::from((ctype.casemap.isalnum)(c32)),
    | wctype::WCTYPE_ALPHA => c_int::from((ctype.casemap.isalpha)(c32)),
    | wctype::WCTYPE_ASCII => c_int::from(wctype::inner_iswascii(c32)),
    | wctype::WCTYPE_BLANK => c_int::from((ctype.casemap.isblank)(c32)),
    | wctype::WCTYPE_CNTRL => c_int::from((ctype.casemap.iscntrl)(c32)),
    | wctype::WCTYPE_DIGIT => c_int::from((ctype.casemap.isdigit)(c32)),
    | wctype::WCTYPE_GRAPH => c_int::from((ctype.casemap.isgraph)(c32)),
    | wctype::WCTYPE_LOWER => c_int::from((ctype.casemap.islower)(c32)),
    | wctype::WCTYPE_PRINT => c_int::from((ctype.casemap.isprint)(c32)),
    | wctype::WCTYPE_PUNCT => c_int::from((ctype.casemap.ispunct)(c32)),
    | wctype::WCTYPE_SPACE => c_int::from((ctype.casemap.isspace)(c32)),
    | wctype::WCTYPE_UPPER => c_int::from((ctype.casemap.isupper)(c32)),
    | wctype::WCTYPE_XDIGIT => c_int::from((ctype.casemap.isxdigit)(c32)),
    | _ => 0
  }
}

#[inline]
fn inner_totrans(
  c: c_int,
  cc: wctrans_t,
  locale: locale_t<'static>
) -> c_int {
  let locale_real: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale_real.ctype).unwrap_or_default();
  let mut ps = MBState::new();

  if c < 0 || c > c_uchar::MAX as c_int {
    return c;
  }

  let c = c as c_char;
  let buf = [c as u8];
  let mut c32: char32_t = 0;

  if (ctype.converter.mbtoc32)(&mut c32, &buf, &mut ps) != 1 {
    return c as c_uchar as c_int;
  }

  match cc {
    | wctype::WCTRANS_TOASCII => (c32 as wint_t & 0x7F) as c_int,
    | wctype::WCTRANS_TOLOWER => (ctype.casemap.tolower)(c32) as c_int,
    | wctype::WCTRANS_TOUPPER => (ctype.casemap.toupper)(c32) as c_int,
    | _ => c as c_uchar as c_int
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isascii(wc: c_int) -> c_int {
  rs_isascii_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isascii_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_ASCII, locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isalnum(c: c_int) -> c_int {
  rs_isalnum_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isalnum_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_ALNUM, locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isalpha(c: c_int) -> c_int {
  rs_isalpha_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isalpha_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_ALPHA, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isblank(c: c_int) -> c_int {
  rs_isblank_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isblank_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_BLANK, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_iscntrl(c: c_int) -> c_int {
  rs_iscntrl_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iscntrl_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_CNTRL, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isdigit(c: c_int) -> c_int {
  rs_isdigit_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isdigit_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_DIGIT, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isgraph(c: c_int) -> c_int {
  rs_isgraph_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isgraph_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_GRAPH, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_islower(c: c_int) -> c_int {
  rs_islower_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_islower_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_LOWER, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isprint(c: c_int) -> c_int {
  rs_isprint_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isprint_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_PRINT, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_ispunct(c: c_int) -> c_int {
  rs_ispunct_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_ispunct_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_PUNCT, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isspace(c: c_int) -> c_int {
  rs_isspace_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isspace_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_SPACE, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isupper(c: c_int) -> c_int {
  rs_isupper_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isupper_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_UPPER, locale)
}
#[unsafe(no_mangle)]
pub extern "C" fn rs_isxdigit(c: c_int) -> c_int {
  rs_isxdigit_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_isxdigit_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_isctype(c, wctype::WCTYPE_XDIGIT, locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_toascii(c: c_int) -> c_int {
  rs_toascii_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_toascii_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_totrans(c, wctype::WCTRANS_TOASCII, locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tolower(c: c_int) -> c_int {
  rs_tolower_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tolower_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_totrans(c, wctype::WCTRANS_TOLOWER, locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_toupper(c: c_int) -> c_int {
  rs_toupper_l(c, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_toupper_l(
  c: c_int,
  locale: locale_t<'static>
) -> c_int {
  inner_totrans(c, wctype::WCTRANS_TOUPPER, locale)
}
