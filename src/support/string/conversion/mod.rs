use {crate::std::errno, core::convert::Into};

pub mod clinger;
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

pub trait IsSigned {
  const IS_SIGNED: bool;
}

macro_rules! is_signed_impl {
  ($type:ty, $value:expr) => {
    impl IsSigned for $type {
      const IS_SIGNED: bool = $value;
    }
  };
}

is_signed_impl!(i8, true);
is_signed_impl!(i16, true);
is_signed_impl!(i32, true);
is_signed_impl!(i64, true);
is_signed_impl!(i128, true);
is_signed_impl!(isize, true);

is_signed_impl!(u8, false);
is_signed_impl!(u16, false);
is_signed_impl!(u32, false);
is_signed_impl!(u64, false);
is_signed_impl!(u128, false);
is_signed_impl!(usize, false);

#[inline]
fn b36_char_to_int(ch: char) -> Option<u32> {
  ch.to_digit(36).map(|d| d as u32)
}
