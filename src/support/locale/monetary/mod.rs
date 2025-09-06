use {
  super::{LocaleObject, is_posix_locale},
  crate::{
    allocation::{borrow::ToOwned, string::ToString, vec::Vec},
    c_char,
    c_int,
    std::errno
  },
  allocation::{borrow::Cow, string::String},
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_experimental::dimension::currency::{
    CurrencyCode,
    formatter::CurrencyFormatter,
    options::{CurrencyFormatterOptions, Width}
  },
  icu_locale::Locale,
  tinystr::*,
  writeable::Writeable
};

#[inline]
fn is_sign(c: char) -> bool {
  matches!(c, |'−'| '‐' | '-' | '‒' | '–' | '—' | '﹣' | '－')
}

#[inline]
fn is_space(c: char) -> bool {
  c.is_whitespace() || matches!(c, '\u{00A0}' | '\u{202F}' | '\u{2009}')
}

#[inline]
fn is_group_sep(c: char) -> bool {
  matches!(
    c,
    ',' |
      '.' |
      '\'' |
      '’' |
      '˙' |
      '\u{066C}' |
      '\u{00A0}' |
      '\u{202F}' |
      '\u{2009}'
  )
}

#[inline]
fn is_digitish(
  c: char,
  decimal_point: char
) -> bool {
  c.is_numeric() || c == decimal_point || is_group_sep(c) || is_space(c)
}

#[inline]
pub fn separator(s: &str) -> Option<char> {
  let mut last = None;
  for (i, ch) in s.char_indices() {
    if !ch.is_numeric() && !ch.is_whitespace() {
      last = Some(i);
    }
  }
  match last {
    | Some(i) => s[i..].chars().next(),
    | None => None
  }
}

#[inline]
fn extract_region(locale: &str) -> Option<String> {
  let core = locale.split(['.', '@']).next().unwrap_or(locale);
  for part in core.split(['-', '_']) {
    if part.len() == 2 && part.chars().all(|c| c.is_ascii_uppercase()) {
      return Some(part.to_string());
    }
    if part.len() == 3 && part.chars().all(|c| c.is_ascii_digit()) {
      return Some(part.to_string());
    }
  }
  None
}

#[inline]
pub fn get_iso4217_currency_from_region(
  region: Option<String>
) -> Option<&'static str> {
  // https://lh.2xlibre.net/values/int_curr_symbol/
  Some(match region?.as_str() {
    | "EU" | "AT" | "BE" | "CY" | "EE" | "FI" | "FR" | "DE" | "GR" | "IE" |
    "IT" | "LV" | "LT" | "LU" | "MT" | "NL" | "PT" | "SK" | "SI" | "ES" |
    "MC" | "SM" | "VA" | "AD" | "ME" | "XK" | "HR" => "EUR",
    | "BG" => "BGN",
    | "RO" => "RON",
    | "HU" => "HUF",
    | "CZ" => "CZK",
    | "PL" => "PLN",
    | "GB" | "IM" | "JE" | "GG" => "GBP",
    | "GI" => "GIP",
    | "SE" => "SEK",
    | "NO" | "SJ" => "NOK",
    | "DK" | "FO" | "GL" => "DKK",
    | "IS" => "ISK",
    | "CH" | "LI" => "CHF",
    | "UA" => "UAH",
    | "RU" => "RUB",
    | "RS" => "RSD",
    | "BA" => "BAM",
    | "AL" => "ALL",
    | "MK" => "MKD",
    | "US" | "PR" | "SV" | "PA" | "FM" | "MH" | "PW" | "VG" | "TC" | "AS" |
    "GU" | "MP" | "UM" | "VI" | "BQ" => "USD",
    | "CA" => "CAD",
    | "MX" => "MXN",
    | "DO" => "DOP",
    | "CU" => "CUP",
    | "HT" => "HTG",
    | "JM" => "JMD",
    | "TT" => "TTD",
    | "BS" => "BSD",
    | "BB" => "BBD",
    | "AG" | "DM" | "GD" | "LC" | "VC" | "KN" => "XCD",
    | "AI" | "MS" => "XCD",
    | "BZ" => "BZD",
    | "CR" => "CRC",
    | "GT" => "GTQ",
    | "HN" => "HNL",
    | "NI" => "NIO",
    | "AR" => "ARS",
    | "BO" => "BOB",
    | "BR" => "BRL",
    | "CL" => "CLP",
    | "CO" => "COP",
    | "EC" => "USD",
    | "PE" => "PEN",
    | "PY" => "PYG",
    | "UY" => "UYU",
    | "VE" => "VEF",
    | "AE" => "AED",
    | "SA" => "SAR",
    | "QA" => "QAR",
    | "BH" => "BHD",
    | "KW" => "KWD",
    | "OM" => "OMR",
    | "YE" => "YER",
    | "IQ" => "IQD",
    | "IR" => "IRR",
    | "JO" => "JOD",
    | "LB" => "LBP",
    | "SY" => "SYP",
    | "IL" => "ILS",
    | "TR" => "TRY",
    | "DZ" => "DZD",
    | "EG" => "EGP",
    | "MA" => "MAD",
    | "TN" => "TND",
    | "LY" => "LYD",
    | "SD" => "SDG",
    | "SS" => "SSP",
    | "ET" => "ETB",
    | "ER" => "ERN",
    | "DJ" => "DJF",
    | "SO" => "SOS",
    | "KE" => "KES",
    | "UG" => "UGX",
    | "TZ" => "TZS",
    | "RW" => "RWF",
    | "BI" => "BIF",
    | "CD" => "CDF",
    | "CG" | "GA" | "GQ" | "CF" | "TD" | "CM" => "XAF",
    | "NE" | "BF" | "BJ" | "ML" | "SN" | "TG" | "GW" | "CI" => "XOF",
    | "GM" => "GMD",
    | "LR" => "LRD",
    | "SL" => "SLL",
    | "GH" => "GHS",
    | "NG" => "NGN",
    | "ZA" => "ZAR",
    | "NA" => "NAD",
    | "BW" => "BWP",
    | "ZM" => "ZMW",
    | "ZW" => "USD",
    | "MW" => "MWK",
    | "MZ" => "MZN",
    | "AO" => "AOA",
    | "SZ" => "SZL",
    | "LS" => "LSL",
    | "MR" => "MRU",
    | "RE" | "YT" | "GP" | "MQ" | "GF" | "PM" | "BL" | "MF" => "EUR",
    | "AF" => "AFN",
    | "PK" => "PKR",
    | "IN" => "INR",
    | "BD" => "BDT",
    | "LK" => "LKR",
    | "NP" => "NPR",
    | "BT" => "BTN",
    | "MM" => "MMK",
    | "KZ" => "KZT",
    | "KG" => "KGS",
    | "UZ" => "UZS",
    | "TJ" => "TJS",
    | "TM" => "TMM",
    | "JP" => "JPY",
    | "CN" => "CNY",
    | "HK" => "HKD",
    | "MO" => "MOP",
    | "TW" => "TWD",
    | "KR" => "KRW",
    | "MN" => "MNT",
    | "KH" => "KHR",
    | "LA" => "LAK",
    | "TH" => "THB",
    | "VN" => "VND",
    | "MY" => "MYR",
    | "SG" => "SGD",
    | "PH" => "PHP",
    | "ID" => "IDR",
    | "BN" => "BND",
    | "TL" => "USD",
    | "AU" => "AUD",
    | "NZ" => "NZD",
    | "FJ" => "FJD",
    | "PG" => "PGK",
    | "WS" => "WST",
    | "TO" => "TOP",
    | "SB" => "SBD",
    | "VU" => "VUV",
    | "PF" | "NC" | "WF" => "XPF",
    | "CK" | "NU" | "TK" => "NZD",
    | "KI" | "TV" | "NR" => "AUD",
    | "SX" | "CW" => "ANG",
    | "KY" => "KYD",
    | "BM" => "BMD",
    | "AX" => "EUR",
    | "CV" => "CVE",
    | "KM" => "KMF",
    | "GN" => "GNF",
    | "ST" => "STD",
    | "SH" => "SHP",
    | "MV" => "MVR",
    | "AM" => "AMD",
    | "AZ" => "AZN",
    | "GE" => "GEL",
    | "BY" => "BYR",
    | "GY" => "GYD",
    | "SR" => "SRD",
    | "FK" => "FKP",
    | _ => return None
  })
}

fn get_frac_digits(locale: &str) -> c_char {
  // https://lh.2xlibre.net/values/frac_digits/
  const ZERO_FRAC: &[&str] = &["IS", "JP", "KR", "IR", "AF", "VN", "ER"];
  const THREE_FRAC: &[&str] = &[
    "AE", "BH", "DZ", "EG", "IQ", "JO", "KW", "LB", "LY", "MA", "OM", "QA",
    "SD", "SS", "SY", "TN", "YE", "BT", "AL"
  ];

  let region = extract_region(locale).unwrap_or_default();
  if ZERO_FRAC.contains(&region.as_str()) {
    0
  } else if THREE_FRAC.contains(&region.as_str()) {
    3
  } else {
    2
  }
}

fn get_currency_symbol(s: &str) -> String {
  let clean: String = s.chars().filter(|&ch| !ch.is_whitespace()).collect();

  let from_end: String = clean
    .chars()
    .rev()
    .take_while(|&ch| {
      !(ch.is_numeric() || is_sign(ch) || ch == '.' || ch == ',' || ch == ' ')
    })
    .collect::<Vec<char>>()
    .into_iter()
    .rev()
    .collect();

  if !from_end.is_empty() {
    return from_end;
  }

  let mut sym: String = clean
    .chars()
    .take_while(|&ch| {
      !(ch.is_numeric() || is_sign(ch) || ch == ',' || ch == ' ')
    })
    .collect();

  if !sym.is_empty() && sym.chars().all(|c| c.is_alphabetic()) {
    if clean.as_bytes().get(sym.len()) == Some(&b'.') {
      sym.push('.');
    }
  }

  sym
}

fn get_currency_precedes(
  s: &str,
  currency: &str
) -> c_char {
  let Some(start) = s.find(currency) else { return 0 };
  let end = start + currency.len();

  {
    let mut it = s[end..].chars();

    while let Some(c) = it.clone().next() {
      if is_space(c) {
        it.next();
      } else {
        break;
      }
    }
    if let Some(c) = it.clone().next() {
      if is_sign(c) {
        it.next();
      }
    }
    while let Some(c) = it.clone().next() {
      if is_space(c) {
        it.next();
      } else {
        break;
      }
    }
    if let Some(c) = it.clone().next() {
      if c.is_numeric() {
        return 1;
      }
    }
  }

  {
    let mut it = s[..start].chars().rev();

    while let Some(c) = it.clone().next() {
      if is_space(c) {
        it.next();
      } else {
        break;
      }
    }
    if let Some(c) = it.clone().next() {
      if is_sign(c) {
        it.next();
      }
    }
    while let Some(c) = it.clone().next() {
      if is_space(c) {
        it.next();
      } else {
        break;
      }
    }
    if let Some(c) = it.clone().next() {
      if c.is_numeric() {
        return 0;
      }
    }
  }

  0
}

fn get_currency_space_separation(
  s: &str,
  currency: &str,
  decimal_point: char
) -> c_char {
  let Some(start) = s.find(currency) else { return 0 };
  let end = start + currency.len();

  {
    let mut it = s[end..].chars().peekable();

    let mut saw_leading_space = false;
    while matches!(it.peek(), Some(&c) if is_space(c)) {
      saw_leading_space = true;
      it.next();
    }

    let sign_present = matches!(it.peek(), Some(&c) if is_sign(c));
    if sign_present {
      it.next();
    }

    let mut spaces_after_sign = false;
    while matches!(it.peek(), Some(&c) if is_space(c)) {
      spaces_after_sign = true;
      it.next();
    }

    if matches!(it.peek(), Some(&c) if c.is_numeric()) {
      let mut saw_internal_space = false;
      while let Some(&c) = it.peek() {
        if is_digitish(c, decimal_point) {
          if is_space(c) {
            saw_internal_space = true;
          }
          it.next();
        } else {
          break;
        }
      }

      if sign_present && saw_leading_space && !spaces_after_sign {
        return 2;
      }

      if saw_leading_space || spaces_after_sign || saw_internal_space {
        return 1;
      }
    }
  }

  {
    let mut it = s[..start].chars().rev().peekable();

    let mut saw_trailing_space = false;
    while matches!(it.peek(), Some(&c) if is_space(c)) {
      saw_trailing_space = true;
      it.next();
    }

    if !matches!(it.peek(), Some(&c) if c.is_numeric()) {
      return 0;
    }

    let mut saw_internal_space = false;
    while let Some(&c) = it.peek() {
      if is_digitish(c, decimal_point) {
        it.next();
      } else if is_space(c) {
        saw_internal_space = true;
        it.next();
      } else {
        break;
      }
    }

    if saw_trailing_space || saw_internal_space {
      return 1;
    }
  }

  0
}

fn get_currency_sign_positions(
  formatted_string: &str,
  currency: &str
) -> c_char {
  let trimmed = formatted_string.trim();

  if trimmed.starts_with('(') && trimmed.ends_with(')') {
    return 0;
  }

  let currency_symbols: Vec<char> = currency.chars().collect();

  let currency_pos =
    trimmed.chars().position(|c| currency_symbols.contains(&c));

  let has_negative = trimmed.contains('-');

  if has_negative {
    let negative_pos = trimmed.find('-').unwrap();

    let n_sign_posn = if let Some(_) = currency_pos {
      let curr_char_pos =
        trimmed.chars().take_while(|&c| !currency_symbols.contains(&c)).count();

      if negative_pos == 0 {
        if curr_char_pos == 0 { 3 } else { 1 }
      } else {
        if negative_pos < curr_char_pos {
          3
        } else if negative_pos > curr_char_pos {
          4
        } else {
          1
        }
      }
    } else {
      if negative_pos == 0 { 1 } else { 2 }
    };

    n_sign_posn
  } else {
    let p_sign_posn = if let Some(_) = currency_pos {
      let curr_char_pos =
        trimmed.chars().take_while(|&c| !currency_symbols.contains(&c)).count();

      if curr_char_pos == 0 { 4 } else { 1 }
    } else {
      1
    };

    p_sign_posn
  }
}

fn construct_currency_symbol(s: &str) -> Vec<u8> {
  let mut result = Vec::with_capacity(s.len() + 1);
  result.extend_from_slice(s.as_bytes());
  result.push(b'\0');
  result
}

fn construct_iso4217_currency_symbol(s: &str) -> Vec<u8> {
  let sb = s.as_bytes();
  let mut out = Vec::with_capacity(5);
  out.extend_from_slice(&[sb[0], sb[1], sb[2], b' ', b'\0']);
  out
}

pub struct MonetaryObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub mon_decimal_point: Cow<'a, [u8]>,
  pub mon_thousands_sep: Cow<'a, [u8]>,
  pub mon_grouping: Cow<'a, [u8]>,
  pub positive_sign: Cow<'a, [u8]>,
  pub negative_sign: Cow<'a, [u8]>,
  pub currency_symbol: Cow<'a, [u8]>,
  pub frac_digits: c_char,
  pub p_cs_precedes: c_char,
  pub n_cs_precedes: c_char,
  pub p_sep_by_space: c_char,
  pub n_sep_by_space: c_char,
  pub p_sign_posn: c_char,
  pub n_sign_posn: c_char,
  pub int_curr_symbol: Cow<'a, [u8]>,
  pub int_frac_digits: c_char,
  pub int_p_cs_precedes: c_char,
  pub int_n_cs_precedes: c_char,
  pub int_p_sep_by_space: c_char,
  pub int_n_sep_by_space: c_char,
  pub int_p_sign_posn: c_char,
  pub int_n_sign_posn: c_char
}

impl<'a> LocaleObject for MonetaryObject<'a> {
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

    let region = extract_region(lang);
    let iso4217_currency = match get_iso4217_currency_from_region(region) {
      | Some(iso4217_currency) => iso4217_currency,
      | None => return Err(errno::EINVAL)
    };
    let currency = match TinyAsciiStr::<3>::try_from_str(iso4217_currency) {
      | Ok(currency) => currency,
      | Err(_) => return Err(errno::EINVAL)
    };

    let mut options: options::DecimalFormatterOptions = Default::default();
    options.grouping_strategy =
      Some(super::numeric::get_grouping_strategy_for_locale(&icu_locale));

    let formatter =
      DecimalFormatter::try_new(icu_locale.clone().into(), options);
    let formatter = match formatter {
      | Ok(formatter) => formatter,
      | Err(_) => return Err(errno::EINVAL)
    };

    let mut frac = Decimal::from(1234);
    frac.multiply_pow10(-2);
    let s_frac = formatter.format(&frac);
    let s_frac = s_frac.write_to_string();

    // fallback to POSIX
    let mon_decimal_point = super::numeric::get_decimal_sep(&s_frac)
      .unwrap_or(Cow::Borrowed(&[b'.', b'\0']));

    let big = Decimal::from(1234567890123u128);
    let s_int = formatter.format(&big);
    let s_int = s_int.write_to_string();

    // fallback to POSIX
    let mon_thousands_sep = super::numeric::get_thousands_sep(&s_int)
      .unwrap_or(Cow::Borrowed(&[b'\0']));

    // fallback to POSIX
    let mon_grouping = super::numeric::get_posix_grouping(&formatter)
      .unwrap_or(Cow::Borrowed(&[b'\0']));

    let currency_code = CurrencyCode(currency);

    let mut options = CurrencyFormatterOptions::default();
    options.width = Width::Short;

    let currency_formatter =
      CurrencyFormatter::try_new(icu_locale.clone().into(), options);
    let currency_formatter = match currency_formatter {
      | Ok(currency_formatter) => currency_formatter,
      | Err(_) => return Err(errno::EINVAL)
    };

    let fmt = |n: i128| {
      let d = Decimal::from(n);
      let f = currency_formatter.format_fixed_decimal(&d, currency_code);
      f.to_string()
    };

    let p_fmt = fmt(1234567890123456789);
    let n_fmt = fmt(-1234567890123456789);

    let currency_symbol = get_currency_symbol(&n_fmt);
    let separator = separator(&n_fmt).unwrap_or('.'); // Default to POSIX

    let frac_digits = get_frac_digits(lang);
    let p_cs_precedes = get_currency_precedes(&p_fmt, &currency_symbol);
    let n_cs_precedes = get_currency_precedes(&n_fmt, &currency_symbol);
    let p_sep_by_space =
      get_currency_space_separation(&p_fmt, &currency_symbol, separator);
    let n_sep_by_space =
      get_currency_space_separation(&n_fmt, &currency_symbol, separator);
    let p_sign_posn = get_currency_sign_positions(&p_fmt, &currency_symbol);
    let n_sign_posn = get_currency_sign_positions(&n_fmt, &currency_symbol);

    let curr_sym = construct_currency_symbol(&currency_symbol);
    let int_curr_sym = construct_iso4217_currency_symbol(iso4217_currency);

    self.name = Cow::Owned(locale.to_owned());
    self.mon_decimal_point = mon_decimal_point;
    self.mon_thousands_sep = mon_thousands_sep;
    self.mon_grouping = mon_grouping;
    self.positive_sign = Cow::Borrowed(&[b'\0']);
    self.negative_sign = Cow::Borrowed(&[b'-', b'\0']);
    self.currency_symbol = Cow::Owned(curr_sym);
    self.frac_digits = frac_digits;
    self.p_cs_precedes = p_cs_precedes;
    self.n_cs_precedes = n_cs_precedes;
    self.p_sep_by_space = p_sep_by_space;
    self.n_sep_by_space = n_sep_by_space;
    self.p_sign_posn = p_sign_posn;
    self.n_sign_posn = n_sign_posn;
    self.int_curr_symbol = Cow::Owned(int_curr_sym);
    self.int_frac_digits = frac_digits;
    self.int_p_cs_precedes = p_cs_precedes;
    self.int_n_cs_precedes = n_cs_precedes;
    self.int_p_sep_by_space = p_sep_by_space;
    self.int_n_sep_by_space = n_sep_by_space;
    self.int_p_sign_posn = p_sign_posn;
    self.int_n_sign_posn = n_sign_posn;

    Ok(self.name.as_ref())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_MONETARY;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for MonetaryObject<'a> {
  fn default() -> Self {
    DEFAULT_MONETARY
  }
}

pub const DEFAULT_MONETARY: MonetaryObject = MonetaryObject {
  name: Cow::Borrowed(c"C"),
  mon_decimal_point: Cow::Borrowed(&[b'.', b'\0']),
  mon_thousands_sep: Cow::Borrowed(&[b'\0']),
  mon_grouping: Cow::Borrowed(&[b'\0']),
  positive_sign: Cow::Borrowed(&[b'\0']),
  negative_sign: Cow::Borrowed(&[b'\0']),
  currency_symbol: Cow::Borrowed(&[b'\0']),
  frac_digits: c_char::MAX,
  p_cs_precedes: c_char::MAX,
  n_cs_precedes: c_char::MAX,
  p_sep_by_space: c_char::MAX,
  n_sep_by_space: c_char::MAX,
  p_sign_posn: c_char::MAX,
  n_sign_posn: c_char::MAX,
  int_curr_symbol: Cow::Borrowed(&[b'\0']),
  int_frac_digits: c_char::MAX,
  int_p_cs_precedes: c_char::MAX,
  int_n_cs_precedes: c_char::MAX,
  int_p_sep_by_space: c_char::MAX,
  int_n_sep_by_space: c_char::MAX,
  int_p_sign_posn: c_char::MAX,
  int_n_sign_posn: c_char::MAX
};
