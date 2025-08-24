pub mod casemap;
pub mod converter;

use {
  super::LocaleObject,
  crate::{c_int, std::errno},
  allocation::borrow::{Cow, ToOwned},
  core::ffi,
  icu_casemap::CaseMapper,
  icu_locale::Locale,
  writeable::Writeable
};

pub struct CtypeObject<'a> {
  name: Cow<'a, ffi::CStr>,
  locale: Option<Locale>,
  pub casemap: casemap::CaseMapObject,
  pub converter: converter::ConverterObject<'a>
}

impl<'a> CtypeObject<'a> {
  pub fn tolower(
    &self,
    c: u32
  ) -> u32 {
    if let Some(locale) = &self.locale {
      let Ok(c) = char::try_from(c) else {
        return c as u32;
      };
      let cm = CaseMapper::new();

      let mut buffer = [0; 4];
      let string = c.encode_utf8(&mut buffer);

      let result = cm
        .lowercase(string, &locale.id)
        .write_to_string()
        .chars()
        .next()
        .unwrap_or(c);
      result as u32
    } else {
      if c >= 'A' as u32 && c <= 'Z' as u32 {
        return c - 'A' as u32 + 'a' as u32;
      }
      c
    }
  }

  pub fn toupper(
    &self,
    c: u32
  ) -> u32 {
    if let Some(locale) = &self.locale {
      let Ok(c) = char::try_from(c) else {
        return c as u32;
      };
      let cm = CaseMapper::new();

      let mut buffer = [0; 4];
      let string = c.encode_utf8(&mut buffer);

      let result = cm
        .uppercase(string, &locale.id)
        .write_to_string()
        .chars()
        .next()
        .unwrap_or(c);
      result as u32
    } else {
      if c >= 'a' as u32 && c <= 'z' as u32 {
        return c - 'a' as u32 + 'A' as u32;
      }
      c
    }
  }
}

impl<'a> LocaleObject for CtypeObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str();
    let name = match name {
      | Ok(s) => s,
      | Err(_) => return Err(errno::EINVAL)
    };

    if name == "C" || name == "POSIX" {
      return Ok(self.set_to_posix());
    }

    let mut parts = name.split('.');

    if let Some(lang) = parts.next() &&
      !lang.is_empty()
    {
      // Handle locales such as C.UTF-8 and POSIX.UTF-8
      if name == "C" || name == "POSIX" {
        self.locale = None;
        self.casemap = casemap::ascii::CASEMAP_ASCII;
      } else {
        let icu_locale = Locale::try_from_str(&lang.replace("_", "-"));
        let icu_locale = match icu_locale {
          | Ok(icu_locale) => icu_locale,
          | Err(_) => return Err(errno::EINVAL)
        };

        self.locale = Some(icu_locale);
        self.casemap = casemap::icu::CASEMAP_ICU;
      }
    }
    if let Some(codeset) = parts.next() &&
      !codeset.is_empty()
    {
      for c in converter::AVAILABLE_CONVERTERS {
        if c.name == codeset {
          self.name = Cow::Owned(locale.to_owned());
          self.converter = c.converter;

          return Ok(self.name.as_ref());
        }
      }
    }

    Err(errno::EINVAL)
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
  locale: None,
  casemap: casemap::ascii::CASEMAP_ASCII,
  converter: converter::ascii::CONVERTER_ASCII
};
