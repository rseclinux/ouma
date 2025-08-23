mod collate;
mod ctype;
//mod messages;
mod monetary;
mod numeric;
//mod time;

use {crate::c_int, core::ffi};

trait LocaleObject {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int>;
  fn set_to_posix(&mut self) -> &ffi::CStr;
  fn get_name(&self) -> &ffi::CStr;
}

#[inline]
pub fn is_posix_locale(name: &str) -> bool {
  name == "C" ||
    name == "POSIX" ||
    name.starts_with("C.") ||
    name.starts_with("POSIX.")
}
