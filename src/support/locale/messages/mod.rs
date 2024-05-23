pub mod en_us;
pub mod ru_ru;

#[derive(Copy, Clone)]
pub struct LCMessages<'a> {
  pub strerror: [&'a str; 134],
  pub strsignal: [&'a str; 32],
  pub gai_strerror: [&'a str; 17],
  pub signal_realtime: &'a str,
  pub signal_unknown: &'a str,
  pub unknown_error: &'a str,
  pub yesexpr: &'a str,
  pub noexpr: &'a str
}
