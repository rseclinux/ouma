use {
  super::{
    LocaleObject,
    canonicalize_locale,
    is_posix_locale,
    numeric::{
      get_decimal_point,
      get_grouping,
      get_grouping_strategy,
      get_thousands_sep
    }
  },
  crate::{
    std::errno,
    support::string::strtocstr,
    types::{c_char, c_int}
  },
  allocation::{
    borrow::{Cow, ToOwned},
    string::{String, ToString}
  },
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_experimental::dimension::currency::{
    CurrencyType,
    formatter::CurrencyFormatter,
    options::{CurrencyFormatterOptions, CurrencyUsage}
  },
  icu_locale::Locale,
  smallvec::SmallVec
};

mod static_data;

const MINUS_SIGNS: &[char] = &[
  '\u{002D}', '\u{02D7}', '\u{0320}', '\u{2052}', '\u{207B}', '\u{208B}',
  '\u{2212}', '\u{2216}', '\u{2238}', '\u{2242}', '\u{2296}', '\u{229F}',
  '\u{2756}', '\u{2796}', '\u{293C}', '\u{2A29}', '\u{2A2A}', '\u{2A2B}',
  '\u{2A2C}', '\u{2A3A}', '\u{2A41}', '\u{2A6C}', '\u{FE63}', '\u{FF0D}'
];

#[inline]
fn normalize_for_bidi(input: &str) -> String {
  input
    .chars()
    .filter(|&c| {
      !matches!(c,
          '\u{200E}' | '\u{200F}' |
          '\u{202A}'..='\u{202E}' |
          '\u{2066}'..='\u{2069}'
      )
    })
    .collect()
}

#[inline]
fn construct_iso4217_currency_symbol(s: &str) -> SmallVec<[u8; 5]> {
  let sb = s.as_bytes();
  let mut result: SmallVec<[u8; 5]> = SmallVec::new();
  result.extend_from_slice(&[sb[0], sb[1], sb[2], b' ', b'\0']);
  result
}

#[inline]
fn get_sign_posn(
  fmt: &str,
  currency: &str
) -> Option<u8> {
  let fmt = normalize_for_bidi(fmt.trim());
  let currency = normalize_for_bidi(currency);

  if fmt.starts_with('(') && fmt.ends_with(')') {
    return Some(0);
  }

  let Some((sign, sign_char)) =
    fmt.char_indices().find(|(_, c)| MINUS_SIGNS.contains(c))
  else {
    return Some(1);
  };
  let sign_end = sign + sign_char.len_utf8();

  let Some(cur) = fmt.find(&currency) else {
    return None;
  };
  let cur_end = cur + currency.len();

  if sign == 0 {
    return Some(1);
  }

  if sign_end == fmt.len() {
    return Some(2);
  }

  if sign_end <= cur {
    return Some(3);
  }

  if cur_end <= sign {
    return Some(4);
  }

  None
}

#[inline]
fn get_cs_precedes(
  fmt: &str,
  currency: &str
) -> Option<u8> {
  let fmt = normalize_for_bidi(fmt.trim());
  let currency = normalize_for_bidi(currency);

  let Some(cur) = fmt.find(&currency) else {
    return None;
  };
  let cur_end = cur + currency.len();

  let first_numeric = fmt.find(char::is_numeric).unwrap_or(0);

  if cur_end <= first_numeric { Some(1) } else { Some(0) }
}

#[inline]
fn get_sep_by_space(
  fmt: &str,
  currency: &str
) -> Option<u8> {
  let fmt = normalize_for_bidi(fmt.trim());
  let currency = normalize_for_bidi(currency);
  let fmt_lc = fmt.to_lowercase().to_string();
  let currency_lc = currency.to_lowercase().to_string();
  let cs_start = fmt_lc.find(&currency_lc)?;
  let cs_end = cs_start + currency_lc.len();
  let first_digit = fmt.find(|c: char| c.is_ascii_digit())?;
  let cs_precedes = cs_start < first_digit;

  let sign = fmt.char_indices().find_map(|(i, c)| {
    (MINUS_SIGNS.contains(&c) && (i < cs_start || i >= cs_end))
      .then_some((i, c))
  });

  let cs_side = |left: usize, right: usize| -> bool {
    let neighbor = if cs_precedes {
      fmt[right..].chars().next()
    } else {
      fmt[..left].chars().last()
    };
    neighbor.map_or(false, |c| c.is_whitespace())
  };

  let Some((s, sc)) = sign else {
    return Some(if cs_side(cs_start, cs_end) { 1 } else { 0 });
  };
  let s_end = s + sc.len_utf8();

  let (gap_lo, gap_hi) =
    if cs_start < s { (cs_end, s) } else { (s_end, cs_start) };
  let gap = &fmt[gap_lo..gap_hi];

  if !gap.chars().any(|c| c.is_ascii_digit()) {
    if gap.chars().any(|c| c.is_whitespace()) {
      return Some(2);
    }

    let group_start = cs_start.min(s);
    let group_end = cs_end.max(s_end);
    return Some(if cs_side(group_start, group_end) { 1 } else { 0 });
  }

  if cs_side(cs_start, cs_end) {
    return Some(1);
  }

  let sign_precedes_value = s < first_digit;
  let sign_neighbor = if sign_precedes_value {
    fmt[s_end..].chars().next()
  } else {
    fmt[..s].chars().last()
  };

  if sign_neighbor.map_or(false, |c| c.is_whitespace()) {
    return Some(2);
  }

  Some(0)
}

#[inline]
fn get_currency(
  locale: &Locale,
  currency: &CurrencyType
) -> Result<String, i32> {
  let mut options = CurrencyFormatterOptions::default();
  options.usage = CurrencyUsage::Standard;

  let formatter = CurrencyFormatter::try_new_compact_symbol(
    locale.into(),
    currency.clone(),
    options
  )
  .map_err(|_| errno::ENOENT)?;

  let mini_fmt = |n: u8| -> String {
    let d = Decimal::from(n);
    let f = formatter.format_fixed_decimal(&d);
    f.to_string()
  };

  let fmt = mini_fmt(100);

  let clean: String = fmt.chars().filter(|&ch| !ch.is_whitespace()).collect();

  let result: String = clean
    .chars()
    .filter(|&ch| {
      !(ch.is_numeric() || MINUS_SIGNS.contains(&ch) || ch.is_numeric())
    })
    .collect();

  Ok(result.trim().to_string())
}

#[derive(Debug, Clone)]
pub struct MonetaryObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub mon_decimal_point: Cow<'a, ffi::CStr>,
  pub mon_thousands_sep: Cow<'a, ffi::CStr>,
  pub mon_grouping: SmallVec<[u8; 3]>,
  pub positive_sign: Cow<'a, ffi::CStr>,
  pub negative_sign: Cow<'a, ffi::CStr>,
  pub currency_symbol: Cow<'a, ffi::CStr>,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub n_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_curr_symbol: SmallVec<[u8; 5]>,
  pub int_frac_digits: c_char,
  pub int_p_cs_precedes: c_char,
  pub int_n_cs_precedes: c_char,
  pub int_p_sep_by_space: c_char,
  pub int_n_sep_by_space: c_char,
  pub int_p_sign_posn: c_char,
  pub int_n_sign_posn: c_char
}

impl<'a> MonetaryObject<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      name: Cow::Borrowed(c"C"),
      mon_decimal_point: Cow::Borrowed(c""),
      mon_thousands_sep: Cow::Borrowed(c""),
      mon_grouping: SmallVec::new_const(),
      positive_sign: Cow::Borrowed(c""),
      negative_sign: Cow::Borrowed(c""),
      currency_symbol: Cow::Borrowed(c""),
      frac_digits: c_char::MAX,
      p_cs_precedes: c_char::MAX,
      n_cs_precedes: c_char::MAX,
      p_sep_by_space: c_char::MAX,
      n_sep_by_space: c_char::MAX,
      p_sign_posn: c_char::MAX,
      n_sign_posn: c_char::MAX,
      int_curr_symbol: SmallVec::new_const(),
      int_frac_digits: c_char::MAX,
      int_p_cs_precedes: c_char::MAX,
      int_n_cs_precedes: c_char::MAX,
      int_p_sep_by_space: c_char::MAX,
      int_n_sep_by_space: c_char::MAX,
      int_p_sign_posn: c_char::MAX,
      int_n_sign_posn: c_char::MAX
    }
  }

  #[inline]
  pub fn get_decimal_point(&self) -> Option<char> {
    self.mon_decimal_point.to_str().ok()?.chars().nth(0)
  }

  #[inline]
  pub fn get_thousands_sep(&self) -> Option<char> {
    self.mon_thousands_sep.to_str().ok()?.chars().nth(0)
  }

  #[inline]
  pub fn get_negative_sign(&self) -> Option<char> {
    self.negative_sign.to_str().ok()?.chars().nth(0)
  }

  #[inline]
  pub fn get_positive_sign(&self) -> Option<char> {
    self.positive_sign.to_str().ok()?.chars().nth(0)
  }
}

impl<'a> LocaleObject for MonetaryObject<'a> {
  #[inline]
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    self.mon_grouping.clear();
    self.int_curr_symbol.clear();

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

    self.mon_decimal_point = strtocstr(&decimal_point);
    self.mon_thousands_sep = strtocstr(&thousands_sep);
    self.mon_grouping = grouping;

    self.positive_sign = Cow::Borrowed(c"");
    self.negative_sign = Cow::Borrowed(c"-");

    let region = icu_locale.id.region.and_then(|r| Some(r.to_string()));

    let iso4217_currency =
      static_data::get_iso4217_currency_from_region(region)
        .ok_or(errno::ENOENT)?;

    let currency_code = CurrencyType::try_from_str(&iso4217_currency)
      .map_err(|_| errno::ENOENT)?;

    let currency =
      get_currency(&icu_locale, &currency_code).map_err(|_| errno::ENOENT)?;

    let mut options = CurrencyFormatterOptions::default();
    options.usage = CurrencyUsage::Standard;

    let formatter = CurrencyFormatter::try_new_symbol(
      icu_locale.clone().into(),
      currency_code,
      options
    )
    .map_err(|_| errno::ENOENT)?;

    let int_formatter = CurrencyFormatter::try_new_code(
      icu_locale.clone().into(),
      currency_code,
      options
    )
    .map_err(|_| errno::ENOENT)?;

    let fmt_func = |n: i64| -> String {
      let d = Decimal::from(n);
      let f = formatter.format_fixed_decimal(&d);
      f.to_string()
    };

    let int_fmt_func = |n: i64| -> String {
      let d = Decimal::from(n);
      let f = int_formatter.format_fixed_decimal(&d);
      f.to_string()
    };

    let p_fmt = fmt_func(123456789012345);
    let n_fmt = fmt_func(-123456789012345);
    let int_p_fmt = int_fmt_func(123456789012345);
    let int_n_fmt = int_fmt_func(-123456789012345);

    let p_cs_precedes =
      get_cs_precedes(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_cs_precedes =
      get_cs_precedes(&n_fmt, &currency).ok_or(errno::ENOENT)?;
    self.p_cs_precedes = p_cs_precedes as c_char;
    self.n_cs_precedes = n_cs_precedes as c_char;

    let int_p_cs_precedes =
      get_cs_precedes(&int_p_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    let int_n_cs_precedes =
      get_cs_precedes(&int_n_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    self.int_p_cs_precedes = int_p_cs_precedes as c_char;
    self.int_n_cs_precedes = int_n_cs_precedes as c_char;

    let p_sign_posn = get_sign_posn(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_sign_posn = get_sign_posn(&n_fmt, &currency).ok_or(errno::ENOENT)?;
    self.p_sign_posn = p_sign_posn as c_char;
    self.n_sign_posn = n_sign_posn as c_char;

    let int_p_sign_posn =
      get_sign_posn(&int_p_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    let int_n_sign_posn =
      get_sign_posn(&int_n_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    self.int_p_sign_posn = int_p_sign_posn as c_char;
    self.int_n_sign_posn = int_n_sign_posn as c_char;

    let p_sep_by_space =
      get_sep_by_space(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_sep_by_space =
      get_sep_by_space(&n_fmt, &currency).ok_or(errno::ENOENT)?;
    self.p_sep_by_space = p_sep_by_space as c_char;
    self.n_sep_by_space = n_sep_by_space as c_char;

    let int_p_sep_by_space =
      get_sep_by_space(&int_p_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    let int_n_sep_by_space =
      get_sep_by_space(&int_n_fmt, iso4217_currency).ok_or(errno::ENOENT)?;
    self.int_p_sep_by_space = int_p_sep_by_space as c_char;
    self.int_n_sep_by_space = int_n_sep_by_space as c_char;

    let frac_digits = static_data::get_frac_digits(&icu_locale);
    self.frac_digits = frac_digits as c_char;
    self.int_frac_digits = frac_digits as c_char;

    let int_curr_symbol = construct_iso4217_currency_symbol(&iso4217_currency);
    self.int_curr_symbol = int_curr_symbol;

    self.currency_symbol = strtocstr(&currency);

    self.name = Cow::Owned(locale.to_owned());

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

impl<'a> Default for MonetaryObject<'a> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
