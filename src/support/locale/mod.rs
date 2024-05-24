use crate::LocaleStruct;

pub mod collate;
pub mod ctype;
pub mod messages;
pub mod monetary;
pub mod numeric;
pub mod time;

const DEFAULT_LOCALE: LocaleStruct = LocaleStruct {
  lc_all: c"C",
  lc_collate: c"C",
  lc_ctype: c"C",
  lc_messages: c"C",
  lc_monetary: c"C",
  lc_numeric: c"C",
  lc_time: c"C",
  collate: Some(collate::COLLATE_POSIX),
  ctype: Some(ctype::ascii::CTYPE_ASCII),
  messages: Some(messages::en_us::MESSAGES_EN_US),
  monetary: Some(monetary::posix::MONETARY_POSIX),
  numeric: Some(numeric::posix::NUMERIC_POSIX),
  time: Some(time::posix::TIME_POSIX)
};

#[thread_local]
pub static mut ThreadLocale: LocaleStruct = DEFAULT_LOCALE;

#[inline(always)]
pub fn get_thread_locale() -> LocaleStruct<'static> {
  unsafe { ThreadLocale }
}

#[inline(always)]
pub fn set_thread_locale(locale: LocaleStruct<'static>) {
  unsafe { ThreadLocale = locale };
}
