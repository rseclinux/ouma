use {
  super::{LocaleObject, canonicalize_locale, is_posix_locale},
  crate::{
    allocation::{
      borrow::ToOwned,
      collections::BTreeMap,
      string::{String, ToString}
    },
    c_int,
    support::{locale::errno, string::strtocstr}
  },
  allocation::borrow::Cow,
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_locale::Locale,
  smallvec::SmallVec
};

pub fn get_grouping_strategy_for_locale(
  locale: &Locale
) -> options::GroupingStrategy {
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

pub fn get_posix_grouping(
  formatter: &DecimalFormatter
) -> Option<SmallVec<[u8; 3]>> {
  let fmt =
    |n: u128| -> String { formatter.format(&Decimal::from(n)).to_string() };

  let probe = fmt(123_456_789_012_345_u128);

  let sep: char = {
    let mut counts = BTreeMap::<char, usize>::new();
    for ch in probe.chars() {
      if !ch.is_ascii_digit() && !ch.is_ascii_alphabetic() {
        *counts.entry(ch).or_default() += 1;
      }
    }
    let winner = counts
      .into_iter()
      .filter(|&(_, n)| n >= 2)
      .max_by_key(|&(_, n)| n)
      .map(|(ch, _)| ch);
    winner?
  };

  let mut raw: SmallVec<[u8; 3]> = SmallVec::new();
  let mut cur: u8 = 0;

  for ch in probe.chars().rev() {
    if ch == sep {
      raw.push(cur);
      cur = 0;
    } else if ch.is_ascii_digit() {
      cur = cur.saturating_add(1);
    }
  }
  if cur > 0 {
    raw.push(cur);
  }

  if raw.is_empty() {
    return None;
  }

  let probe_1234 = fmt(1_234_u128);
  let probe_12345 = fmt(12_345_u128);
  let fmt_contains_sep = |s: &str| s.contains(sep);
  let is_min2 =
    fmt_contains_sep(&probe_12345) && !fmt_contains_sep(&probe_1234);

  let primary = raw[0];
  let all_same = raw.iter().all(|&g| g == primary);

  let mut result: SmallVec<[u8; 3]> = SmallVec::new();

  if all_same {
    result.push(primary);
    if !is_min2 {
      result.push(0);
    }
  } else {
    let tail = raw[raw.len() - 1];
    let last_distinct =
      raw.iter().rposition(|&g| g != tail).map_or(0, |p| p + 1);
    for &g in &raw[..=last_distinct] {
      result.push(g);
    }
    result.push(0);
  }

  result.push(b'\0');
  Some(result)
}

pub fn get_thousands_sep(s: &str) -> Option<String> {
  if !s.chars().any(|c| c.is_ascii_punctuation() || c == '’') {
    return Some(s.to_string());
  }

  for ch in s.chars() {
    if ch.is_ascii_punctuation() || ch == '’' {
      let mut b = [0; 4];
      let encoded = ch.encode_utf8(&mut b);

      return Some(String::from(encoded));
    }
  }

  None
}

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

#[derive(Debug, Clone)]
pub struct NumericObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub decimal_point: Cow<'a, ffi::CStr>,
  pub thousands_sep: Cow<'a, ffi::CStr>,
  pub grouping: SmallVec<[u8; 3]>
}

impl<'a> NumericObject<'a> {
  #[inline]
  pub fn get_decimal_point(&self) -> Option<char> {
    self.decimal_point.to_str().ok()?.chars().next()
  }

  #[inline]
  pub fn get_thousands_sep(&self) -> Option<char> {
    self.thousands_sep.to_str().ok()?.chars().next()
  }
}

impl<'a> LocaleObject for NumericObject<'a> {
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

    self.grouping.clear();

    let mut options: options::DecimalFormatterOptions = Default::default();
    options.grouping_strategy =
      Some(get_grouping_strategy_for_locale(&icu_locale));

    let formatter = DecimalFormatter::try_new(icu_locale.into(), options)
      .map_err(|_| errno::ENOENT)?;

    let mut frac = Decimal::from(1234);
    frac.multiply_pow10(-2);
    let s_frac = formatter.format(&frac);
    let s_frac = s_frac.to_string();

    let big = Decimal::from(1234567890123u128);
    let s_int = formatter.format(&big);
    let s_int = s_int.to_string();

    let decimal_point = get_decimal_point(&s_frac).ok_or(errno::ENOENT)?;
    let thousands_sep = get_thousands_sep(&s_int).ok_or(errno::ENOENT)?;
    let grouping = get_posix_grouping(&formatter).ok_or(errno::ENOENT)?;

    self.name = Cow::Owned(locale.to_owned());
    self.decimal_point = strtocstr(&decimal_point);
    self.thousands_sep = strtocstr(&thousands_sep);
    self.grouping = grouping.into();

    Ok(self.name.as_ref())
  }

  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr {
    *self = DEFAULT_NUMERIC;

    self.name = Cow::Owned(locale.to_owned());
    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for NumericObject<'a> {
  fn default() -> Self {
    DEFAULT_NUMERIC
  }
}

pub const DEFAULT_NUMERIC: NumericObject = NumericObject {
  name: Cow::Borrowed(c"C"),
  decimal_point: Cow::Borrowed(c"."),
  thousands_sep: Cow::Borrowed(c""),
  grouping: SmallVec::new_const()
};
