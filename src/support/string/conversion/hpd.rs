// High Precision Decimal string to float implementation
// https://github.com/llvm/llvm-project/blob/10e7761cac92ee695d2a74a813ad3ebba4e649c0/libc/src/__support/high_precision_decimal.h
//

use {
  super::strtoint,
  crate::{
    c_int,
    support::{
      float::rounding_mode::Rounding,
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::b36_char_to_int,
      traits::char::{CharToAscii, MatchChar, get_char_with_index}
    }
  }
};

const MAX_NUM_DIGITS: usize = 800; // 800 is an arbitary number

pub struct HPD {
  pub ndigits: usize,
  pub exponenta: i32,
  pub trunc: bool,
  pub digits: [u8; MAX_NUM_DIGITS]
}

impl HPD {
  const SHIFT_POW5: [u8; 1308] = [
    5u8, 2u8, 5u8, 1u8, 2u8, 5u8, 6u8, 2u8, 5u8, 3u8, 1u8, 2u8, 5u8, 1u8, 5u8,
    6u8, 2u8, 5u8, 7u8, 8u8, 1u8, 2u8, 5u8, 3u8, 9u8, 0u8, 6u8, 2u8, 5u8, 1u8,
    9u8, 5u8, 3u8, 1u8, 2u8, 5u8, 9u8, 7u8, 6u8, 5u8, 6u8, 2u8, 5u8, 4u8, 8u8,
    8u8, 2u8, 8u8, 1u8, 2u8, 5u8, 2u8, 4u8, 4u8, 1u8, 4u8, 0u8, 6u8, 2u8, 5u8,
    1u8, 2u8, 2u8, 0u8, 7u8, 0u8, 3u8, 1u8, 2u8, 5u8, 6u8, 1u8, 0u8, 3u8, 5u8,
    1u8, 5u8, 6u8, 2u8, 5u8, 3u8, 0u8, 5u8, 1u8, 7u8, 5u8, 7u8, 8u8, 1u8, 2u8,
    5u8, 1u8, 5u8, 2u8, 5u8, 8u8, 7u8, 8u8, 9u8, 0u8, 6u8, 2u8, 5u8, 7u8, 6u8,
    2u8, 9u8, 3u8, 9u8, 4u8, 5u8, 3u8, 1u8, 2u8, 5u8, 3u8, 8u8, 1u8, 4u8, 6u8,
    9u8, 7u8, 2u8, 6u8, 5u8, 6u8, 2u8, 5u8, 1u8, 9u8, 0u8, 7u8, 3u8, 4u8, 8u8,
    6u8, 3u8, 2u8, 8u8, 1u8, 2u8, 5u8, 9u8, 5u8, 3u8, 6u8, 7u8, 4u8, 3u8, 1u8,
    6u8, 4u8, 0u8, 6u8, 2u8, 5u8, 4u8, 7u8, 6u8, 8u8, 3u8, 7u8, 1u8, 5u8, 8u8,
    2u8, 0u8, 3u8, 1u8, 2u8, 5u8, 2u8, 3u8, 8u8, 4u8, 1u8, 8u8, 5u8, 7u8, 9u8,
    1u8, 0u8, 1u8, 5u8, 6u8, 2u8, 5u8, 1u8, 1u8, 9u8, 2u8, 0u8, 9u8, 2u8, 8u8,
    9u8, 5u8, 5u8, 0u8, 7u8, 8u8, 1u8, 2u8, 5u8, 5u8, 9u8, 6u8, 0u8, 4u8, 6u8,
    4u8, 4u8, 7u8, 7u8, 5u8, 3u8, 9u8, 0u8, 6u8, 2u8, 5u8, 2u8, 9u8, 8u8, 0u8,
    2u8, 3u8, 2u8, 2u8, 3u8, 8u8, 7u8, 6u8, 9u8, 5u8, 3u8, 1u8, 2u8, 5u8, 1u8,
    4u8, 9u8, 0u8, 1u8, 1u8, 6u8, 1u8, 1u8, 9u8, 3u8, 8u8, 4u8, 7u8, 6u8, 5u8,
    6u8, 2u8, 5u8, 7u8, 4u8, 5u8, 0u8, 5u8, 8u8, 0u8, 5u8, 9u8, 6u8, 9u8, 2u8,
    3u8, 8u8, 2u8, 8u8, 1u8, 2u8, 5u8, 3u8, 7u8, 2u8, 5u8, 2u8, 9u8, 0u8, 2u8,
    9u8, 8u8, 4u8, 6u8, 1u8, 9u8, 1u8, 4u8, 0u8, 6u8, 2u8, 5u8, 1u8, 8u8, 6u8,
    2u8, 6u8, 4u8, 5u8, 1u8, 4u8, 9u8, 2u8, 3u8, 0u8, 9u8, 5u8, 7u8, 0u8, 3u8,
    1u8, 2u8, 5u8, 9u8, 3u8, 1u8, 3u8, 2u8, 2u8, 5u8, 7u8, 4u8, 6u8, 1u8, 5u8,
    4u8, 7u8, 8u8, 5u8, 1u8, 5u8, 6u8, 2u8, 5u8, 4u8, 6u8, 5u8, 6u8, 6u8, 1u8,
    2u8, 8u8, 7u8, 3u8, 0u8, 7u8, 7u8, 3u8, 9u8, 2u8, 5u8, 7u8, 8u8, 1u8, 2u8,
    5u8, 2u8, 3u8, 2u8, 8u8, 3u8, 0u8, 6u8, 4u8, 3u8, 6u8, 5u8, 3u8, 8u8, 6u8,
    9u8, 6u8, 2u8, 8u8, 9u8, 0u8, 6u8, 2u8, 5u8, 1u8, 1u8, 6u8, 4u8, 1u8, 5u8,
    3u8, 2u8, 1u8, 8u8, 2u8, 6u8, 9u8, 3u8, 4u8, 8u8, 1u8, 4u8, 4u8, 5u8, 3u8,
    1u8, 2u8, 5u8, 5u8, 8u8, 2u8, 0u8, 7u8, 6u8, 6u8, 0u8, 9u8, 1u8, 3u8, 4u8,
    6u8, 7u8, 4u8, 0u8, 7u8, 2u8, 2u8, 6u8, 5u8, 6u8, 2u8, 5u8, 2u8, 9u8, 1u8,
    0u8, 3u8, 8u8, 3u8, 0u8, 4u8, 5u8, 6u8, 7u8, 3u8, 3u8, 7u8, 0u8, 3u8, 6u8,
    1u8, 3u8, 2u8, 8u8, 1u8, 2u8, 5u8, 1u8, 4u8, 5u8, 5u8, 1u8, 9u8, 1u8, 5u8,
    2u8, 2u8, 8u8, 3u8, 6u8, 6u8, 8u8, 5u8, 1u8, 8u8, 0u8, 6u8, 6u8, 4u8, 0u8,
    6u8, 2u8, 5u8, 7u8, 2u8, 7u8, 5u8, 9u8, 5u8, 7u8, 6u8, 1u8, 4u8, 1u8, 8u8,
    3u8, 4u8, 2u8, 5u8, 9u8, 0u8, 3u8, 3u8, 2u8, 0u8, 3u8, 1u8, 2u8, 5u8, 3u8,
    6u8, 3u8, 7u8, 9u8, 7u8, 8u8, 8u8, 0u8, 7u8, 0u8, 9u8, 1u8, 7u8, 1u8, 2u8,
    9u8, 5u8, 1u8, 6u8, 6u8, 0u8, 1u8, 5u8, 6u8, 2u8, 5u8, 1u8, 8u8, 1u8, 8u8,
    9u8, 8u8, 9u8, 4u8, 0u8, 3u8, 5u8, 4u8, 5u8, 8u8, 5u8, 6u8, 4u8, 7u8, 5u8,
    8u8, 3u8, 0u8, 0u8, 7u8, 8u8, 1u8, 2u8, 5u8, 9u8, 0u8, 9u8, 4u8, 9u8, 4u8,
    7u8, 0u8, 1u8, 7u8, 7u8, 2u8, 9u8, 2u8, 8u8, 2u8, 3u8, 7u8, 9u8, 1u8, 5u8,
    0u8, 3u8, 9u8, 0u8, 6u8, 2u8, 5u8, 4u8, 5u8, 4u8, 7u8, 4u8, 7u8, 3u8, 5u8,
    0u8, 8u8, 8u8, 6u8, 4u8, 6u8, 4u8, 1u8, 1u8, 8u8, 9u8, 5u8, 7u8, 5u8, 1u8,
    9u8, 5u8, 3u8, 1u8, 2u8, 5u8, 2u8, 2u8, 7u8, 3u8, 7u8, 3u8, 6u8, 7u8, 5u8,
    4u8, 4u8, 3u8, 2u8, 3u8, 2u8, 0u8, 5u8, 9u8, 4u8, 7u8, 8u8, 7u8, 5u8, 9u8,
    7u8, 6u8, 5u8, 6u8, 2u8, 5u8, 1u8, 1u8, 3u8, 6u8, 8u8, 6u8, 8u8, 3u8, 7u8,
    7u8, 2u8, 1u8, 6u8, 1u8, 6u8, 0u8, 2u8, 9u8, 7u8, 3u8, 9u8, 3u8, 7u8, 9u8,
    8u8, 8u8, 2u8, 8u8, 1u8, 2u8, 5u8, 5u8, 6u8, 8u8, 4u8, 3u8, 4u8, 1u8, 8u8,
    8u8, 6u8, 0u8, 8u8, 0u8, 8u8, 0u8, 1u8, 4u8, 8u8, 6u8, 9u8, 6u8, 8u8, 9u8,
    9u8, 4u8, 1u8, 4u8, 0u8, 6u8, 2u8, 5u8, 2u8, 8u8, 4u8, 2u8, 1u8, 7u8, 0u8,
    9u8, 4u8, 3u8, 0u8, 4u8, 0u8, 4u8, 0u8, 0u8, 7u8, 4u8, 3u8, 4u8, 8u8, 4u8,
    4u8, 9u8, 7u8, 0u8, 7u8, 0u8, 3u8, 1u8, 2u8, 5u8, 1u8, 4u8, 2u8, 1u8, 0u8,
    8u8, 5u8, 4u8, 7u8, 1u8, 5u8, 2u8, 0u8, 2u8, 0u8, 0u8, 3u8, 7u8, 1u8, 7u8,
    4u8, 2u8, 2u8, 4u8, 8u8, 5u8, 3u8, 5u8, 1u8, 5u8, 6u8, 2u8, 5u8, 7u8, 1u8,
    0u8, 5u8, 4u8, 2u8, 7u8, 3u8, 5u8, 7u8, 6u8, 0u8, 1u8, 0u8, 0u8, 1u8, 8u8,
    5u8, 8u8, 7u8, 1u8, 1u8, 2u8, 4u8, 2u8, 6u8, 7u8, 5u8, 7u8, 8u8, 1u8, 2u8,
    5u8, 3u8, 5u8, 5u8, 2u8, 7u8, 1u8, 3u8, 6u8, 7u8, 8u8, 8u8, 0u8, 0u8, 5u8,
    0u8, 0u8, 9u8, 2u8, 9u8, 3u8, 5u8, 5u8, 6u8, 2u8, 1u8, 3u8, 3u8, 7u8, 8u8,
    9u8, 0u8, 6u8, 2u8, 5u8, 1u8, 7u8, 7u8, 6u8, 3u8, 5u8, 6u8, 8u8, 3u8, 9u8,
    4u8, 0u8, 0u8, 2u8, 5u8, 0u8, 4u8, 6u8, 4u8, 6u8, 7u8, 7u8, 8u8, 1u8, 0u8,
    6u8, 6u8, 8u8, 9u8, 4u8, 5u8, 3u8, 1u8, 2u8, 5u8, 8u8, 8u8, 8u8, 1u8, 7u8,
    8u8, 4u8, 1u8, 9u8, 7u8, 0u8, 0u8, 1u8, 2u8, 5u8, 2u8, 3u8, 2u8, 3u8, 3u8,
    8u8, 9u8, 0u8, 5u8, 3u8, 3u8, 4u8, 4u8, 7u8, 2u8, 6u8, 5u8, 6u8, 2u8, 5u8,
    4u8, 4u8, 4u8, 0u8, 8u8, 9u8, 2u8, 0u8, 9u8, 8u8, 5u8, 0u8, 0u8, 6u8, 2u8,
    6u8, 1u8, 6u8, 1u8, 6u8, 9u8, 4u8, 5u8, 2u8, 6u8, 6u8, 7u8, 2u8, 3u8, 6u8,
    3u8, 2u8, 8u8, 1u8, 2u8, 5u8, 2u8, 2u8, 2u8, 0u8, 4u8, 4u8, 6u8, 0u8, 4u8,
    9u8, 2u8, 5u8, 0u8, 3u8, 1u8, 3u8, 0u8, 8u8, 0u8, 8u8, 4u8, 7u8, 2u8, 6u8,
    3u8, 3u8, 3u8, 6u8, 1u8, 8u8, 1u8, 6u8, 4u8, 0u8, 6u8, 2u8, 5u8, 1u8, 1u8,
    1u8, 0u8, 2u8, 2u8, 3u8, 0u8, 2u8, 4u8, 6u8, 2u8, 5u8, 1u8, 5u8, 6u8, 5u8,
    4u8, 0u8, 4u8, 2u8, 3u8, 6u8, 3u8, 1u8, 6u8, 6u8, 8u8, 0u8, 9u8, 0u8, 8u8,
    2u8, 0u8, 3u8, 1u8, 2u8, 5u8, 5u8, 5u8, 5u8, 1u8, 1u8, 1u8, 5u8, 1u8, 2u8,
    3u8, 1u8, 2u8, 5u8, 7u8, 8u8, 2u8, 7u8, 0u8, 2u8, 1u8, 1u8, 8u8, 1u8, 5u8,
    8u8, 3u8, 4u8, 0u8, 4u8, 5u8, 4u8, 1u8, 0u8, 1u8, 5u8, 6u8, 2u8, 5u8, 2u8,
    7u8, 7u8, 5u8, 5u8, 5u8, 7u8, 5u8, 6u8, 1u8, 5u8, 6u8, 2u8, 8u8, 9u8, 1u8,
    3u8, 5u8, 1u8, 0u8, 5u8, 9u8, 0u8, 7u8, 9u8, 1u8, 7u8, 0u8, 2u8, 2u8, 7u8,
    0u8, 5u8, 0u8, 7u8, 8u8, 1u8, 2u8, 5u8, 1u8, 3u8, 8u8, 7u8, 7u8, 7u8, 8u8,
    7u8, 8u8, 0u8, 7u8, 8u8, 1u8, 4u8, 4u8, 5u8, 6u8, 7u8, 5u8, 5u8, 2u8, 9u8,
    5u8, 3u8, 9u8, 5u8, 8u8, 5u8, 1u8, 1u8, 3u8, 5u8, 2u8, 5u8, 3u8, 9u8, 0u8,
    6u8, 2u8, 5u8, 6u8, 9u8, 3u8, 8u8, 8u8, 9u8, 3u8, 9u8, 0u8, 3u8, 9u8, 0u8,
    7u8, 2u8, 2u8, 8u8, 3u8, 7u8, 7u8, 6u8, 4u8, 7u8, 6u8, 9u8, 7u8, 9u8, 2u8,
    5u8, 5u8, 6u8, 7u8, 6u8, 2u8, 6u8, 9u8, 5u8, 3u8, 1u8, 2u8, 5u8, 3u8, 4u8,
    6u8, 9u8, 4u8, 4u8, 6u8, 9u8, 5u8, 1u8, 9u8, 5u8, 3u8, 6u8, 1u8, 4u8, 1u8,
    8u8, 8u8, 8u8, 2u8, 3u8, 8u8, 4u8, 8u8, 9u8, 6u8, 2u8, 7u8, 8u8, 3u8, 8u8,
    1u8, 3u8, 4u8, 7u8, 6u8, 5u8, 6u8, 2u8, 5u8, 1u8, 7u8, 3u8, 4u8, 7u8, 2u8,
    3u8, 4u8, 7u8, 5u8, 9u8, 7u8, 6u8, 8u8, 0u8, 7u8, 0u8, 9u8, 4u8, 4u8, 1u8,
    1u8, 9u8, 2u8, 4u8, 4u8, 8u8, 1u8, 3u8, 9u8, 1u8, 9u8, 0u8, 6u8, 7u8, 3u8,
    8u8, 2u8, 8u8, 1u8, 2u8, 5u8, 8u8, 6u8, 7u8, 3u8, 6u8, 1u8, 7u8, 3u8, 7u8,
    9u8, 8u8, 8u8, 4u8, 0u8, 3u8, 5u8, 4u8, 7u8, 2u8, 0u8, 5u8, 9u8, 6u8, 2u8,
    2u8, 4u8, 0u8, 6u8, 9u8, 5u8, 9u8, 5u8, 3u8, 3u8, 6u8, 9u8, 1u8, 4u8, 0u8,
    6u8, 2u8, 5u8
  ];

  const DECIMAL_LEFT_SHIFT_TABLE: [u16; 65] = [
    0u16, 0x800u16, 0x801u16, 0x803u16, 0x1006u16, 0x1009u16, 0x100du16,
    0x1812u16, 0x1817u16, 0x181du16, 0x2024u16, 0x202bu16, 0x2033u16,
    0x203cu16, 0x2846u16, 0x2850u16, 0x285bu16, 0x3067u16, 0x3073u16,
    0x3080u16, 0x388eu16, 0x389cu16, 0x38abu16, 0x38bbu16, 0x40ccu16,
    0x40ddu16, 0x40efu16, 0x4902u16, 0x4915u16, 0x4929u16, 0x513eu16,
    0x5153u16, 0x5169u16, 0x5180u16, 0x5998u16, 0x59b0u16, 0x59c9u16,
    0x61e3u16, 0x61fdu16, 0x6218u16, 0x6a34u16, 0x6a50u16, 0x6a6du16,
    0x6a8bu16, 0x72aau16, 0x72c9u16, 0x72e9u16, 0x7b0au16, 0x7b2bu16,
    0x7b4du16, 0x8370u16, 0x8393u16, 0x83b7u16, 0x83dcu16, 0x8c02u16,
    0x8c28u16, 0x8c4fu16, 0x9477u16, 0x949fu16, 0x94c8u16, 0x9cf2u16, 0x51cu16,
    0x51cu16, 0x51cu16, 0x51cu16
  ];

  const MAX_SHIFT_AMOUNT: u32 = (core::mem::size_of::<u64>() - 4) as u32;

  pub fn new<T: Into<CharToAscii> + Copy + MatchChar>(
    src: &[T],
    numeric: &NumericObject,
    ctype: &CtypeObject
  ) -> Result<Self, c_int> {
    let decimal_point = numeric.get_decimal_point().unwrap_or('\0');

    let mut current = 0usize;
    let mut saw_decimal_point = false;
    let mut exponenta = 0i32;
    let mut total_digits = 0u32;
    let mut ndigits = 0usize;
    let mut digits = [0u8; MAX_NUM_DIGITS];
    let mut trunc = false;

    loop {
      if let Some(num) = get_char_with_index(src, current) &&
        (ctype.casemap.isdigit)(num as u32)
      {
        let digit =
          b36_char_to_int(num).and_then(|c| Some(c as u8)).unwrap_or(0);
        if digit == 0 && ndigits == 0 {
          exponenta -= 1;
          current += 1;
          continue;
        }
        total_digits += 1;
        if ndigits < MAX_NUM_DIGITS {
          digits[ndigits] = digit;
          ndigits += 1;
        } else if digit != 0 {
          trunc = true;
        }
        current += 1;
        continue;
      }
      if T::char_matches(decimal_point, src, current) {
        if saw_decimal_point {
          break;
        }
        exponenta = total_digits as i32;
        saw_decimal_point = true;
        current += 1;
        continue;
      }

      break;
    }

    if !saw_decimal_point {
      exponenta = total_digits as i32;
    }

    if let Some(num) = get_char_with_index(src, current) &&
      (ctype.casemap.tolower)(num as u32) == 'e' as u32
    {
      current += 1;
      if let Some(next) = get_char_with_index(src, current) &&
        ((ctype.casemap.isdigit)(next as u32) || next == '+' || next == '-')
      {
        let r: strtoint::StrToIntResult<i32> =
          strtoint::strtoint(&src[current..], 10, ctype);
        if r.error != 0 {
          return Err(r.error);
        }
        let add = r.value;
        let mut e: i64 = i64::from(exponenta) + i64::from(add);
        if e > (1 << 30) {
          e = 1 << 30;
        } else if e < -(1 << 30) {
          e = -(1 << 30);
        }
        exponenta = e as i32;
      }
    }

    let mut result = Self { exponenta, ndigits, trunc, digits };
    result.trim_trailing_zeroes();

    Ok(result)
  }

  // https://github.com/lemire/fast_float/blob/48c017aa963aa7d419c43261e83986ea71b9679f/include/fast_float/simple_decimal_conversion.h#L33
  fn generate_new_digits(
    &self,
    shift_amount: u32
  ) -> u32 {
    let shift = (shift_amount & 63) as usize;

    let xa: u32 = Self::DECIMAL_LEFT_SHIFT_TABLE[shift].into();
    let xb: u32 = Self::DECIMAL_LEFT_SHIFT_TABLE[shift + 1].into();

    let new_digits: u32 = xa.wrapping_shr(11);
    let pow5a: u32 = 0x7FFu32 & xa;
    let pow5b: u32 = 0x7FFu32 & xb;

    let n = (pow5b - pow5a) as usize;
    let off = (pow5a as usize) + n;
    let pow5: &[u8] = &Self::SHIFT_POW5[(pow5a as usize)..off];

    for (digit_index, p) in pow5.iter().enumerate() {
      if digit_index >= self.ndigits {
        return new_digits - 1;
      }
      match self.digits.get(digit_index) {
        | Some(d) if d == p => continue,
        | Some(d) if d < p => return new_digits - 1,
        | _ => return new_digits
      }
    }
    new_digits
  }

  fn trim_trailing_zeroes(&mut self) {
    while self.ndigits > 0 && self.digits[self.ndigits - 1] == 0u8 {
      self.ndigits -= 1;
    }
    if self.ndigits == 0 {
      self.exponenta = 0;
    }
  }

  fn should_round_up(
    &self,
    rtd: i32,
    round: Rounding
  ) -> bool {
    if rtd < 0 || (rtd as usize) >= self.ndigits {
      return false;
    }

    let rtd = rtd as usize;

    if round == Rounding::Upward {
      return true;
    } else if round == Rounding::Downward {
      return false;
    }

    if self.digits[rtd] == 5 && rtd + 1 == self.ndigits {
      if self.trunc {
        return true;
      }

      if rtd == 0 {
        return false;
      }

      return self.digits[rtd - 1] % 2 != 0;
    }

    self.digits[rtd] >= 5
  }

  fn left_shift(
    &mut self,
    shift: u32
  ) {
    let nd = self.generate_new_digits(shift);

    let mut ri: isize = (self.ndigits - 1) as isize;
    let mut wi: usize = self.ndigits + (nd as usize);

    let mut acc: u64 = 0;

    while ri >= 0 {
      acc += u64::from(self.digits[ri as usize]) << u64::from(shift);
      let na = acc / 10u64;
      let wd = acc - (10u64 * na);
      wi -= 1;
      if wi < MAX_NUM_DIGITS {
        self.digits[wi] = (wd) as u8;
      } else if wd != 0 {
        self.trunc = true;
      }
      acc = na;
      ri -= 1;
    }

    // write remaining
    while acc > 0 {
      let na = acc / 10u64;
      let wd = acc - (10u64 * na);
      wi -= 1;
      if wi < MAX_NUM_DIGITS {
        self.digits[wi] = (wd) as u8;
      } else if wd != 0 {
        self.trunc = true;
      }
      acc = na;
    }

    self.ndigits += nd as usize;
    if self.ndigits > MAX_NUM_DIGITS {
      self.ndigits = MAX_NUM_DIGITS;
    }
    self.exponenta += (nd) as i32;
    self.trim_trailing_zeroes();
  }

  fn right_shift(
    &mut self,
    shift: u32
  ) {
    let mut ri = 0usize;
    let mut wi = 0usize;

    let mut acc = 0u64;
    let sm = (1u64 << u64::from(shift)) - 1u64;

    while acc >> u64::from(shift) == 0 {
      let rd: u64 = if (ri as usize) < self.ndigits {
        u64::from(self.digits[ri])
      } else {
        0u64
      };
      acc = acc * 10u64 + rd;
      ri += 1;
    }

    self.exponenta -= ri as i32 - 1;

    while ri < self.ndigits {
      let rd = u64::from(self.digits[ri]);
      let wd = acc >> u64::from(shift);
      acc &= sm;
      self.digits[wi] = (wd) as u8;
      acc = acc * 10u64 + rd;
      ri += 1;
      wi += 1;
    }

    while acc > 0 {
      let wd = acc >> u64::from(shift);
      acc &= sm;
      if wi < MAX_NUM_DIGITS {
        self.digits[wi] = (wd) as u8;
        wi += 1;
      } else if wd > 0 {
        self.trunc = true;
      }
      acc = acc * 10u64;
    }

    self.ndigits = wi;
    self.trim_trailing_zeroes();
  }

  pub fn shift(
    &mut self,
    mut shift: i32
  ) {
    if shift == 0 {
      return;
    } else if shift > 0 {
      while shift > Self::MAX_SHIFT_AMOUNT as i32 {
        self.left_shift(Self::MAX_SHIFT_AMOUNT);
        shift -= Self::MAX_SHIFT_AMOUNT as i32;
      }
      self.left_shift(shift as u32);
    } else {
      while shift < -(Self::MAX_SHIFT_AMOUNT as i32) {
        self.right_shift(Self::MAX_SHIFT_AMOUNT);
        shift += Self::MAX_SHIFT_AMOUNT as i32;
      }
      self.right_shift((-shift) as u32);
    }
  }

  pub fn get_mantissa<I>(
    &self,
    round: Rounding
  ) -> I
  where
    I: From<u32>
      + num_traits::PrimInt
      + num_traits::NumAssignOps
      + num_traits::Zero {
    let mut result = I::zero();
    let mut current = 0u32;

    while (current as i32) < self.exponenta && (current as usize) < self.ndigits
    {
      result =
        result * 10u32.into() + u32::from(self.digits[current as usize]).into();
      current += 1;
    }

    while (current as i32) < self.exponenta {
      result *= 10u32.into();
      current += 1;
    }

    let do_round = u32::from(self.should_round_up(self.exponenta, round));

    result + do_round.into()
  }
}
