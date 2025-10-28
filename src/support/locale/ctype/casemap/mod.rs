pub mod ascii;
pub mod icu;

pub struct CaseMapObject {
  pub isalnum: fn(u32) -> bool,
  pub isalpha: fn(u32) -> bool,
  pub isblank: fn(u32) -> bool,
  pub iscntrl: fn(u32) -> bool,
  pub isdigit: fn(u32) -> bool,
  pub isgraph: fn(u32) -> bool,
  pub islower: fn(u32) -> bool,
  pub isprint: fn(u32) -> bool,
  pub ispunct: fn(u32) -> bool,
  pub isspace: fn(u32) -> bool,
  pub isupper: fn(u32) -> bool,
  pub isxdigit: fn(u32) -> bool,
  pub tolower: fn(u32) -> u32,
  pub toupper: fn(u32) -> u32
}
