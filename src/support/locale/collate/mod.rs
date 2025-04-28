mod posix;
mod uca;

use {
  crate::{c_char, c_int, size_t, wchar_t},
  core::ffi
};

#[derive(Copy, Clone)]
pub struct LCCollate {
  name: *const c_char,
  pub strcoll: fn(*const c_char, *const c_char) -> c_int,
  pub strxfrm: fn(*mut c_char, *const c_char, size_t) -> size_t,
  pub wcscoll: fn(*const wchar_t, *const wchar_t) -> c_int,
  pub wcsxfrm: fn(*mut wchar_t, *const wchar_t, size_t) -> size_t
}

impl super::LocaleObject for LCCollate {
  fn set_to(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<*mut c_char, c_int> {
    // Basically setlocale on LC_COLLATE will never fail
    // Since on any other locales it will UCA
    if locale == c"C" || locale == c"POSIX" {
      self.strcoll = DEFAULT_COLLATE.strcoll;
      self.strxfrm = DEFAULT_COLLATE.strxfrm;
      self.wcscoll = DEFAULT_COLLATE.wcscoll;
      self.wcsxfrm = DEFAULT_COLLATE.wcsxfrm;
    } else {
      self.strcoll = uca::UCA_COLLATE.strcoll;
      self.strxfrm = uca::UCA_COLLATE.strxfrm;
      self.wcscoll = uca::UCA_COLLATE.wcscoll;
      self.wcsxfrm = uca::UCA_COLLATE.wcsxfrm;
    }

    self.name = locale.as_ptr();
    Ok(locale.as_ptr().cast_mut())
  }

  fn get_name(&self) -> *mut c_char {
    self.name.cast_mut()
  }
}

pub const DEFAULT_COLLATE: LCCollate = LCCollate {
  name: c"C".as_ptr(),
  strcoll: posix::POSIX_COLLATE.strcoll,
  strxfrm: posix::POSIX_COLLATE.strxfrm,
  wcscoll: posix::POSIX_COLLATE.wcscoll,
  wcsxfrm: posix::POSIX_COLLATE.wcsxfrm
};
