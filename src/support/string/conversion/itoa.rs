use {
  super::IsSigned,
  core::{ascii, mem::size_of, ops::Neg},
  num_traits::{PrimInt, Signed, Unsigned}
};

#[derive(PartialEq)]
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

// Buffer calculation taken from LLVM libc:
// https://github.com/llvm/llvm-project/blob/1557256ab02eab80557dbdb37631c7170bf46cfa/libc/src/__support/integer_to_string.h#L116
#[inline]
pub const fn buffer_size<T: PrimInt + IsSigned, const BASE: usize>() -> usize {
  let bits_per_digit = BASE.ilog2() as usize;
  let type_size = size_of::<T>() * 8;
  let delta: usize = bits_per_digit - 1;
  let buffer_size_common: usize = (type_size + delta) / bits_per_digit;
  let buffer_size_base10: usize = (size_of::<T>() * 5 + 1) / 2;
  let signed: usize = if T::IS_SIGNED { 1 } else { 0 };
  let result: usize =
    if BASE == 10 { buffer_size_base10 } else { buffer_size_common };
  signed + result
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
