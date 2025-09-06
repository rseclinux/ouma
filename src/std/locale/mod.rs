use {
  crate::{c_char, c_int, intptr_t, locale_t, support::locale},
  core::{ffi, ptr}
};

pub const LC_CTYPE: c_int = 0;
pub const LC_NUMERIC: c_int = 1;
pub const LC_TIME: c_int = 2;
pub const LC_COLLATE: c_int = 3;
pub const LC_MONETARY: c_int = 4;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = 6;

pub const LC_GLOBAL_LOCALE: locale_t = -1 as intptr_t as locale_t;

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
      let changed = locale::get_thread_locale();
      if let Ok(result) = changed.setlocale(c as c_int, l) {
        return result.querylocale(c as c_int);
      }
    }
  }

  ptr::null_mut()
}
