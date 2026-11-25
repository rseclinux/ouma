use crate::{
  std::nl_types::nl_item,
  support::locale,
  types::{c_char, locale_t}
};

pub const CODESET: nl_item = 0;
pub const D_T_FMT: nl_item = 1;
pub const D_FMT: nl_item = 2;
pub const T_FMT: nl_item = 3;
pub const T_FMT_AMPM: nl_item = 4;
pub const AM_STR: nl_item = 5;
pub const PM_STR: nl_item = 6;
pub const DAY_1: nl_item = 7;
pub const DAY_2: nl_item = 8;
pub const DAY_3: nl_item = 9;
pub const DAY_4: nl_item = 10;
pub const DAY_5: nl_item = 11;
pub const DAY_6: nl_item = 12;
pub const DAY_7: nl_item = 13;
pub const ABDAY_1: nl_item = 14;
pub const ABDAY_2: nl_item = 15;
pub const ABDAY_3: nl_item = 16;
pub const ABDAY_4: nl_item = 17;
pub const ABDAY_5: nl_item = 18;
pub const ABDAY_6: nl_item = 19;
pub const ABDAY_7: nl_item = 20;
pub const MON_1: nl_item = 21;
pub const MON_2: nl_item = 22;
pub const MON_3: nl_item = 23;
pub const MON_4: nl_item = 24;
pub const MON_5: nl_item = 25;
pub const MON_6: nl_item = 26;
pub const MON_7: nl_item = 27;
pub const MON_8: nl_item = 28;
pub const MON_9: nl_item = 29;
pub const MON_10: nl_item = 30;
pub const MON_11: nl_item = 31;
pub const MON_12: nl_item = 32;
pub const ALTMON_1: nl_item = 33;
pub const ALTMON_2: nl_item = 34;
pub const ALTMON_3: nl_item = 35;
pub const ALTMON_4: nl_item = 36;
pub const ALTMON_5: nl_item = 37;
pub const ALTMON_6: nl_item = 38;
pub const ALTMON_7: nl_item = 39;
pub const ALTMON_8: nl_item = 40;
pub const ALTMON_9: nl_item = 41;
pub const ALTMON_10: nl_item = 42;
pub const ALTMON_11: nl_item = 43;
pub const ALTMON_12: nl_item = 44;
pub const ABMON_1: nl_item = 45;
pub const ABMON_2: nl_item = 46;
pub const ABMON_3: nl_item = 47;
pub const ABMON_4: nl_item = 48;
pub const ABMON_5: nl_item = 49;
pub const ABMON_6: nl_item = 50;
pub const ABMON_7: nl_item = 51;
pub const ABMON_8: nl_item = 52;
pub const ABMON_9: nl_item = 53;
pub const ABMON_10: nl_item = 54;
pub const ABMON_11: nl_item = 55;
pub const ABMON_12: nl_item = 56;
pub const ABALTMON_1: nl_item = 57;
pub const ABALTMON_2: nl_item = 58;
pub const ABALTMON_3: nl_item = 59;
pub const ABALTMON_4: nl_item = 60;
pub const ABALTMON_5: nl_item = 61;
pub const ABALTMON_6: nl_item = 62;
pub const ABALTMON_7: nl_item = 63;
pub const ABALTMON_8: nl_item = 64;
pub const ABALTMON_9: nl_item = 65;
pub const ABALTMON_10: nl_item = 66;
pub const ABALTMON_11: nl_item = 67;
pub const ABALTMON_12: nl_item = 68;
pub const ERA: nl_item = 69;
pub const ERA_D_FMT: nl_item = 70;
pub const ERA_D_T_FMT: nl_item = 71;
pub const ERA_T_FMT: nl_item = 72;
pub const ALT_DIGITS: nl_item = 73;
pub const RADIXCHAR: nl_item = 74;
pub const THOUSEP: nl_item = 75;
pub const YESEXPR: nl_item = 76;
pub const NOEXPR: nl_item = 77;
pub const CRNCYSTR: nl_item = 78;

#[inline]
fn inner_langinfo<'a>(
  item: nl_item,
  locale: &locale::Locale<'a>
) -> Option<*mut c_char> {
  get_slot!(ctype, locale.ctype, locale::ctype::CtypeObject);
  get_slot!(messages, locale.messages, locale::messages::MessagesObject);
  get_slot!(numeric, locale.numeric, locale::numeric::NumericObject);
  get_slot!(time, locale.time, locale::time::TimeObject);

  match item {
    | CODESET => Some(ctype.converter.codeset.as_ptr().cast_mut()),
    | D_T_FMT => Some(time.date_time_fmt.as_ptr().cast_mut()),
    | D_FMT => Some(time.date_fmt.as_ptr().cast_mut()),
    | T_FMT => Some(time.time_fmt.as_ptr().cast_mut()),
    | T_FMT_AMPM => Some(time.time_fmt_am_pm.as_ptr().cast_mut()),
    | AM_STR => Some(time.am_str.as_ptr().cast_mut()),
    | PM_STR => Some(time.pm_str.as_ptr().cast_mut()),
    | DAY_1..=DAY_7 => {
      Some(time.days[(item - DAY_1) as usize].as_ptr().cast_mut())
    },
    | ABDAY_1..=ABDAY_7 => {
      Some(time.days_abbr[(item - ABDAY_1) as usize].as_ptr().cast_mut())
    },
    | MON_1..=MON_12 => {
      Some(time.months[(item - MON_1) as usize].as_ptr().cast_mut())
    },
    | ALTMON_1..=ALTMON_12 => {
      Some(time.alt_months[(item - ALTMON_1) as usize].as_ptr().cast_mut())
    },
    | ABMON_1..=ABMON_12 => {
      Some(time.months_abbr[(item - ABMON_1) as usize].as_ptr().cast_mut())
    },
    | ABALTMON_1..=ABALTMON_12 => Some(
      time.alt_months_abbr[(item - ABALTMON_1) as usize].as_ptr().cast_mut()
    ),
    | ERA..=ERA_T_FMT => None,
    | ALT_DIGITS => {
      time.alternative_digits.get(0).map(|f| f.as_ptr().cast_mut())
    },
    | RADIXCHAR => Some(numeric.decimal_point.as_ptr().cast_mut()),
    | THOUSEP => Some(numeric.thousands_sep.as_ptr().cast_mut()),
    | YESEXPR => Some(messages.yesexpr.as_ptr().cast_mut()),
    | NOEXPR => Some(messages.noexpr.as_ptr().cast_mut()),
    | CRNCYSTR => Some(locale.crncystr.as_ptr().cast()),
    | _ => None
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_nl_langinfo(item: nl_item) -> *mut c_char {
  rs_nl_langinfo_l(item, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_nl_langinfo_l(
  item: nl_item,
  locale: locale_t<'static>
) -> *mut c_char {
  let locale = locale::get_real_locale(locale);
  match inner_langinfo(item, locale) {
    | Some(s) => s,
    | None => c"".as_ptr().cast_mut()
  }
}
