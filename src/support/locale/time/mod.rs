use {
  super::{LocaleObject, canonicalize_locale, is_posix_locale},
  crate::{
    allocation::{
      borrow::{Cow, ToOwned},
      format,
      string::{String, ToString}
    },
    c_int,
    support::{locale::errno, string::strtocstr}
  },
  core::ffi,
  icu_calendar::cal::Gregorian,
  icu_datetime::{
    DateTimeFormatter,
    NoCalendarFormatter,
    fieldsets::{self, T, YMD, YMDET},
    input::{Date, Time}
  },
  icu_decimal::{DecimalFormatter, input::Decimal},
  icu_locale::Locale,
  icu_time::DateTime,
  smallvec::SmallVec,
  strum::IntoEnumIterator,
  strum_macros::EnumIter
};

#[derive(EnumIter, Clone)]
enum Weekday {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday
}

#[derive(EnumIter, Clone)]
enum Month {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December
}

#[derive(Clone, PartialEq)]
enum FormatType {
  Time,
  TimeAMPM,
  Date,
  DateTime
}

fn extract_am_pm_strings(
  locale: &Locale,
  is_pm: bool
) -> Option<String> {
  let post_meridiem = Time::try_new(13, 0, 0, 0).ok()?;

  let time = if is_pm { post_meridiem } else { Time::start_of_day() };

  let formatter =
    NoCalendarFormatter::try_new(locale.into(), fieldsets::T::hm()).ok()?;

  let out = formatter.format(&time).to_string();
  let out = out.trim();

  let result: String = out
    .chars()
    .filter(|c| {
      !(c.is_ascii_punctuation() || c.is_numeric() || c.is_whitespace())
    })
    .collect();

  if result.is_empty() {
    return Some(String::from(""));
  }

  Some(result)
}

fn extract_weekday_strings(
  locale: &Locale,
  day: Weekday,
  abbreviated: bool
) -> Option<String> {
  let day_num = match day {
    | Weekday::Monday => 3,
    | Weekday::Tuesday => 4,
    | Weekday::Wednesday => 5,
    | Weekday::Thursday => 6,
    | Weekday::Friday => 7,
    | Weekday::Saturday => 8,
    | Weekday::Sunday => 9
  };

  let date = Date::try_new_iso(2000, 1, day_num).ok()?;

  let fieldset =
    if abbreviated { fieldsets::E::short() } else { fieldsets::E::long() };

  let formatter =
    DateTimeFormatter::<fieldsets::E>::try_new(locale.into(), fieldset).ok()?;

  let out = formatter.format(&date).to_string();

  if out.is_empty() {
    return None;
  }

  Some(out)
}

#[derive(Debug, Clone)]
struct MonthReturn {
  pub month: String,
  pub alt_month: Option<String>
}

fn extract_month_strings(
  locale: &Locale,
  month: Month,
  abbr: bool
) -> Option<MonthReturn> {
  let month_num = match month {
    | Month::January => 1,
    | Month::February => 2,
    | Month::March => 3,
    | Month::April => 4,
    | Month::May => 5,
    | Month::June => 6,
    | Month::July => 7,
    | Month::August => 8,
    | Month::September => 9,
    | Month::October => 10,
    | Month::November => 11,
    | Month::December => 12
  };
  let date = Date::try_new_iso(2000, month_num, 1).ok()?;

  let language = locale.id.language.as_str();
  let cjk: bool = matches!(language, "ja" | "zh" | "ko" | "yue");

  let fmt_m =
    if abbr && !cjk { fieldsets::M::medium() } else { fieldsets::M::long() };
  let formatter_m =
    DateTimeFormatter::<fieldsets::M>::try_new(locale.into(), fmt_m).ok()?;
  let month_standalone =
    formatter_m.format(&date).to_string().trim().to_string();

  let fmt_d =
    if abbr && !cjk { fieldsets::D::medium() } else { fieldsets::D::long() };
  let formatter_d =
    DateTimeFormatter::<fieldsets::D>::try_new(locale.into(), fmt_d).ok()?;
  let day_str = formatter_d.format(&date).to_string();

  let fmt_md =
    if abbr && !cjk { fieldsets::MD::medium() } else { fieldsets::MD::long() };
  let formatter_md =
    DateTimeFormatter::<fieldsets::MD>::try_new(locale.into(), fmt_md).ok()?;
  let md_str = formatter_md.format(&date).to_string();

  let month_format_raw = if cjk {
    md_str
      .replace(day_str.trim(), "")
      .trim_matches(|c: char| {
        c.is_whitespace() || matches!(c, ',' | '.' | '-' | '/' | '、')
      })
      .to_string()
  } else {
    md_str
      .replace(day_str.trim(), "")
      .trim_matches(|c: char| {
        c.is_whitespace() ||
          c.is_numeric() ||
          matches!(c, ',' | '.' | '-' | '/' | '、')
      })
      .to_string()
  };

  let month_format = if month_format_raw.is_empty() {
    month_standalone.clone()
  } else {
    month_format_raw
  };

  if month_standalone == month_format {
    Some(MonthReturn { month: month_standalone, alt_month: None })
  } else {
    Some(MonthReturn { month: month_format, alt_month: Some(month_standalone) })
  }
}

fn extract_alternative_digits(
  locale: &Locale,
  digit: u32
) -> Option<String> {
  let language = locale.id.language.as_str();

  let tag = match language {
    | "ar" => format!("{}-u-nu-arab", language),
    | "th" => format!("{}-u-nu-thai", language),
    | "ja" | "yue" | "zh" => format!("{}-u-nu-hanidec", language),
    | _ => return None
  };

  let icu_locale: Locale = tag.parse().ok()?;

  let formatter =
    DecimalFormatter::try_new(icu_locale.into(), Default::default()).ok()?;
  let decimal = Decimal::from(digit);

  let result = formatter.format(&decimal).to_string();

  if result.chars().all(|c| c.is_ascii_alphanumeric()) {
    return None;
  }

  Some(result)
}

fn get_format_pattern(
  locale: &Locale,
  t: FormatType
) -> Option<String> {
  let mut result = String::new();
  let time = Time::start_of_day();
  let date = Date::try_from_str("2000-01-01", Gregorian).ok()?;
  let datetime =
    DateTime::try_from_str("2000-01-01T13:00:00", Gregorian).ok()?;
  let has_am_pm = if let Some(h) = extract_am_pm_strings(&locale, false) &&
    !h.is_empty()
  {
    true
  } else {
    false
  };

  let tf = DateTimeFormatter::try_new(locale.into(), T::medium()).ok()?;
  let df = DateTimeFormatter::try_new(locale.into(), YMD::medium()).ok()?;
  let dtf = DateTimeFormatter::try_new(locale.into(), YMDET::medium()).ok()?;

  let fmt: String = match t {
    | FormatType::Time | FormatType::TimeAMPM => {
      tf.format(&time).pattern().to_string()
    },
    | FormatType::Date => df.format(&date).pattern().to_string(),
    | FormatType::DateTime => dtf.format(&datetime).pattern().to_string()
  };

  let mut iter = fmt.chars().peekable();

  while let Some(ch) = iter.next() {
    if ch.is_ascii_alphabetic() {
      match ch {
        | 'a' => result.push_str("%p"),
        | 'h' => {
          if iter.peek() == Some(&'h') {
            iter.next();
            result.push_str("%I");
          } else {
            result.push_str("%l");
          }
        },
        | 'k' => {
          if iter.peek() == Some(&'k') {
            iter.next();
            result.push_str("%I");
          } else {
            result.push_str("%l");
          }
        },
        | 'H' => {
          if iter.peek() == Some(&'H') {
            iter.next();
            result.push_str("%H");
          } else {
            result.push_str("%k");
          }
        },
        | 'm' => {
          if iter.peek() == Some(&'m') {
            iter.next();
          }
          result.push_str("%M");
        },
        | 's' => {
          if iter.peek() == Some(&'s') {
            iter.next();
          }
          result.push_str("%S");
        },
        | 'd' => {
          if iter.peek() == Some(&'d') {
            iter.next();
            result.push_str("%d");
          } else {
            result.push_str("%e");
          }
        },
        | 'D' => result.push_str("%j"),
        | 'Y' => {
          while iter.peek() == Some(&'Y') {
            iter.next();
          }
          result.push_str("%G");
        },
        | 'y' => {
          let mut count = 1usize;
          while iter.peek() == Some(&'y') {
            iter.next();
            count += 1;
          }
          if count == 2 {
            result.push_str("%y");
          } else {
            result.push_str("%Y");
          }
        },
        | 'M' => {
          let mut count = 1usize;
          while iter.peek() == Some(&'M') {
            iter.next();
            count += 1;
          }
          match count {
            | 1 | 2 => result.push_str("%m"),
            | 3 => result.push_str("%b"),
            | _ => result.push_str("%B")
          }
        },
        | 'E' | 'e' => {
          let same = ch;
          let mut count = 1usize;
          while iter.peek() == Some(&same) {
            iter.next();
            count += 1;
          }
          match count {
            | 1 | 2 => result.push_str("%a"),
            | 3 => result.push_str("%a"),
            | _ => result.push_str("%A")
          }
        },

        | 'G' | 'Q' | 'q' | 'L' | 'w' | 'W' | 'F' | 'g' | 'z' | 'Z' | 'O' |
        'v' | 'V' | 'X' | 'x' => {
          let same = ch;
          while iter.peek() == Some(&same) {
            iter.next();
          }
        },
        | other => {
          while iter.peek() == Some(&other) {
            iter.next();
          }
        },
      }
    } else if ch == '\'' {
      loop {
        match iter.next() {
          | None => break,
          | Some('\'') => {
            if iter.peek() == Some(&'\'') {
              iter.next();
              result.push('\'');
            } else {
              break;
            }
          },
          | Some(c) => result.push(c)
        }
      }
    } else {
      result.push(ch);
    }
  }

  if has_am_pm && t == FormatType::TimeAMPM {
    result = result.replace("%H", "%I").replace("%k", "%l");
  } else if !has_am_pm && t == FormatType::TimeAMPM {
    return Some(String::new());
  }

  Some(result)
}

const DAYS: [Cow<'static, ffi::CStr>; 7] = [
  Cow::Borrowed(c"Monday"),
  Cow::Borrowed(c"Tuesday"),
  Cow::Borrowed(c"Wednesday"),
  Cow::Borrowed(c"Thursday"),
  Cow::Borrowed(c"Friday"),
  Cow::Borrowed(c"Saturday"),
  Cow::Borrowed(c"Sunday")
];

const DAYS_ABBR: [Cow<'static, ffi::CStr>; 7] = [
  Cow::Borrowed(c"Mon"),
  Cow::Borrowed(c"Tue"),
  Cow::Borrowed(c"Wed"),
  Cow::Borrowed(c"Thu"),
  Cow::Borrowed(c"Fri"),
  Cow::Borrowed(c"Sat"),
  Cow::Borrowed(c"Sun")
];

const MONTHS: [Cow<'static, ffi::CStr>; 12] = [
  Cow::Borrowed(c"January"),
  Cow::Borrowed(c"February"),
  Cow::Borrowed(c"March"),
  Cow::Borrowed(c"April"),
  Cow::Borrowed(c"May"),
  Cow::Borrowed(c"June"),
  Cow::Borrowed(c"July"),
  Cow::Borrowed(c"August"),
  Cow::Borrowed(c"September"),
  Cow::Borrowed(c"October"),
  Cow::Borrowed(c"November"),
  Cow::Borrowed(c"December")
];

const MONTHS_ABBR: [Cow<'static, ffi::CStr>; 12] = [
  Cow::Borrowed(c"Jan"),
  Cow::Borrowed(c"Feb"),
  Cow::Borrowed(c"Mar"),
  Cow::Borrowed(c"Apr"),
  Cow::Borrowed(c"May"),
  Cow::Borrowed(c"Jun"),
  Cow::Borrowed(c"Jul"),
  Cow::Borrowed(c"Aug"),
  Cow::Borrowed(c"Sep"),
  Cow::Borrowed(c"Oct"),
  Cow::Borrowed(c"Nov"),
  Cow::Borrowed(c"Dec")
];

#[derive(Debug, Clone)]
pub struct TimeObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub am_str: Cow<'a, ffi::CStr>,
  pub pm_str: Cow<'a, ffi::CStr>,
  pub days: SmallVec<[Cow<'a, ffi::CStr>; 7]>,
  pub days_abbr: SmallVec<[Cow<'a, ffi::CStr>; 7]>,
  pub months: SmallVec<[Cow<'a, ffi::CStr>; 12]>,
  pub months_abbr: SmallVec<[Cow<'a, ffi::CStr>; 12]>,
  pub alt_months: SmallVec<[Cow<'a, ffi::CStr>; 12]>,
  pub alt_months_abbr: SmallVec<[Cow<'a, ffi::CStr>; 12]>,
  pub alternative_digits: SmallVec<[Cow<'a, ffi::CStr>; 367]>,
  pub time_fmt: Cow<'a, ffi::CStr>,
  pub time_fmt_am_pm: Cow<'a, ffi::CStr>,
  pub date_fmt: Cow<'a, ffi::CStr>,
  pub date_time_fmt: Cow<'a, ffi::CStr>
}

impl<'a> TimeObject<'a> {
  pub const fn default_time() -> TimeObject<'a> {
    TimeObject {
      name: Cow::Borrowed(c"C"),
      am_str: Cow::Borrowed(c"AM"),
      pm_str: Cow::Borrowed(c"PM"),
      days: SmallVec::from_const(DAYS),
      days_abbr: SmallVec::from_const(DAYS_ABBR),
      months: SmallVec::from_const(MONTHS),
      months_abbr: SmallVec::from_const(MONTHS_ABBR),
      alt_months: SmallVec::new_const(),
      alt_months_abbr: SmallVec::new_const(),
      alternative_digits: SmallVec::new_const(),
      time_fmt: Cow::Borrowed(c"%H:%M:%S"),
      time_fmt_am_pm: Cow::Borrowed(c"%I:%M:%S %p"),
      date_fmt: Cow::Borrowed(c"%m/%d/%y"),
      date_time_fmt: Cow::Borrowed(c"%a %b %e %H:%M:%S %Y")
    }
  }
}

impl<'a> LocaleObject for TimeObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str().map_err(|_| errno::ENOENT)?;

    if is_posix_locale(name) {
      return Ok(self.set_to_posix(locale));
    }

    let (icu_locale_name, _) = canonicalize_locale(name);

    let icu_locale =
      Locale::try_from_str(&icu_locale_name).map_err(|_| errno::ENOENT)?;

    self.days.clear();
    self.days_abbr.clear();
    self.months.clear();
    self.months_abbr.clear();
    self.alt_months.clear();
    self.alt_months_abbr.clear();
    self.alternative_digits.clear();

    self.am_str = strtocstr(
      &extract_am_pm_strings(&icu_locale, false).ok_or(errno::ENOENT)?
    );
    self.pm_str = strtocstr(
      &extract_am_pm_strings(&icu_locale, true).ok_or(errno::ENOENT)?
    );

    for d in Weekday::iter() {
      let day = extract_weekday_strings(&icu_locale, d.clone(), false)
        .ok_or(errno::ENOENT)?;
      let day_abbr = extract_weekday_strings(&icu_locale, d.clone(), true)
        .ok_or(errno::ENOENT)?;

      self.days.push(strtocstr(&day));
      self.days_abbr.push(strtocstr(&day_abbr));
    }

    for m in Month::iter() {
      let month = extract_month_strings(&icu_locale, m.clone(), false)
        .ok_or(errno::ENOENT)?;
      let month_abbr = extract_month_strings(&icu_locale, m.clone(), true)
        .ok_or(errno::ENOENT)?;

      if let (Some(a), Some(b)) = (month.alt_month, month_abbr.alt_month) {
        self.alt_months.push(strtocstr(&a));
        self.alt_months_abbr.push(strtocstr(&b));
      }

      self.months.push(strtocstr(&month.month));
      self.months_abbr.push(strtocstr(&month_abbr.month));
    }

    for i in 0..367usize {
      let digit = extract_alternative_digits(&icu_locale, i as u32);
      if let Some(d) = digit {
        self.alternative_digits.push(strtocstr(&d));
      }
    }

    self.time_fmt = strtocstr(
      &get_format_pattern(&icu_locale, FormatType::Time)
        .ok_or(errno::ENOENT)?
    );
    self.time_fmt_am_pm = strtocstr(
      &get_format_pattern(&icu_locale, FormatType::TimeAMPM)
        .ok_or(errno::ENOENT)?
    );
    self.date_fmt = strtocstr(
      &get_format_pattern(&icu_locale, FormatType::Date)
        .ok_or(errno::ENOENT)?
    );
    self.date_time_fmt = strtocstr(
      &get_format_pattern(&icu_locale, FormatType::DateTime)
        .ok_or(errno::ENOENT)?
    );

    self.name = Cow::Owned(locale.to_owned());
    Ok(self.name.as_ref())
  }

  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr {
    *self = Self::default_time();

    self.name = Cow::Owned(locale.to_owned());
    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for TimeObject<'a> {
  fn default() -> Self {
    Self::default_time()
  }
}
