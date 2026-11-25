use {
  super::{LocaleObject, canonicalize_locale, is_posix_locale},
  crate::{c_int, std::errno},
  allocation::{
    borrow::{Cow, ToOwned},
    string::String
  },
  allocator_api2::{collections::TryReserveError, vec::Vec},
  bstr::{B, ByteSlice},
  core::{cmp::Ordering, ffi},
  icu_collator::{
    CollationKeySink,
    Collator,
    CollatorBorrowed,
    options::{CollatorOptions, Strength}
  },
  icu_locale::Locale
};

struct VecWrap(pub Vec<u8>);

impl CollationKeySink for VecWrap {
  type Error = TryReserveError;
  type State = ();
  type Output = ();

  #[inline]
  fn write(
    &mut self,
    _: &mut Self::State,
    buf: &[u8]
  ) -> Result<(), Self::Error> {
    self.0.try_reserve_exact(buf.len())?;
    self.0.extend_from_slice(buf);
    Ok(())
  }

  #[inline]
  fn finish(
    &mut self,
    _: Self::State
  ) -> Result<Self::Output, Self::Error> {
    Ok(())
  }
}

#[derive(Debug)]
pub struct CollateObject<'a> {
  name: Cow<'a, ffi::CStr>,
  collator: Option<CollatorBorrowed<'a>>
}

impl<'a> CollateObject<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self { name: Cow::Borrowed(c"C"), collator: None }
  }

  #[inline]
  pub fn get_sortkey_u8(
    &self,
    source: &'a [u8]
  ) -> Cow<'a, [u8]> {
    if let Some(collator) = &self.collator {
      let mut sortkey: VecWrap = VecWrap(Vec::new());

      if collator.write_sort_key_utf8_to(source, &mut sortkey).is_err() {
        return Cow::Borrowed(&source);
      }

      Cow::Owned(sortkey.0.to_vec())
    } else {
      Cow::Borrowed(&source)
    }
  }

  #[inline]
  pub fn get_sortkey_u32(
    &self,
    source: &'a [u32]
  ) -> Cow<'a, [u32]> {
    if let Some(collator) = &self.collator {
      let s = &source
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<String>();
      let mut sortkey: VecWrap = VecWrap(Vec::new());

      if collator.write_sort_key_to(s, &mut sortkey).is_err() {
        return Cow::Borrowed(&source);
      }

      let result: Vec<u32> = B(&sortkey.0).chars().map(|c| c as u32).collect();

      Cow::Owned(result.to_vec())
    } else {
      Cow::Borrowed(&source)
    }
  }

  #[inline]
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

  #[inline]
  pub fn collate_u32(
    &self,
    lhs: &[u32],
    rhs: &[u32]
  ) -> Ordering {
    if let Some(collator) = &self.collator {
      let lhs = &lhs
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<String>();
      let rhs = &rhs
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<String>();

      collator.compare(lhs, rhs)
    } else {
      lhs.cmp(rhs)
    }
  }
}

impl<'a> LocaleObject for CollateObject<'a> {
  #[inline]
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str().map_err(|_| errno::EINVAL)?;

    if is_posix_locale(name) {
      return Ok(self.set_to_posix(locale));
    }

    let (icu_locale_name, _) = canonicalize_locale(name);

    let icu_locale =
      Locale::try_from_str(&icu_locale_name).map_err(|_| errno::ENOENT)?;

    let mut options = CollatorOptions::default();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(icu_locale.into(), options)
      .map_err(|_| errno::ENOENT)?;

    self.name = Cow::Owned(locale.to_owned());
    self.collator = Some(collator);

    Ok(&self.name)
  }

  #[inline]
  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr {
    *self = Self::new();

    self.name = Cow::Owned(locale.to_owned());
    self.name.as_ref()
  }

  #[inline]
  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for CollateObject<'a> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
