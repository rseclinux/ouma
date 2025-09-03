use {
  super::{LocaleObject, is_posix_locale},
  crate::{c_int, std::errno},
  allocation::{
    borrow::{Cow, ToOwned},
    collections::BTreeMap,
    vec
  },
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_locale::Locale,
  writeable::Writeable
};

pub fn get_grouping_strategy_for_locale(
  locale: &Locale
) -> options::GroupingStrategy {
  // https://lh.2xlibre.net/values/grouping/

  if let Some(region) = locale.id.region {
    match region.as_str() {
      | "CN" | "HK" | "PH" | "SG" | "FR" | "TW" | "MT" | "NP" | "MA" | "JP" => {
        return options::GroupingStrategy::Min2;
      },
      | "PT" | "RS" | "SL" | "CU" | "NK" => {
        return options::GroupingStrategy::Never;
      },
      | _ => ()
    }
  }

  match locale.id.language.as_str() {
    | "ar" | "az" | "ckb" | "fa" | "pl" | "ja" => {
      options::GroupingStrategy::Min2
    },
    | "el" | "gl" => options::GroupingStrategy::Never,
    | _ => options::GroupingStrategy::Auto
  }
}

pub fn get_decimal_sep<'a>(s: &str) -> Option<Cow<'a, [u8]>> {
  let mut last = None;
  for (i, ch) in s.char_indices() {
    if !ch.is_numeric() && !ch.is_whitespace() {
      last = Some(i);
    }
  }
  match last {
    | Some(i) => {
      let mut buffer: vec::Vec<u8> = vec::Vec::new();

      let sep = s[i..].chars().next()?;

      let mut b = [0; 4];
      let encoded = sep.encode_utf8(&mut b).as_bytes();

      buffer.extend_from_slice(encoded);
      buffer.push(b'\0');

      Some(Cow::Owned(buffer))
    },
    | None => None
  }
}

pub fn get_thousands_sep<'a>(s: &str) -> Option<Cow<'a, [u8]>> {
  let mut buffer: vec::Vec<u8> = vec::Vec::new();

  for ch in s.chars() {
    if !ch.is_numeric() && !ch.is_whitespace() {
      let mut b = [0; 4];
      let encoded = ch.encode_utf8(&mut b).as_bytes();

      buffer.extend_from_slice(encoded);
      buffer.push(b'\0');

      return Some(Cow::Owned(buffer));
    }
  }

  None
}

pub fn get_posix_grouping<'a>(
  formatter: &DecimalFormatter
) -> Option<Cow<'a, [u8]>> {
  let mut buffer: vec::Vec<u8> = vec::Vec::new();
  let mut cur: usize = 0;

  let fmt = |n: u128| {
    let d = Decimal::from(n);
    formatter.format_to_string(&d)
  };

  let probe = fmt(123456789012345u128);
  let sep = {
    let mut counts = BTreeMap::<char, usize>::new();

    for ch in probe.chars() {
      if !ch.is_numeric() {
        *counts.entry(ch).or_default() += 1;
      }
    }

    counts.into_iter().max_by_key(|&(_, c)| c).map(|(ch, _)| ch)
  };

  let Some(sep) = sep else { return None };

  for ch in probe.chars().rev() {
    if ch == sep {
      buffer.push(cur as u8);
      cur = 0;
    } else if ch.is_numeric() {
      cur += 1;
    }
  }
  if cur > 0 {
    buffer.push(cur as u8)
  }

  let primary = buffer[0];
  let secondary = buffer.get(1).copied();

  let big = fmt(12345).contains(sep);
  let small = fmt(1234).contains(sep);
  let is_min2 = big && !small;

  let mut result: vec::Vec<u8> = vec::Vec::new();

  if is_min2 && secondary == Some(primary) {
    result.push(primary);
  } else if let Some(s) = secondary {
    result.push(primary);
    result.push(s);
  } else {
    result.push(primary);
  }
  result.push(b'\0');

  Some(Cow::Owned(result))
}

pub struct NumericObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub decimal_point: Cow<'a, [u8]>,
  pub thousands_sep: Cow<'a, [u8]>,
  pub grouping: Cow<'a, [u8]>
}

impl<'a> LocaleObject for NumericObject<'a> {
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

    let mut options: options::DecimalFormatterOptions = Default::default();
    options.grouping_strategy =
      Some(get_grouping_strategy_for_locale(&icu_locale));

    let formatter = DecimalFormatter::try_new(icu_locale.into(), options);
    let formatter = match formatter {
      | Ok(formatter) => formatter,
      | Err(_) => return Err(errno::EINVAL)
    };

    let mut frac = Decimal::from(1234);
    frac.multiply_pow10(-2);
    let s_frac = formatter.format(&frac);
    let s_frac = s_frac.write_to_string();

    // fallback to POSIX
    let decimal_point =
      get_decimal_sep(&s_frac).unwrap_or(Cow::Borrowed(&[b'.', b'\0']));

    let big = Decimal::from(1234567890123u128);
    let s_int = formatter.format(&big);
    let s_int = s_int.write_to_string();

    // fallback to POSIX
    let thousands_sep =
      get_thousands_sep(&s_int).unwrap_or(Cow::Borrowed(&[b'\0']));

    // fallback to POSIX
    let grouping =
      get_posix_grouping(&formatter).unwrap_or(Cow::Borrowed(&[b'\0']));

    self.name = Cow::Owned(locale.to_owned());
    self.decimal_point = decimal_point;
    self.thousands_sep = thousands_sep;
    self.grouping = grouping;

    Ok(self.name.as_ref())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_NUMERIC;

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
  decimal_point: Cow::Borrowed(&[b'.', b'\0']),
  thousands_sep: Cow::Borrowed(&[b'\0']),
  grouping: Cow::Borrowed(&[b'\0'])
};
