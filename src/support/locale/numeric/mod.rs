use {
  super::{LocaleObject, canonicalize_locale, is_posix_locale},
  crate::{std::errno, support::string::strtocstr, types::c_int},
  allocation::{
    borrow::{Cow, ToOwned},
    string::{String, ToString}
  },
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_locale::Locale,
  smallvec::{SmallVec, smallvec}
};

#[inline]
pub fn get_grouping_strategy(locale: &Locale) -> options::GroupingStrategy {
  if let Some(region) = locale.id.region {
    match region.as_str() {
      | "CN" | "HK" | "PH" | "SG" | "FR" | "TW" | "MT" | "NP" | "MA" | "JP" => {
        return options::GroupingStrategy::Min2;
      },
      | _ => ()
    }
  }

  match locale.id.language.as_str() {
    | "ar" | "az" | "ckb" | "fa" | "pl" | "ja" => {
      options::GroupingStrategy::Min2
    },
    | _ => options::GroupingStrategy::Always
  }
}

#[inline]
pub fn get_thousands_sep(
  s: &str,
  strategy: options::GroupingStrategy
) -> Option<String> {
  let mut result: String = String::new();
  let mut index = 0usize;
  let iter = s.chars();

  for ch in iter.clone() {
    if let Some(c) = iter.clone().nth(index + 1) &&
      !c.is_whitespace() &&
      ch.is_whitespace()
    {
      return Some(String::from(' '));
    }
    if !ch.is_numeric() && !ch.is_whitespace() {
      let mut b = [0; 4];
      let encoded = ch.encode_utf8(&mut b);

      result.push_str(encoded);

      break;
    }
    index += 1;
  }

  if strategy == options::GroupingStrategy::Min2 && result.is_empty() {
    Some(String::from(' '))
  } else if result.is_empty() {
    None
  } else {
    Some(result)
  }
}

#[inline]
pub fn get_decimal_point(s: &str) -> Option<String> {
  let mut last = None;
  for (i, ch) in s.char_indices() {
    if !ch.is_numeric() && !ch.is_whitespace() {
      last = Some(i);
    }
  }
  match last {
    | Some(i) => {
      let sep = s[i..].chars().next()?;

      let mut b = [0; 4];
      let encoded = sep.encode_utf8(&mut b);

      Some(String::from(encoded))
    },
    | None => None
  }
}

#[inline]
pub fn get_grouping(locale: &Locale) -> SmallVec<[u8; 3]> {
  // https://lh.2xlibre.net/values/grouping/
  let lang = locale.id.language.to_string();
  let region = locale
    .id
    .region
    .and_then(|d| Some(d.to_string()))
    .unwrap_or(String::from(""));

  let mut result = match (region.as_str(), lang.as_str()) {
    | ("IN", "bn") |
    ("IN", "ml") |
    ("IN", "en") |
    ("IN", "ta") |
    ("IN", "te") |
    ("IN", "or") |
    ("IN", "mjw") => smallvec![3, 2],
    | ("TW", "cmn") | ("TW", "hak") | ("TW", "lzh") | ("TW", "nan") => {
      smallvec![4]
    },
    | ("BT", _) => smallvec![3, 2],
    | ("AN", _) |
    ("AW", _) |
    ("BA", _) |
    ("CU", _) |
    ("CW", _) |
    ("CY", _) |
    ("DJ", _) |
    ("ER", _) |
    ("GR", _) |
    ("MG", _) |
    ("PT", _) |
    ("RS", _) |
    ("RW", _) |
    ("SA", _) |
    ("SI", _) => {
      smallvec![]
    },
    | _ => smallvec![3]
  };

  result.push(0);
  result
}

#[derive(Debug, Clone)]
pub struct NumericObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub decimal_point: Cow<'a, ffi::CStr>,
  pub thousands_sep: Cow<'a, ffi::CStr>,
  pub grouping: SmallVec<[u8; 3]>
}

impl<'a> NumericObject<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      name: Cow::Borrowed(c"C"),
      decimal_point: Cow::Borrowed(c"."),
      thousands_sep: Cow::Borrowed(c""),
      grouping: SmallVec::new_const()
    }
  }

  #[inline]
  pub fn get_decimal_point(&self) -> Option<char> {
    self.decimal_point.to_str().ok()?.chars().nth(0)
  }

  #[inline]
  pub fn get_thousands_sep(&self) -> Option<char> {
    self.thousands_sep.to_str().ok()?.chars().nth(0)
  }
}

impl<'a> LocaleObject for NumericObject<'a> {
  #[inline]
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    self.grouping.clear();

    let name = locale.to_str().map_err(|_| errno::EINVAL)?;

    if is_posix_locale(name) {
      return Ok(self.set_to_posix(locale));
    }

    let (icu_locale_name, _) = canonicalize_locale(name);

    let icu_locale =
      Locale::try_from_str(&icu_locale_name).map_err(|_| errno::ENOENT)?;

    let grouping_strategy = get_grouping_strategy(&icu_locale);

    let mut options: options::DecimalFormatterOptions = Default::default();
    options.grouping_strategy = Some(grouping_strategy);

    let formatter =
      DecimalFormatter::try_new(icu_locale.clone().into(), options)
        .map_err(|_| errno::ENOENT)?;

    let mut frac = Decimal::from(1234);
    frac.multiply_pow10(-2);
    let s_frac = formatter.format(&frac);
    let s_frac = s_frac.to_string();

    let big = Decimal::from(1234567890123u128);
    let s_int = formatter.format(&big);
    let s_int = s_int.to_string();

    let decimal_point = get_decimal_point(&s_frac).ok_or(errno::ENOENT)?;
    let thousands_sep =
      get_thousands_sep(&s_int, grouping_strategy).ok_or(errno::ENOENT)?;
    let grouping = get_grouping(&icu_locale);

    self.name = Cow::Owned(locale.to_owned());
    self.decimal_point = strtocstr(&decimal_point);
    self.thousands_sep = strtocstr(&thousands_sep);
    self.grouping = grouping;

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

impl<'a> Default for NumericObject<'a> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
