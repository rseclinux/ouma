use {
  crate::{c_char, c_int, char32_t, mbstate_t, size_t, ssize_t, std::errno},
  core::ffi
};

mod ascii;
mod utf8;

#[derive(Copy, Clone)]
pub struct LCCtype<'a> {
  name: *const c_char,
  pub codeset: &'a ffi::CStr,
  pub mbtoc32:
    fn(*mut char32_t, *const c_char, size_t, *mut mbstate_t) -> ssize_t,
  pub c32tomb: fn(*mut c_char, char32_t, *mut mbstate_t) -> ssize_t,
  pub mb_cur_max: c_int
}

struct Ctypes<'a> {
  pub name: &'a str,
  pub ctype: &'a LCCtype<'a>
}

const AVAILABLE_CTYPES: [Ctypes; 4] = [
  Ctypes { name: "ASCII", ctype: &ascii::CTYPE_ASCII },
  Ctypes { name: "US-ASCII", ctype: &ascii::CTYPE_ASCII },
  Ctypes { name: "UTF8", ctype: &utf8::CTYPE_UTF8 },
  Ctypes { name: "UTF-8", ctype: &utf8::CTYPE_UTF8 }
];

impl<'a> super::LocaleObject for LCCtype<'a> {
  fn set_to(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<*mut c_char, c_int> {
    let name = locale.to_str();
    let name: &str = match name {
      | Ok(name) => name,
      | Err(_) => return Err(errno::EINVAL)
    };

    // Handle POSIX locale
    if name == "C" || name == "POSIX" {
      self.name = locale.as_ptr();
      self.codeset = DEFAULT_CTYPE.codeset;
      self.mbtoc32 = DEFAULT_CTYPE.mbtoc32;
      self.c32tomb = DEFAULT_CTYPE.c32tomb;
      self.mb_cur_max = DEFAULT_CTYPE.mb_cur_max;
      return Ok(locale.as_ptr().cast_mut());
    }

    let mut parts = name.split('.');
    if let Some(_) = parts.next() {
      // Skip language processing
    }
    if let Some(codeset) = parts.next() {
      if !codeset.is_empty() {
        for c in AVAILABLE_CTYPES {
          if c.name.to_lowercase() == codeset.to_lowercase() {
            self.name = locale.as_ptr();
            self.codeset = c.ctype.codeset;
            self.mbtoc32 = c.ctype.mbtoc32;
            self.c32tomb = c.ctype.c32tomb;
            self.mb_cur_max = c.ctype.mb_cur_max;
            return Ok(locale.as_ptr().cast_mut());
          }
        }
      }
    }

    Err(errno::ENOENT)
  }

  fn get_name(&self) -> *mut c_char {
    self.name.cast_mut()
  }
}

pub const DEFAULT_CTYPE: LCCtype = LCCtype {
  name: c"C".as_ptr(),
  codeset: ascii::CTYPE_ASCII.codeset,
  mbtoc32: ascii::CTYPE_ASCII.mbtoc32,
  c32tomb: ascii::CTYPE_ASCII.c32tomb,
  mb_cur_max: ascii::CTYPE_ASCII.mb_cur_max
};
