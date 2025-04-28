pub mod collate;
pub mod ctype;
//pub mod messages;
//pub mod monetary;
//pub mod numeric;
//pub mod time;

use {
  crate::{
    c_char,
    c_int,
    std::{errno, locale}
  },
  core::ffi
};

trait LocaleObject {
  fn set_to(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<*mut c_char, c_int>;
  fn get_name(&self) -> *mut c_char;
}

#[derive(Copy, Clone)]
pub struct Locale<'a> {
  pub collate: collate::LCCollate,
  pub ctype: ctype::LCCtype<'a>
}

impl<'a> Locale<'a> {
  pub fn new() -> Self {
    Self { collate: collate::DEFAULT_COLLATE, ctype: ctype::DEFAULT_CTYPE }
  }

  pub fn setlocale(
    &mut self,
    category: c_int,
    locale: &ffi::CStr
  ) -> Result<*mut c_char, c_int> {
    match category {
      | locale::LC_CTYPE => self.ctype.set_to(&locale),
      | locale::LC_COLLATE => self.collate.set_to(&locale),
      | locale::LC_ALL => {
        if self.ctype.set_to(&locale).is_ok() &&
          self.collate.set_to(&locale).is_ok()
        {
          Ok(locale.as_ptr().cast_mut())
        } else {
          Err(errno::ENOENT)
        }
      },
      | _ => Err(errno::EINVAL)
    }
  }

  pub fn querylocale(
    &self,
    _category: c_int
  ) -> *mut c_char {
    c"C".as_ptr().cast_mut()
  }
}

pub const DEFAULT_LOCALE: Locale =
  Locale { collate: collate::DEFAULT_COLLATE, ctype: ctype::DEFAULT_CTYPE };

#[thread_local]
pub static mut ThreadLocale: Locale = DEFAULT_LOCALE;

#[inline(always)]
pub fn get_thread_locale() -> Locale<'static> {
  unsafe { ThreadLocale }
}

#[inline(always)]
pub fn set_thread_locale(locale: Locale<'static>) {
  unsafe { ThreadLocale = locale };
}
