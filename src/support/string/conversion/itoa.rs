use {
  core::ascii,
  num_traits::{PrimInt, WrappingNeg}
};

#[derive(PartialEq, Clone)]
pub enum ItoaFormat {
  Binary,
  Octal,
  Decimal,
  Hexadecimal
}

#[inline]
pub fn format_integer<T: PrimInt + WrappingNeg>(
  value: T,
  fmt: ItoaFormat,
  buffer: &mut [ascii::Char],
  lowercase: bool
) -> &mut [ascii::Char] {
  let base = match fmt {
    | ItoaFormat::Binary => 2,
    | ItoaFormat::Octal => 8,
    | ItoaFormat::Decimal => 10,
    | ItoaFormat::Hexadecimal => 16
  };
  let Some(base) = T::from(base) else {
    return &mut [];
  };
  let a = if lowercase { ascii::Char::SmallA } else { ascii::Char::CapitalA };

  let mut value = if value < T::zero() { value.wrapping_neg() } else { value };

  let mut counter = 0usize;
  while !value.is_zero() {
    let i = value % base;
    value = value / base;
    let acc = i.to_u8().unwrap_or(0);
    let ascii_digit: u8 =
      if acc < 10 { acc + b'0' } else { acc + a as u8 - 10 };
    buffer[counter] =
      ascii::Char::from_u8(ascii_digit).unwrap_or(ascii::Char::Null);
    counter += 1;
  }

  buffer[..counter].reverse();
  &mut buffer[..counter]
}
