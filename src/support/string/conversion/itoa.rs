use {
  core::{ascii, ops::Neg},
  num_traits::{PrimInt, Signed, Unsigned}
};

#[derive(PartialEq, Clone)]
pub enum ItoaFormat {
  Binary,
  Octal,
  Decimal,
  Hexadecimal
}

#[inline]
fn write_digits<T: PrimInt>(
  mut value: T,
  base: T,
  a: ascii::Char,
  buffer: &mut [ascii::Char]
) -> Option<usize> {
  let mut counter = 0usize;
  while value != T::zero() {
    let rem = (value % base).to_u8()?;
    let byte = if rem < 10 { b'0' + rem } else { a.to_u8() + rem - 10 };
    buffer[counter] = ascii::Char::from_u8(byte).unwrap_or(ascii::Char::Null);
    counter += 1;
    value = value / base;
  }
  Some(counter)
}

#[inline]
pub fn format_signed<T: PrimInt + Signed + Neg<Output = T>>(
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

  if value == T::zero() {
    buffer[0] = ascii::Char::Digit0;
    return &mut buffer[..1];
  }

  let negative = value < T::zero() && fmt == ItoaFormat::Decimal;
  let value = if negative { value.neg() } else { value };

  let Some(counter) = write_digits(value, base, a, buffer) else {
    return &mut [];
  };

  let counter = if negative {
    buffer[counter] = ascii::Char::HyphenMinus;
    counter + 1
  } else {
    counter
  };

  buffer[..counter].reverse();
  &mut buffer[..counter]
}

#[inline]
pub fn format_unsigned<T: PrimInt + Unsigned>(
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

  if value == T::zero() {
    buffer[0] = ascii::Char::Digit0;
    return &mut buffer[..1];
  }

  let Some(counter) = write_digits(value, base, a, buffer) else {
    return &mut [];
  };

  buffer[..counter].reverse();
  &mut buffer[..counter]
}
