pub mod en_us;
pub mod posix;
pub mod ru_ru;

#[derive(Copy, Clone)]
pub struct LCTime<'a> {
  pub d_t_fmt: &'a str,
  pub d_fmt: &'a str,
  pub t_fmt: &'a str,
  pub t_fmt_ampm: &'a str,
  pub am_str: &'a str,
  pub pm_str: &'a str,
  pub day: [&'a str; 7],
  pub abday: [&'a str; 7],
  pub mon: [&'a str; 12],
  pub abmon: [&'a str; 12]
}
