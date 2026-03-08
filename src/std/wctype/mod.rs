use {
  crate::{
    c_char,
    c_int,
    char32_t,
    locale_t,
    std::{errno, stdlib},
    support::{locale, locale::ctype::CtypeObject},
    wctrans_t,
    wctype_t,
    wint_t
  },
  core::ffi
};

pub const WCTYPE_ALNUM: wctype_t = 1;
pub const WCTYPE_ALPHA: wctype_t = 2;
pub const WCTYPE_ASCII: wctype_t = 13;
pub const WCTYPE_BLANK: wctype_t = 3;
pub const WCTYPE_CNTRL: wctype_t = 4;
pub const WCTYPE_DIGIT: wctype_t = 5;
pub const WCTYPE_GRAPH: wctype_t = 6;
pub const WCTYPE_LOWER: wctype_t = 7;
pub const WCTYPE_PRINT: wctype_t = 8;
pub const WCTYPE_PUNCT: wctype_t = 9;
pub const WCTYPE_SPACE: wctype_t = 10;
pub const WCTYPE_UPPER: wctype_t = 11;
pub const WCTYPE_XDIGIT: wctype_t = 12;

pub const WCTRANS_TOASCII: wctrans_t = 3 as wctrans_t;
pub const WCTRANS_TOLOWER: wctrans_t = 1 as wctrans_t;
pub const WCTRANS_TOUPPER: wctrans_t = 2 as wctrans_t;

#[inline]
pub fn inner_iswascii(wc: wint_t) -> bool {
  (wc & !0x7F) == 0
}

#[inline]
fn valid_in_locale<'a>(
  wc: wint_t,
  ctype: &'a CtypeObject<'a>
) -> bool {
  let mut buf: [u8; stdlib::constants::MB_LEN_MAX] =
    [0; stdlib::constants::MB_LEN_MAX];
  (ctype.converter.c32tomb)(&mut buf, wc as char32_t) != -1
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswascii(wc: wint_t) -> c_int {
  rs_iswascii_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswascii_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from(inner_iswascii(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswalnum(wc: wint_t) -> c_int {
  rs_iswalnum_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswalnum_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isalnum)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswalpha(wc: wint_t) -> c_int {
  rs_iswalpha_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswalpha_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isalpha)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswblank(wc: wint_t) -> c_int {
  rs_iswblank_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswblank_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isblank)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswcntrl(wc: wint_t) -> c_int {
  rs_iswcntrl_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswcntrl_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.iscntrl)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswctype(
  wc: wint_t,
  cc: wctype_t
) -> c_int {
  rs_iswctype_l(wc, cc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswctype_l(
  wc: wint_t,
  cc: wctype_t,
  locale: locale_t<'static>
) -> c_int {
  match cc {
    | WCTYPE_ALNUM => rs_iswalnum_l(wc, locale),
    | WCTYPE_ALPHA => rs_iswalpha_l(wc, locale),
    | WCTYPE_BLANK => rs_iswblank_l(wc, locale),
    | WCTYPE_CNTRL => rs_iswcntrl_l(wc, locale),
    | WCTYPE_DIGIT => rs_iswdigit_l(wc, locale),
    | WCTYPE_GRAPH => rs_iswgraph_l(wc, locale),
    | WCTYPE_LOWER => rs_iswlower_l(wc, locale),
    | WCTYPE_PRINT => rs_iswprint_l(wc, locale),
    | WCTYPE_PUNCT => rs_iswpunct_l(wc, locale),
    | WCTYPE_SPACE => rs_iswspace_l(wc, locale),
    | WCTYPE_UPPER => rs_iswupper_l(wc, locale),
    | WCTYPE_XDIGIT => rs_iswxdigit_l(wc, locale),
    | _ => 0
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswdigit(wc: wint_t) -> c_int {
  rs_iswdigit_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswdigit_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isdigit)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswgraph(wc: wint_t) -> c_int {
  rs_iswgraph_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswgraph_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isgraph)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswlower(wc: wint_t) -> c_int {
  rs_iswlower_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswlower_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.islower)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswprint(wc: wint_t) -> c_int {
  rs_iswprint_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswprint_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isprint)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswpunct(wc: wint_t) -> c_int {
  rs_iswpunct_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswpunct_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.ispunct)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswspace(wc: wint_t) -> c_int {
  rs_iswspace_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswspace_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isspace)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswupper(wc: wint_t) -> c_int {
  rs_iswupper_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswupper_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isupper)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswxdigit(wc: wint_t) -> c_int {
  rs_iswxdigit_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_iswxdigit_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  c_int::from((ctype.casemap.isxdigit)(wc) && valid_in_locale(wc, &ctype))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towctrans(
  wc: wint_t,
  desc: wctrans_t
) -> wint_t {
  rs_towctrans_l(wc, desc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towlower(wc: wint_t) -> c_int {
  rs_towlower_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towlower_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let nwc = (ctype.casemap.tolower)(wc) as c_int;
  if valid_in_locale(nwc as wint_t, &ctype) { nwc } else { wc as c_int }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towupper(wc: wint_t) -> c_int {
  rs_towupper_l(wc, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towupper_l(
  wc: wint_t,
  locale: locale_t<'static>
) -> c_int {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();

  let nwc = (ctype.casemap.toupper)(wc) as c_int;
  if valid_in_locale(nwc as wint_t, &ctype) { nwc } else { wc as c_int }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_towctrans_l(
  wc: wint_t,
  desc: wctrans_t,
  locale: locale_t<'static>
) -> wint_t {
  match desc {
    | WCTRANS_TOLOWER => rs_towlower_l(wc, locale) as wint_t,
    | WCTRANS_TOUPPER => rs_towupper_l(wc, locale) as wint_t,
    | _ => {
      errno::set_errno(errno::EINVAL);
      0
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wctrans(charclass: *const c_char) -> wctrans_t {
  let c = unsafe { ffi::CStr::from_ptr(charclass) };
  match c.to_bytes() {
    | b"tolower" => WCTRANS_TOLOWER,
    | b"toupper" => WCTRANS_TOUPPER,
    | _ => 0 as wctrans_t
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wctrans_l(
  charclass: *const c_char,
  _: locale_t<'static>
) -> wctrans_t {
  rs_wctrans(charclass)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wctype(property: *const c_char) -> wctype_t {
  let c = unsafe { ffi::CStr::from_ptr(property) };
  const PROPERTIES: [&'static ffi::CStr; 13] = [
    c"<invalid>",
    c"alnum",
    c"alpha",
    c"blank",
    c"cntrl",
    c"digit",
    c"graph",
    c"lower",
    c"print",
    c"punct",
    c"space",
    c"upper",
    c"xdigit"
  ];

  let mut i = 0;
  while i < PROPERTIES.len() {
    if PROPERTIES[i] == c {
      return i as wctype_t;
    }
    i += 1;
  }
  0
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wctype_l(
  property: *const c_char,
  _: locale_t<'static>
) -> wctype_t {
  rs_wctype(property)
}
