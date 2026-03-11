use {
  super::{
    LocaleObject,
    is_posix_locale,
    numeric::{
      get_decimal_point,
      get_grouping_strategy_for_locale,
      get_posix_grouping,
      get_thousands_sep
    }
  },
  crate::{
    allocation::{
      borrow::ToOwned,
      string::{String, ToString}
    },
    c_char,
    c_int,
    support::{locale::errno, string::strtocstr}
  },
  allocation::borrow::Cow,
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal, options},
  icu_experimental::dimension::currency::{
    CurrencyCode,
    formatter::CurrencyFormatter,
    options::{CurrencyFormatterOptions, Width}
  },
  icu_locale::Locale,
  smallvec::SmallVec,
  tinystr::*,
  unicode_normalization::UnicodeNormalization
};

mod static_data;

#[derive(Default, Debug, Clone, Copy)]
struct Token {
  pub start: usize,
  pub end: usize
}

fn union(
  a: Token,
  b: Token
) -> Token {
  Token { start: a.start.min(b.start), end: a.end.max(b.end) }
}

#[inline]
fn is_bidi_control(c: char) -> bool {
  matches!(
    c,
    '\u{061C}' |
      '\u{200E}' |
      '\u{200F}' |
      '\u{202A}' |
      '\u{202B}' |
      '\u{202C}' |
      '\u{202D}' |
      '\u{202E}' |
      '\u{2066}' |
      '\u{2067}' |
      '\u{2068}' |
      '\u{2069}'
  )
}

#[inline]
fn normalize_for_bidi(input: &str) -> String {
  let stripped: String =
    input.chars().filter(|&c| !is_bidi_control(c)).collect();

  stripped.nfkc().collect()
}

fn is_sign(ch: char) -> bool {
  match ch {
    | '-' | '−' | '－' | '﹣' | '+' | '＋' => true,
    | _ => false
  }
}

fn extract_currency(s: &str) -> String {
  let mut punct_at_the_end = false;

  let clean: String = s.chars().filter(|&ch| !ch.is_whitespace()).collect();

  let rev: String = clean
    .chars()
    .rev()
    .filter(|&ch| {
      !(ch.is_numeric() || is_sign(ch) || ch == '\'' || ch == ',' || ch == ' ')
    })
    .collect();

  if let Some(c) = s.chars().rev().nth(0) &&
    c == '.'
  {
    punct_at_the_end = true;
  }

  let mut result: String =
    rev.chars().rev().filter(|&ch| !(ch == '.')).collect();

  if punct_at_the_end {
    result.push('.');
  }

  result.trim().to_string()
}

fn extract_region(locale: &str) -> Option<String> {
  let core = locale.split(['.', '@']).next().unwrap_or(locale);
  for part in core.split(['-', '_']) {
    if part.len() == 2 && part.chars().all(|c| c.is_uppercase()) {
      return Some(part.to_string());
    }
    if part.len() == 3 && part.chars().all(|c| c.is_numeric()) {
      return Some(part.to_string());
    }
  }
  None
}

fn find_sign_token(s: &str) -> Option<Token> {
  for (i, ch) in s.char_indices() {
    let is_sign = is_sign(ch);

    if is_sign {
      let end = i + ch.len_utf8();

      return Some(Token { start: i, end: end });
    }
  }

  None
}

fn find_substring_range(
  haystack: &str,
  needle: &str
) -> Option<Token> {
  haystack
    .find(needle)
    .map(|start| Token { start: start, end: start + needle.len() })
}

fn find_digit_span(s: &str) -> Option<Token> {
  let mut first: Option<usize> = None;
  let mut last: Option<usize> = None;

  for (i, ch) in s.char_indices() {
    if ch.is_numeric() {
      if first.is_none() {
        first = Some(i);
      }

      last = Some(i + ch.len_utf8());
    }
  }

  match (first, last) {
    | (Some(a), Some(b)) if a < b => Some(Token { start: a, end: b }),
    | _ => None
  }
}

fn is_wrapped_in_parens(s: &str) -> bool {
  let s = s.trim();
  s.starts_with('(') && s.ends_with(')') && s.len() >= 2
}

fn between<'a>(
  s: &'a str,
  a: Token,
  b: Token
) -> Option<&'a str> {
  if a.end <= b.start {
    Some(&s[a.end..b.start])
  } else if b.end <= a.start {
    Some(&s[b.end..a.start])
  } else {
    None
  }
}

fn is_ws_only_between(
  s: &str,
  a: Token,
  b: Token
) -> bool {
  match between(s, a, b) {
    | Some(m) => m.chars().all(|c| c.is_whitespace()),
    | None => false
  }
}

fn is_space_between(
  s: &str,
  a: Token,
  b: Token
) -> bool {
  match between(s, a, b) {
    | Some(m) => !m.is_empty() && m.chars().all(|c| c.is_whitespace()),
    | None => false
  }
}

fn no_spaces_between_adj_parts(
  s: &str,
  sign: Option<Token>,
  cs: Token,
  val: Token
) -> bool {
  let mut parts = SmallVec::<[Token; 3]>::new();
  if let Some(sig) = sign {
    parts.push(sig);
  }
  parts.push(cs);
  parts.push(val);

  parts.sort_by_key(|p| p.start);

  for w in parts.windows(2) {
    let a = &w[0];
    let b = &w[1];

    let Some(mid) = between(s, *a, *b) else {
      return false;
    };

    if mid.chars().any(|c| c.is_whitespace()) {
      return false;
    }
  }

  true
}

fn detect_monetary_sign_posn(
  fmt: &str,
  currency: &str
) -> Option<c_char> {
  if currency.is_empty() {
    return None;
  }

  let s = fmt.trim();
  if s.is_empty() {
    return None;
  }

  let cur = find_substring_range(s, currency)?;
  let qty = find_digit_span(s)?;

  if is_wrapped_in_parens(s) {
    let inner = s[1..s.len() - 1].trim();

    if inner.contains(currency) && inner.chars().any(|c| c.is_numeric()) {
      return Some(0);
    }
  }

  let sign = find_sign_token(s)?;

  if sign.end == cur.start {
    return Some(3);
  }
  if cur.end == sign.start {
    return Some(4);
  }

  if sign.start < qty.start {
    return Some(1);
  }
  if sign.start >= qty.end {
    return Some(2);
  }

  None
}

fn detect_monetary_cs_precedes(
  fmt: &str,
  currency: &str
) -> Option<c_char> {
  let s = fmt.trim();

  if s.is_empty() || currency.is_empty() {
    return None;
  }

  let cs = find_substring_range(s, currency)?;
  let v = find_digit_span(s)?;

  if cs.start < v.start { Some(1) } else { Some(0) }
}

fn detect_separation_by_space(
  fmt: &str,
  currency: &str
) -> Option<c_char> {
  let s = fmt.trim();
  if s.is_empty() || currency.is_empty() {
    return None;
  }

  let cs = find_substring_range(s, currency)?;
  let v = find_digit_span(s)?;

  if is_wrapped_in_parens(s) {
    return None;
  }

  let sign = find_sign_token(s);

  if no_spaces_between_adj_parts(s, sign, cs, v) {
    return Some(0);
  }

  let sign = match sign {
    | Some(sign) => sign,
    | None => return if is_space_between(s, cs, v) { Some(1) } else { None }
  };

  let cs_sign_adj = is_ws_only_between(s, cs, sign);
  let cs_val_space = is_space_between(s, cs, v);
  let cs_sign_space = is_space_between(s, cs, sign);
  let sign_val_space = is_space_between(s, sign, v);

  let block = union(cs, sign);
  let block_val_space = is_space_between(s, block, v);

  if (cs_sign_adj && cs_sign_space) || (!cs_sign_adj && sign_val_space) {
    return Some(2);
  }

  if (cs_sign_adj && block_val_space) || (!cs_sign_adj && cs_val_space) {
    return Some(1);
  }

  None
}

fn construct_iso4217_currency_symbol(s: &str) -> SmallVec<[u8; 5]> {
  let sb = s.as_bytes();
  let mut result: SmallVec<[u8; 5]> = SmallVec::new();
  result.extend_from_slice(&[sb[0], sb[1], sb[2], b' ', b'\0']);
  result
}

#[derive(Debug, Clone)]
pub struct MonetaryObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub mon_decimal_point: Cow<'a, [u8]>,
  pub mon_thousands_sep: Cow<'a, [u8]>,
  pub mon_grouping: SmallVec<[u8; 3]>,
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
  pub int_curr_symbol: SmallVec<[u8; 5]>,
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
    let name = locale.to_str().map_err(|_| errno::ENOENT)?;

    if is_posix_locale(name) {
      return Ok(self.set_to_posix());
    }

    let mut parts = name.split(['.', '@']);
    let lang = parts.next().unwrap_or("");
    if lang.is_empty() {
      return Err(errno::ENOENT);
    }

    let icu_locale = Locale::try_from_str(&lang.replace("_", "-"))
      .map_err(|_| errno::ENOENT)?;

    let mut options: options::DecimalFormatterOptions = Default::default();
    options.grouping_strategy =
      Some(get_grouping_strategy_for_locale(&icu_locale));

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

    let mon_decimal_point = get_decimal_point(&s_frac).ok_or(errno::ENOENT)?;
    let mon_thousands_sep = get_thousands_sep(&s_int).ok_or(errno::ENOENT)?;
    let mon_grouping = get_posix_grouping(&formatter).ok_or(errno::ENOENT)?;

    let frac_digits = static_data::get_frac_digits(lang);

    let region = extract_region(lang);
    let iso4217_currency =
      static_data::get_iso4217_currency_from_region(region)
        .ok_or(errno::ENOENT)?;

    let currency_code = TinyAsciiStr::<3>::try_from_str(iso4217_currency)
      .map_err(|_| errno::ENOENT)?;
    let currency_code = CurrencyCode(currency_code);

    let int_curr_symbol = construct_iso4217_currency_symbol(iso4217_currency);

    let mut currency_options = CurrencyFormatterOptions::default();
    currency_options.width = Width::Short;

    let currency_formatter =
      CurrencyFormatter::try_new(icu_locale.clone().into(), currency_options)
        .map_err(|_| errno::ENOENT)?;

    let fmt = |n: i128, positive: bool| {
      let n = n.wrapping_neg();
      let d = Decimal::from(n);
      let f = currency_formatter.format_fixed_decimal(&d, currency_code);
      let result =
        if positive { f.to_string().replace("-", "+") } else { f.to_string() };
      normalize_for_bidi(&result)
    };

    let p_fmt = fmt(1234567890123456789, true);
    let n_fmt = fmt(1234567890123456789, false);

    let currency_dirty = extract_currency(&n_fmt);
    let currency = normalize_for_bidi(&currency_dirty);

    let p_sign_posn =
      detect_monetary_sign_posn(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_sign_posn =
      detect_monetary_sign_posn(&n_fmt, &currency).ok_or(errno::ENOENT)?;

    let p_cs_precedes =
      detect_monetary_cs_precedes(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_cs_precedes =
      detect_monetary_cs_precedes(&n_fmt, &currency).ok_or(errno::ENOENT)?;

    let p_sep_by_space =
      detect_separation_by_space(&p_fmt, &currency).ok_or(errno::ENOENT)?;
    let n_sep_by_space =
      detect_separation_by_space(&n_fmt, &currency).ok_or(errno::ENOENT)?;

    self.name = Cow::Owned(locale.to_owned());
    self.mon_decimal_point = strtocstr(&mon_decimal_point);
    self.mon_thousands_sep = strtocstr(&mon_thousands_sep);
    self.mon_grouping = mon_grouping.into();
    self.positive_sign = Cow::Borrowed(&[b'\0']);
    self.negative_sign = Cow::Borrowed(&[b'-', b'\0']);
    self.frac_digits = frac_digits;
    self.int_frac_digits = frac_digits;
    self.currency_symbol = strtocstr(&currency_dirty);
    self.int_curr_symbol = int_curr_symbol;
    self.p_sign_posn = p_sign_posn;
    self.n_sign_posn = n_sign_posn;
    self.p_cs_precedes = p_cs_precedes;
    self.n_cs_precedes = n_cs_precedes;
    self.p_sep_by_space = p_sep_by_space;
    self.n_sep_by_space = n_sep_by_space;
    self.int_p_sign_posn = p_sign_posn;
    self.int_n_sign_posn = n_sign_posn;
    self.int_p_cs_precedes = p_cs_precedes;
    self.int_n_cs_precedes = n_cs_precedes;
    self.int_p_sep_by_space = p_sep_by_space;
    self.int_n_sep_by_space = n_sep_by_space;

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
  mon_decimal_point: Cow::Borrowed(&[b'\0']),
  mon_thousands_sep: Cow::Borrowed(&[b'\0']),
  mon_grouping: SmallVec::new_const(),
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
  int_curr_symbol: SmallVec::new_const(),
  int_frac_digits: c_char::MAX,
  int_p_cs_precedes: c_char::MAX,
  int_n_cs_precedes: c_char::MAX,
  int_p_sep_by_space: c_char::MAX,
  int_n_sep_by_space: c_char::MAX,
  int_p_sign_posn: c_char::MAX,
  int_n_sign_posn: c_char::MAX
};
