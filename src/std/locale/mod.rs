pub mod posix;

use {
  crate::{LocaleStruct, c_char, c_int, std::errno, support::locale::*},
  core::{ffi, ptr}
};

pub const LC_CTYPE: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 2;
pub const LC_COLLATE: c_int = 3;
pub const LC_MONETARY: c_int = 4;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = 6;

const LC_LAST: c_int = 7;

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

struct Linguas<'a> {
  pub name: &'a ffi::CStr,
  pub collate: collate::LCCollate,
  pub ctype: ctype::LCCtype<'a>,
  pub messages: messages::LCMessages<'a>,
  pub monetary: monetary::LCMonetary<'a>,
  pub numeric: numeric::LCNumeric<'a>,
  pub time: time::LCTime<'a>
}

const AVAILABLE_LINGUAS: [Linguas; 6] = [
  Linguas {
    name: c"C",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::ascii::CTYPE_ASCII,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::posix::MONETARY_POSIX,
    numeric: numeric::posix::NUMERIC_POSIX,
    time: time::posix::TIME_POSIX
  },
  Linguas {
    name: c"C.UTF-8",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::utf8::CTYPE_UTF8,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::posix::MONETARY_POSIX,
    numeric: numeric::posix::NUMERIC_POSIX,
    time: time::posix::TIME_POSIX
  },
  Linguas {
    name: c"POSIX",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::ascii::CTYPE_ASCII,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::posix::MONETARY_POSIX,
    numeric: numeric::posix::NUMERIC_POSIX,
    time: time::posix::TIME_POSIX
  },
  Linguas {
    name: c"en_US",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::ascii::CTYPE_ASCII,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::en_us::MONETARY_EN_US,
    numeric: numeric::en_us::NUMERIC_EN_US,
    time: time::en_us::TIME_EN_US
  },
  Linguas {
    name: c"en_US.UTF-8",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::utf8::CTYPE_UTF8,
    messages: messages::en_us::MESSAGES_EN_US,
    monetary: monetary::en_us::MONETARY_EN_US,
    numeric: numeric::en_us::NUMERIC_EN_US,
    time: time::en_us::TIME_EN_US
  },
  Linguas {
    name: c"ru_RU.UTF-8",
    collate: collate::COLLATE_POSIX,
    ctype: ctype::utf8::CTYPE_UTF8,
    messages: messages::ru_ru::MESSAGES_RU_RU,
    monetary: monetary::ru_ru::MONETARY_RU_RU,
    numeric: numeric::ru_ru::NUMERIC_RU_RU,
    time: time::ru_ru::TIME_RU_RU
  }
];

#[inline(always)]
fn get_locale_component_name(
  locale: LocaleStruct,
  category: c_int
) -> *mut c_char {
  match category {
    | LC_ALL => return locale.lc_all.as_ptr().cast_mut(),
    | LC_COLLATE => return locale.lc_collate.as_ptr().cast_mut(),
    | LC_CTYPE => return locale.lc_ctype.as_ptr().cast_mut(),
    | LC_MESSAGES => return locale.lc_messages.as_ptr().cast_mut(),
    | LC_MONETARY => return locale.lc_monetary.as_ptr().cast_mut(),
    | LC_NUMERIC => return locale.lc_numeric.as_ptr().cast_mut(),
    | LC_TIME => return locale.lc_time.as_ptr().cast_mut(),
    | _ => unreachable!()
  }
}

#[no_mangle]
pub extern "C" fn rs_setlocale(
  category: c_int,
  name: *const c_char
) -> *mut c_char {
  if category >= LC_LAST {
    errno::set_errno(errno::EINVAL);
    return ptr::null_mut();
  }

  if name.is_null() {
    return get_locale_component_name(get_thread_locale(), category);
  }

  let mut new_locale = get_thread_locale(); // C tells us to copy from thread

  let name = unsafe { ffi::CStr::from_ptr(name) };
  let name = if name.is_empty() {
    name // TODO: get locale from environment
  } else {
    name
  };

  for i in 0..AVAILABLE_LINGUAS.len() {
    macro_rules! update_category {
      ($name:ident, $component:ident, $category:expr) => {
        if category == LC_ALL {
          new_locale.lc_all = AVAILABLE_LINGUAS[i].name;
          new_locale.$name = AVAILABLE_LINGUAS[i].name;
          new_locale.$component = Some(AVAILABLE_LINGUAS[i].$component);
        } else {
          if $category == category {
            new_locale.$name = AVAILABLE_LINGUAS[i].name;
            new_locale.$component = Some(AVAILABLE_LINGUAS[i].$component);
          }
        }
      };
    }

    if name == AVAILABLE_LINGUAS[i].name {
      update_category!(lc_collate, collate, LC_COLLATE);
      update_category!(lc_ctype, ctype, LC_CTYPE);
      update_category!(lc_messages, messages, LC_MESSAGES);
      update_category!(lc_monetary, monetary, LC_MONETARY);
      update_category!(lc_numeric, numeric, LC_NUMERIC);
      update_category!(lc_time, time, LC_TIME);

      set_thread_locale(new_locale);

      return get_locale_component_name(new_locale, category);
    }
  }

  ptr::null_mut()
}
