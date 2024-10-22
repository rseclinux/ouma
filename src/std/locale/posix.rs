use {
  crate::{
    LocaleStruct,
    c_char,
    c_int,
    locale_t,
    std::errno,
    support::locale::*
  },
  allocation::boxed,
  core::{ffi, ptr}
};

pub const LC_CTYPE_MASK: c_int = 1 << super::LC_CTYPE;
pub const LC_NUMERIC_MASK: c_int = 1 << super::LC_NUMERIC;
pub const LC_TIME_MASK: c_int = 1 << super::LC_TIME;
pub const LC_COLLATE_MASK: c_int = 1 << super::LC_COLLATE;
pub const LC_MONETARY_MASK: c_int = 1 << super::LC_MONETARY;
pub const LC_MESSAGES_MASK: c_int = 1 << super::LC_MESSAGES;
pub const LC_ALL_MASK: c_int = LC_CTYPE_MASK |
  LC_NUMERIC_MASK |
  LC_TIME_MASK |
  LC_COLLATE_MASK |
  LC_MONETARY_MASK |
  LC_MESSAGES_MASK;

struct Linguas<'a> {
  pub name: &'a ffi::CStr,
  pub collate: collate::LCCollate,
  pub messages: messages::LCMessages<'a>,
  pub monetary: monetary::LCMonetary<'a>,
  pub numeric: numeric::LCNumeric<'a>,
  pub time: time::LCTime<'a>
}

struct Ctypes<'a> {
  pub name: &'a ffi::CStr,
  pub ctype: &'a ctype::LCCtype<'a>
}

const AVAILABLE_LINGUAS: [Linguas; 2] = [
  Linguas {
    name: c"en_US",
    collate: collate::COLLATE_POSIX,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::en_us::MONETARY_EN_US,
    numeric: numeric::en_us::NUMERIC_EN_US,
    time: time::en_us::TIME_EN_US
  },
  Linguas {
    name: c"ru_RU",
    collate: collate::COLLATE_POSIX,
    messages: messages::ru_ru::MESSAGES_RU_RU,
    monetary: monetary::ru_ru::MONETARY_RU_RU,
    numeric: numeric::ru_ru::NUMERIC_RU_RU,
    time: time::ru_ru::TIME_RU_RU
  }
];

const AVAILABLE_CTYPES: [Ctypes; 4] = [
  Ctypes { name: c"ASCII", ctype: &ctype::ascii::CTYPE_ASCII },
  Ctypes { name: c"US-ASCII", ctype: &ctype::ascii::CTYPE_ASCII },
  Ctypes { name: c"UTF8", ctype: &ctype::utf8::CTYPE_UTF8 },
  Ctypes { name: c"UTF-8", ctype: &ctype::utf8::CTYPE_UTF8 }
];

#[no_mangle]
pub extern "C" fn rs_duplocale(base: locale_t) -> locale_t {
  base
}

#[no_mangle]
pub extern "C" fn rs_freelocale(_: locale_t) {
  // Nothing to do
}

#[no_mangle]
pub extern "C" fn rs_newlocale(
  mask: c_int,
  name: *const c_char,
  base: locale_t
) -> locale_t {
  let name = if name.is_null() {
    c"C" // TODO: Get from environment
  } else {
    unsafe { ffi::CStr::from_ptr(name) }
  };
  let base =
    if base.is_null() { &mut get_thread_locale() as locale_t } else { base };
  let mut new_locale = LocaleStruct::new();

  if name.is_empty() || mask == 0 {
    errno::set_errno(errno::EINVAL);
    return -1isize as locale_t;
  }

  let mut parts = name.to_str().expect("Malformed locale name").split('.');
  if let Some(lingua) = parts.next() {
    for i in 0..AVAILABLE_LINGUAS.len() {
      if lingua ==
        AVAILABLE_LINGUAS[i].name.to_str().expect("Malformed locale name")
      {
        new_locale.lc_all = name;
        new_locale.lc_collate = name;
        new_locale.lc_messages = name;
        new_locale.lc_monetary = name;
        new_locale.lc_numeric = name;
        new_locale.lc_time = name;

        new_locale.collate = Some(AVAILABLE_LINGUAS[i].collate);
        new_locale.messages = Some(AVAILABLE_LINGUAS[i].messages);
        new_locale.monetary = Some(AVAILABLE_LINGUAS[i].monetary);
        new_locale.numeric = Some(AVAILABLE_LINGUAS[i].numeric);
        new_locale.time = Some(AVAILABLE_LINGUAS[i].time);
      }
    }
  }
  if let Some(ctype) = parts.next() {
    for i in 0..AVAILABLE_CTYPES.len() {
      if ctype ==
        AVAILABLE_CTYPES[i].name.to_str().expect("Malformed locale name")
      {
        new_locale.lc_ctype = name;
        new_locale.ctype = Some(*AVAILABLE_CTYPES[i].ctype);
      }
    }
  }

  let mut different = false;
  macro_rules! update_category {
    ($name:ident, $component:ident, $bit:expr) => {
      unsafe {
        if mask & LC_ALL_MASK == 0 {
          new_locale.lc_all = (*base).lc_all;
          new_locale.$name = (*base).$name;
          new_locale.$component = (*base).$component;
        } else if mask & $bit == 0 {
          new_locale.$name = (*base).$name;
          new_locale.$component = (*base).$component;
        } else if new_locale.$name != (*base).$name {
          different = true;
          if new_locale.$component.is_none() {
            errno::set_errno(errno::ENOENT);
            return ptr::null_mut();
          }
        }
      }
    };
  }

  update_category!(lc_collate, collate, LC_COLLATE_MASK);
  update_category!(lc_ctype, ctype, LC_CTYPE_MASK);
  update_category!(lc_messages, messages, LC_MESSAGES_MASK);
  update_category!(lc_monetary, monetary, LC_MONETARY_MASK);
  update_category!(lc_numeric, numeric, LC_NUMERIC_MASK);
  update_category!(lc_time, time, LC_TIME_MASK);

  if !different {
    return base;
  }

  let copy = boxed::Box::into_raw(boxed::Box::new(new_locale));
  if copy.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  copy
}

#[no_mangle]
pub extern "C" fn rs_uselocale(new: locale_t<'static>) -> locale_t<'static> {
  let old = &mut get_thread_locale() as locale_t;
  if !new.is_null() {
    unsafe { set_thread_locale(*new) };
  }
  if old.is_null() { old } else { -1isize as locale_t }
}
