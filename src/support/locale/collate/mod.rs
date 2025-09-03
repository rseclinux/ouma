use {
  super::{LocaleObject, is_posix_locale},
  crate::{c_int, std::errno},
  allocation::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec
  },
  bstr::{B, ByteSlice},
  core::{cmp::Ordering, ffi},
  icu_collator::{
    Collator,
    CollatorBorrowed,
    options::{CollatorOptions, Strength}
  },
  icu_locale::Locale
};

pub struct CollateObject<'a> {
  name: Cow<'a, ffi::CStr>,
  collator: Option<CollatorBorrowed<'a>>
}

impl<'a> CollateObject<'a> {
  pub fn get_sortkey_u8(
    &self,
    source: &'a [u8]
  ) -> Cow<'a, [u8]> {
    if let Some(collator) = &self.collator {
      let mut sortkey: Vec<u8> = Vec::new();

      if collator.write_sort_key_utf8_to(source, &mut sortkey).is_err() {
        return Cow::Borrowed(&[]);
      }

      Cow::Owned(sortkey)
    } else {
      Cow::Borrowed(&source)
    }
  }

  pub fn get_sortkey_u32(
    &self,
    source: &'a [u32]
  ) -> Cow<'a, [u32]> {
    if let Some(collator) = &self.collator {
      let source: &[u8] = &source
        .iter()
        .filter_map(|c| char::from_u32(*c))
        .collect::<String>()
        .into_bytes();
      let mut sortkey: Vec<u8> = Vec::new();

      if collator.write_sort_key_utf8_to(source, &mut sortkey).is_err() {
        return Cow::Borrowed(&[]);
      }

      let result: Vec<u32> = B(&sortkey).chars().map(|c| c as u32).collect();

      Cow::Owned(result)
    } else {
      Cow::Borrowed(&source)
    }
  }

  pub fn collate_u8(
    &self,
    lhs: &[u8],
    rhs: &[u8]
  ) -> Ordering {
    if let Some(collator) = &self.collator {
      collator.compare_utf8(lhs, rhs)
    } else {
      lhs.cmp(rhs)
    }
  }

  pub fn collate_u32(
    &self,
    lhs: &[u32],
    rhs: &[u32]
  ) -> Ordering {
    if let Some(collator) = &self.collator {
      let lhs: &[u8] = &lhs
        .iter()
        .filter_map(|c| char::from_u32(*c))
        .collect::<String>()
        .into_bytes();
      let rhs: &[u8] = &rhs
        .iter()
        .filter_map(|c| char::from_u32(*c))
        .collect::<String>()
        .into_bytes();

      collator.compare_utf8(lhs, rhs)
    } else {
      lhs.cmp(rhs)
    }
  }
}

impl<'a> LocaleObject for CollateObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str();
    let name = match name {
      | Ok(s) => s,
      | Err(_) => return Err(errno::EINVAL)
    };

    if is_posix_locale(name) {
      return Ok(self.set_to_posix());
    }

    let mut parts = name.split('.');
    let lang = parts.next().unwrap_or("");
    if lang.is_empty() {
      return Err(errno::EINVAL);
    }

    let icu_locale = Locale::try_from_str(&lang.replace("_", "-"));
    let icu_locale = match icu_locale {
      | Ok(icu_locale) => icu_locale,
      | Err(_) => return Err(errno::EINVAL)
    };

    let mut options = CollatorOptions::default();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(icu_locale.into(), options);
    let collator = match collator {
      | Ok(collator) => collator,
      | Err(_) => return Err(errno::EINVAL)
    };

    self.name = Cow::Owned(locale.to_owned());
    self.collator = Some(collator);

    Ok(self.name.as_ref())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_COLLATE;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Clone for CollateObject<'a> {
  fn clone(&self) -> Self {
    let name = self.name.clone();

    let collator = if self.collator.is_some() {
      let mut rebuilt: Option<CollatorBorrowed<'a>> = None;

      if let Ok(s) = name.as_ref().to_str() {
        if !super::is_posix_locale(s) {
          let lang = s.split('.').next().unwrap_or("");
          if !lang.is_empty() {
            if let Ok(icu_locale) =
              icu_locale::Locale::try_from_str(&lang.replace("_", "-"))
            {
              let mut options =
                icu_collator::options::CollatorOptions::default();
              options.strength =
                Some(icu_collator::options::Strength::Quaternary);
              if let Ok(c) =
                icu_collator::Collator::try_new(icu_locale.into(), options)
              {
                rebuilt = Some(c);
              }
            }
          }
        }
      }
      rebuilt
    } else {
      None
    };

    CollateObject { name, collator }
  }
}

impl<'a> Default for CollateObject<'a> {
  fn default() -> Self {
    DEFAULT_COLLATE
  }
}

pub const DEFAULT_COLLATE: CollateObject =
  CollateObject { name: Cow::Borrowed(c"C"), collator: None };
