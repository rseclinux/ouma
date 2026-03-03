use {
  crate::{
    allocation::{
      borrow::{Cow, ToOwned},
      boxed::Box
    },
    c_char,
    c_int,
    intptr_t,
    locale_t,
    std::errno,
    support::locale
  },
  atomic_refcell::AtomicRefCell,
  core::{ffi, ptr},
  smallvec::SmallVec
};

mod available;

pub const LC_CTYPE: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 2;
pub const LC_COLLATE: c_int = 3;
pub const LC_MONETARY: c_int = 4;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = 6;

pub const LC_CTYPE_MASK: c_int = 1 << LC_CTYPE;
pub const LC_NUMERIC_MASK: c_int = 1 << LC_NUMERIC;
pub const LC_TIME_MASK: c_int = 1 << LC_TIME;
pub const LC_COLLATE_MASK: c_int = 1 << LC_COLLATE;
pub const LC_MONETARY_MASK: c_int = 1 << LC_MONETARY;
pub const LC_MESSAGES_MASK: c_int = 1 << LC_MESSAGES;

pub const LC_ALL_MASK: c_int = LC_CTYPE_MASK |
  LC_NUMERIC_MASK |
  LC_TIME_MASK |
  LC_COLLATE_MASK |
  LC_MONETARY_MASK |
  LC_MESSAGES_MASK;

pub const LC_GLOBAL_LOCALE: locale_t = -1 as intptr_t as locale_t;

#[repr(C)]
pub struct lconv {
  pub decimal_point: *mut c_char,
  pub thousands_sep: *mut c_char,
  pub grouping: *mut c_char,
  pub int_curr_symbol: *mut c_char,
  pub currency_symbol: *mut c_char,
  pub mon_decimal_point: *mut c_char,
  pub mon_thousands_sep: *mut c_char,
  pub mon_grouping: *mut c_char,
  pub positive_sign: *mut c_char,
  pub negative_sign: *mut c_char,
  pub int_frac_digits: c_char,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_cs_precedes: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_p_cs_precedes: c_char,
  pub int_p_sep_by_space: c_char,
  pub int_n_cs_precedes: c_char,
  pub int_n_sep_by_space: c_char,
  pub int_p_sign_posn: c_char,
  pub int_n_sign_posn: c_char
}

unsafe impl Send for lconv {}
unsafe impl Sync for lconv {}

impl lconv {
  pub fn from_locale(locale: &locale::Locale<'static>) -> Self {
    let monetary = locale::get_slot(&locale.monetary);
    let numeric = locale::get_slot(&locale.numeric);

    let decimal_point: *mut c_char =
      numeric.decimal_point.as_ptr() as *mut u8 as *mut c_char;
    let thousands_sep: *mut c_char =
      numeric.thousands_sep.as_ptr() as *mut u8 as *mut c_char;
    let grouping: *mut c_char =
      numeric.grouping.as_ptr() as *mut u8 as *mut c_char;

    let int_curr_symbol: *mut c_char =
      monetary.int_curr_symbol.as_ptr() as *mut u8 as *mut c_char;
    let currency_symbol: *mut c_char =
      monetary.currency_symbol.as_ptr() as *mut u8 as *mut c_char;
    let mon_decimal_point: *mut c_char =
      monetary.mon_decimal_point.as_ptr() as *mut u8 as *mut c_char;
    let mon_thousands_sep: *mut c_char =
      monetary.mon_thousands_sep.as_ptr() as *mut u8 as *mut c_char;
    let mon_grouping: *mut c_char =
      monetary.mon_grouping.as_ptr() as *mut u8 as *mut c_char;
    let positive_sign: *mut c_char =
      monetary.positive_sign.as_ptr() as *mut u8 as *mut c_char;
    let negative_sign: *mut c_char =
      monetary.negative_sign.as_ptr() as *mut u8 as *mut c_char;

    Self {
      decimal_point: decimal_point,
      thousands_sep: thousands_sep,
      grouping: grouping,
      int_curr_symbol: int_curr_symbol,
      currency_symbol: currency_symbol,
      mon_decimal_point: mon_decimal_point,
      mon_thousands_sep: mon_thousands_sep,
      mon_grouping: mon_grouping,
      positive_sign: positive_sign,
      negative_sign: negative_sign,
      int_frac_digits: monetary.int_frac_digits,
      frac_digits: monetary.frac_digits,
      p_cs_precedes: monetary.p_cs_precedes,
      p_sep_by_space: monetary.p_sep_by_space,
      n_cs_precedes: monetary.n_cs_precedes,
      n_sep_by_space: monetary.n_sep_by_space,
      p_sign_posn: monetary.p_sign_posn,
      n_sign_posn: monetary.n_sign_posn,
      int_p_cs_precedes: monetary.int_p_cs_precedes,
      int_p_sep_by_space: monetary.int_p_sep_by_space,
      int_n_cs_precedes: monetary.int_n_cs_precedes,
      int_n_sep_by_space: monetary.int_n_sep_by_space,
      int_p_sign_posn: monetary.int_p_sign_posn,
      int_n_sign_posn: monetary.int_n_sign_posn
    }
  }
}

fn normalize_locale_name<'a>(name: &'a ffi::CStr) -> Cow<'a, ffi::CStr> {
  let bytes = name.to_bytes();

  let Some(dot) = bytes.iter().position(|&b| b == b'.') else {
    return Cow::Borrowed(name);
  };

  let codeset_start = dot + 1;

  let at = bytes[codeset_start..]
    .iter()
    .position(|&b| b == b'@')
    .map(|i| codeset_start + i)
    .unwrap_or(bytes.len());

  let codeset = &bytes[codeset_start..at];
  let modifier = &bytes[at..];

  let canonical: Option<&[u8]> = match codeset {
    | b"UTF-8" => None,
    | b"ASCII" => None,
    | b"utf-8" | b"utf8" | b"UTF8" => Some(b"UTF-8"),
    | b"ascii" | b"us-ascii" | b"US-ascii" | b"US-ASCII" => Some(b"ASCII"),
    | _ => None
  };

  let canonical = match canonical {
    | None => return Cow::Borrowed(name),
    | Some(c) => c
  };

  // TODO: replace 255 with NL_TEXTMAX
  let mut buf = SmallVec::<[u8; 255]>::new();

  buf.extend_from_slice(&bytes[..=dot]);
  buf.extend_from_slice(canonical);
  buf.extend_from_slice(modifier);
  buf.push(b'\0');

  let cstr = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(&buf) };

  Cow::Owned(cstr.to_owned())
}

#[inline]
fn swap<T>(
  lhs: &AtomicRefCell<T>,
  rhs: &AtomicRefCell<T>
) {
  let (first, second) =
    if (lhs as *const _) < (rhs as *const _) { (lhs, rhs) } else { (rhs, lhs) };
  let mut g1 = first.borrow_mut();
  let mut g2 = second.borrow_mut();
  core::mem::swap(&mut *g1, &mut *g2);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_localeconv() -> *mut lconv {
  let locale = locale::get_thread_locale_ptr();
  rs_localeconv_l(locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_localeconv_l(locale: locale_t<'static>) -> *mut lconv {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  let lconv = lconv::from_locale(&locale);

  let result = locale.localeconv.get();
  unsafe { core::ptr::write(result, lconv) };

  result
}

#[unsafe(no_mangle)]
extern "C" fn rs_setlocale(
  category: c_int,
  locale: *const c_char
) -> *mut c_char {
  let mut locales: [Option<&'static ffi::CStr>; 7] = [
    None, // LC_CTYPE
    None, // LC_NUMERIC
    None, // LC_TIME
    None, // LC_COLLATE
    None, // LC_MONETARY
    None, // LC_MESSAGES
    None  // LC_ALL
  ];

  if category < 0 || category > LC_ALL {
    return ptr::null_mut();
  }
  if locale.is_null() {
    return locale::get_thread_locale().querylocale(category);
  }

  let locale = unsafe { ffi::CStr::from_ptr(locale) };
  if locale.is_empty() {
    locales[category as usize] = Some(c"C"); // TODO: get from environment
  } else {
    locales[category as usize] = Some(locale);
  }

  for (c, lc) in locales.iter().enumerate() {
    if let Some(l) = lc {
      let l: &ffi::CStr = &normalize_locale_name(l);

      if !available::AVAILABLE_LOCALES.contains(&&l) {
        return ptr::null_mut();
      }

      let changed = locale::get_thread_locale();
      if let Ok(result) = changed.setlocale(c as c_int, l) {
        return result.querylocale(c as c_int);
      }
    }
  }

  ptr::null_mut()
}

fn newlocale_inner(
  mask: c_int,
  name: *const c_char,
  base: locale_t<'static>
) -> Result<locale_t<'static>, c_int> {
  if mask == 0 || name.is_null() {
    return Err(errno::EINVAL);
  }

  let name = unsafe { ffi::CStr::from_ptr(name) };

  let env: Option<&'static ffi::CStr> = Some(c"C"); // TODO: get from environment

  let name = if name.is_empty() {
    if let Some(e) = env { e } else { c"C" }
  } else {
    name
  };

  let base = if base.is_null() {
    locale::get_thread_locale()
  } else {
    locale::get_real_locale(base)
  };

  let newloc =
    Box::try_new(locale::Locale::new()).map_err(|_| errno::ENOENT)?;

  locale::set_slot(&newloc.collate, name)?;
  locale::set_slot(&newloc.ctype, name)?;
  locale::set_slot(&newloc.messages, name)?;
  locale::set_slot(&newloc.monetary, name)?;
  locale::set_slot(&newloc.numeric, name)?;
  locale::set_slot(&newloc.time, name)?;

  if (mask & LC_COLLATE_MASK) == 0 {
    swap(&newloc.collate, &base.collate);
  }
  if (mask & LC_CTYPE_MASK) == 0 {
    swap(&newloc.ctype, &base.ctype);
  }
  if (mask & LC_MESSAGES_MASK) == 0 {
    swap(&newloc.messages, &base.messages);
  }
  if (mask & LC_MONETARY_MASK) == 0 {
    swap(&newloc.monetary, &base.monetary);
  }
  if (mask & LC_NUMERIC_MASK) == 0 {
    swap(&newloc.numeric, &base.numeric);
  }
  if (mask & LC_TIME_MASK) == 0 {
    swap(&newloc.time, &base.time);
  }

  Ok(Box::into_raw(newloc))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_newlocale(
  mask: c_int,
  locale: *const c_char,
  base: locale_t<'static>
) -> locale_t<'static> {
  match newlocale_inner(mask, locale, base) {
    | Ok(locale) => locale,
    | Err(err) => {
      errno::set_errno(err);
      ptr::null_mut()
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_freelocale(locale: locale_t<'static>) {
  if locale.is_null() || locale == LC_GLOBAL_LOCALE {
    return;
  }

  unsafe { drop(Box::from_raw(locale)) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_duplocale(base: locale_t<'static>) -> locale_t<'static> {
  base
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_uselocale(new: locale_t<'static>) -> locale_t<'static> {
  let old = locale::get_thread_locale_ptr();

  if !new.is_null() {
    locale::set_thread_locale_ptr(new);
  }

  old
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_getlocalename_l(
  category: c_int,
  locale: locale_t<'static>
) -> *const c_char {
  if locale.is_null() || category < 0 || category > LC_ALL {
    return ptr::null();
  }

  let locale = locale::get_real_locale(locale);

  locale.querylocale(category)
}
