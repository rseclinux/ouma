use {crate::std::errno, core::convert::Into};

pub mod detailed_powers_of_ten;
pub mod ftoa;
pub mod hpd;
pub mod itoa;
pub mod ryu;
pub mod ryu_table;
pub mod strtofloat;
pub mod strtoint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrToError {
  InvalidNumber,
  Range
}

impl Into<i32> for StrToError {
  #[inline]
  fn into(self) -> i32 {
    match self {
      | Self::InvalidNumber => errno::EINVAL,
      | Self::Range => errno::ERANGE
    }
  }
}

#[inline]
fn b36_char_to_int(ch: char) -> Option<u32> {
  ch.to_digit(36).map(|d| d as u32)
}
