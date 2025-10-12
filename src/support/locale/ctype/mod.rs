pub mod casemap;
pub mod converter;

use {
  super::LocaleObject,
  crate::{c_int, std::errno},
  allocation::borrow::{Cow, ToOwned},
  core::ffi
};

pub struct CtypeObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub casemap: casemap::CaseMapObject,
  pub converter: converter::ConverterObject<'a>
}

impl<'a> LocaleObject for CtypeObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str();
    let name = match name {
      | Ok(s) => s,
      | Err(_) => return Err(errno::ENOENT)
    };

    if name == "C" || name == "POSIX" {
      return Ok(self.set_to_posix());
    }

    let mut parts = name.split(['.', '@']);

    if let Some(lang) = parts.next() {
      // Handle locales such as C.UTF-8 and POSIX.UTF-8
      if name == "C" || name == "POSIX" || lang.is_empty() {
        self.casemap = casemap::ascii::CASEMAP_ASCII;
      } else {
        self.casemap = casemap::icu::CASEMAP_ICU;
      }
    }
    if let Some(codeset) = parts.next() {
      for c in converter::AVAILABLE_CONVERTERS {
        if c.name == codeset {
          self.name = Cow::Owned(locale.to_owned());
          self.converter = c.converter;

          return Ok(self.name.as_ref());
        }
      }
    }

    Err(errno::ENOENT)
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_CTYPE;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for CtypeObject<'a> {
  fn default() -> Self {
    DEFAULT_CTYPE
  }
}

pub const DEFAULT_CTYPE: CtypeObject = CtypeObject {
  name: Cow::Borrowed(c"C"),
  casemap: casemap::ascii::CASEMAP_ASCII,
  converter: converter::ascii::CONVERTER_ASCII
};
