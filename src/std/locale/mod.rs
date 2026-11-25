use {
  crate::{
    c_char,
    c_int,
    intptr_t,
    locale_t,
    std::errno,
    support::locale::{self, LocaleObject}
  },
  allocation::sync::Arc,
  allocator_api2::boxed::Box,
  core::{ffi, ptr}
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
#[derive(Clone, Copy, Debug)]
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
  #[inline]
  pub const fn new() -> Self {
    Self {
      decimal_point: c".".as_ptr().cast_mut(),
      thousands_sep: c"".as_ptr().cast_mut(),
      grouping: c"".as_ptr().cast_mut(),
      int_curr_symbol: c"".as_ptr().cast_mut(),
      currency_symbol: c"".as_ptr().cast_mut(),
      mon_decimal_point: c"".as_ptr().cast_mut(),
      mon_thousands_sep: c"".as_ptr().cast_mut(),
      mon_grouping: c"".as_ptr().cast_mut(),
      positive_sign: c"".as_ptr().cast_mut(),
      negative_sign: c"".as_ptr().cast_mut(),
      int_frac_digits: c_char::MAX,
      frac_digits: c_char::MAX,
      p_cs_precedes: c_char::MAX,
      p_sep_by_space: c_char::MAX,
      n_cs_precedes: c_char::MAX,
      n_sep_by_space: c_char::MAX,
      p_sign_posn: c_char::MAX,
      n_sign_posn: c_char::MAX,
      int_p_cs_precedes: c_char::MAX,
      int_p_sep_by_space: c_char::MAX,
      int_n_cs_precedes: c_char::MAX,
      int_n_sep_by_space: c_char::MAX,
      int_p_sign_posn: c_char::MAX,
      int_n_sign_posn: c_char::MAX
    }
  }
}

#[inline]
fn inherit<T: LocaleObject>(
  lhs: &mut Option<Arc<T>>,
  rhs: &Option<Arc<T>>
) {
  *lhs = rhs.clone();
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_localeconv() -> *mut lconv {
  let locale = locale::get_thread_locale_ptr();
  rs_localeconv_l(locale)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_localeconv_l(locale: locale_t<'static>) -> *mut lconv {
  let locale: &locale::Locale = locale::get_real_locale(locale);
  locale.localeconv.as_ptr()
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
    return locale::get_global_locale().querylocale(category);
  }

  let locale = unsafe { ffi::CStr::from_ptr(locale) };
  if locale.is_empty() {
    locales[category as usize] = Some(c"C"); // TODO: get from environment
  } else {
    locales[category as usize] = Some(locale);
  }

  for (c, lc) in locales.iter().enumerate() {
    if let Some(l) = lc {
      if !available::AVAILABLE_LOCALES.contains(&&l) {
        return ptr::null_mut();
      }

      let mut newloc = locale::Locale::new();

      let base = locale::get_global_locale();

      inherit(&mut newloc.collate, &base.collate);
      inherit(&mut newloc.ctype, &base.ctype);
      inherit(&mut newloc.messages, &base.messages);
      inherit(&mut newloc.monetary, &base.monetary);
      inherit(&mut newloc.numeric, &base.numeric);
      inherit(&mut newloc.time, &base.time);

      if newloc.setlocale(c as c_int, l).is_err() {
        return ptr::null_mut();
      }

      newloc.update_strings();

      locale::set_global_locale(newloc);
    }
  }

  locale::get_global_locale().querylocale(category)
}

#[inline]
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

  let mut newloc =
    Box::try_new(locale::Locale::new()).map_err(|_| errno::ENOMEM)?;

  inherit(&mut newloc.collate, &base.collate);
  inherit(&mut newloc.ctype, &base.ctype);
  inherit(&mut newloc.messages, &base.messages);
  inherit(&mut newloc.monetary, &base.monetary);
  inherit(&mut newloc.numeric, &base.numeric);
  inherit(&mut newloc.time, &base.time);

  if (mask & LC_COLLATE_MASK) != 0 {
    locale::set_slot(&mut newloc.collate, name)?;
  }
  if (mask & LC_CTYPE_MASK) != 0 {
    locale::set_slot(&mut newloc.ctype, name)?;
  }
  if (mask & LC_MESSAGES_MASK) != 0 {
    locale::set_slot(&mut newloc.messages, name)?;
  }
  if (mask & LC_MONETARY_MASK) != 0 {
    locale::set_slot(&mut newloc.monetary, name)?;
  }
  if (mask & LC_NUMERIC_MASK) != 0 {
    locale::set_slot(&mut newloc.numeric, name)?;
  }
  if (mask & LC_TIME_MASK) != 0 {
    locale::set_slot(&mut newloc.time, name)?;
  }

  newloc.update_strings();

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

  unsafe { drop(Box::from_raw(locale)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_duplocale(locale: locale_t<'static>) -> locale_t<'static> {
  if locale.is_null() {
    errno::set_errno(errno::EINVAL);
    return ptr::null_mut();
  }

  let newloc = Box::try_new(locale::Locale::new());
  let mut newloc = match newloc {
    | Ok(n) => n,
    | Err(_) => {
      errno::set_errno(errno::ENOMEM);
      return ptr::null_mut();
    }
  };

  let base = locale::get_real_locale(locale);

  inherit(&mut newloc.collate, &base.collate);
  inherit(&mut newloc.ctype, &base.ctype);
  inherit(&mut newloc.messages, &base.messages);
  inherit(&mut newloc.monetary, &base.monetary);
  inherit(&mut newloc.numeric, &base.numeric);
  inherit(&mut newloc.time, &base.time);

  newloc.update_strings();

  Box::into_raw(newloc)
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
