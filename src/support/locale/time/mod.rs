use {
  super::{LocaleObject, is_posix_locale},
  crate::{allocation::borrow::ToOwned, c_int, support::locale::errno},
  allocation::borrow::Cow,
  core::ffi
};

pub struct TimeObject<'a> {
  name: Cow<'a, ffi::CStr>
}

impl<'a> LocaleObject for TimeObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str();
    let name = match name {
      | Ok(s) => s,
      | Err(_) => return Err(errno::ENOENT)
    };

    if is_posix_locale(name) {
      return Ok(self.set_to_posix());
    }

    let mut _parts = name.split(['.', '@']);
    let _lang = _parts.next().unwrap_or("");
    if _lang.is_empty() {
      return Err(errno::ENOENT);
    }

    self.name = Cow::Owned(locale.to_owned());

    Ok(self.name.as_ref())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_TIME;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for TimeObject<'a> {
  fn default() -> Self {
    DEFAULT_TIME
  }
}

pub const DEFAULT_TIME: TimeObject = TimeObject { name: Cow::Borrowed(c"C") };
