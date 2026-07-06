// An implementation of Ryu double to fixed algoritghm made by Ulf Adams.
//
// Some parts have been taken from LLVM libc
//

use {
  super::ryu_table::*,
  crate::support::{float::rounding_mode::Rounding, traits::float::FloatBits},
  core::ascii
};

pub struct RyuReturn {
  pub digits: [ascii::Char; 2048],
  pub ndigits: usize,
  pub exponenta: i32
}

impl Default for RyuReturn {
  #[inline]
  fn default() -> Self {
    Self { digits: [ascii::Char::Null; 2048], ndigits: 0, exponenta: 0 }
  }
}

#[inline]
fn resolve_round_up(
  round: &Rounding,
  last_digit: u32,
  trailing_zeros: bool
) -> i32 {
  match round {
    | Rounding::TowardZero | Rounding::Downward => 0,
    | Rounding::Upward => {
      if last_digit != 0 || !trailing_zeros {
        1
      } else {
        0
      }
    },
    | Rounding::ToNearest => i32::from(last_digit >= 5),
    | Rounding::Even => {
      if last_digit != 5 || !trailing_zeros {
        i32::from(last_digit > 5)
      } else {
        2
      }
    },
  }
}

#[inline]
fn decimal_len_9(v: u32) -> usize {
  // Function precondition: v is not a 10-digit number.
  // (f2s: 9 digits are sufficient for round-tripping.)
  // (d2fixed: We print 9-digit blocks.)
  debug_assert!(v < 1000000000);
  if v >= 100000000 {
    return 9;
  }
  if v >= 10000000 {
    return 8;
  }
  if v >= 1000000 {
    return 7;
  }
  if v >= 100000 {
    return 6;
  }
  if v >= 10000 {
    return 5;
  }
  if v >= 1000 {
    return 4;
  }
  if v >= 100 {
    return 3;
  }
  if v >= 10 {
    return 2;
  }
  return 1;
}

#[inline]
fn log10_pow2(e: u32) -> u32 {
  (e * 78913) >> 18
}

#[inline]
fn pow5_factor(v: u64) -> u32 {
  let m_inv_5 = 14757395258967641293u64;
  let n_div_5 = 3689348814741910323u64;
  let mut v = v;
  let mut cnt = 0u32;
  loop {
    debug_assert!(v != 0);
    v *= m_inv_5;
    if v > n_div_5 {
      break;
    }
    cnt += 1;
  }
  cnt
}

#[inline]
fn multiple_of_pow2(
  v: u64,
  p: u32
) -> bool {
  let p = u64::from(p);
  (v & ((1u64 << p) - 1)) == 0
}

#[inline]
fn multiple_of_pow5(
  v: u64,
  p: u32
) -> bool {
  pow5_factor(v) >= p
}

// (u128, u128) -> (product_low, product_high)
#[inline]
fn umul256(
  a: u128,
  bhi: u64,
  blo: u64
) -> (u128, u128) {
  let alo: u64 = a as u64;
  let ahi: u64 = (a >> 64) as u64;

  let b00: u128 = u128::from(alo) * u128::from(blo);
  let b01: u128 = u128::from(alo) * u128::from(bhi);
  let b10: u128 = u128::from(ahi) * u128::from(blo);
  let b11: u128 = u128::from(ahi) * u128::from(bhi);

  let b00lo: u64 = b00 as u64;
  let b00hi: u64 = (b00 >> 64) as u64;

  let mid1: u128 = b10 + u128::from(b00hi);
  let mid1lo: u64 = mid1 as u64;
  let mid1hi: u64 = (mid1 >> 64) as u64;

  let mid2: u128 = b01 + u128::from(mid1lo);
  let mid2lo: u64 = mid2 as u64;
  let mid2hi: u64 = (mid2 >> 64) as u64;

  let phi: u128 = b11 + u128::from(mid1hi) + u128::from(mid2hi);
  let plo: u128 = (u128::from(mid2lo) << 64) | u128::from(b00lo);

  (plo, phi)
}

#[inline]
fn umul256_hi(
  a: u128,
  bhi: u64,
  blo: u64
) -> u128 {
  let (_, hi) = umul256(a, bhi, blo);
  hi
}

#[inline]
fn u128_mod1e9(v: u128) -> u32 {
  // After multiplying, we're going to shift right by 29, then truncate
  // to uint32_t.
  // This means that we need only 29 + 32 = 61 bits, so we can truncate
  // to uint64_t before shifting.
  let mult: u64 = umul256_hi(v, 0x89705F4136B4A597, 0x31680A88F8953031) as u64;

  let shifted: u32 = (mult >> 29) as u32;
  (v as u32).wrapping_sub(1000000000u32.wrapping_mul(shifted))
}

#[inline]
fn mul_shift_mod_1e9(
  mant: u64,
  mul: [u64; 3],
  shift: i32
) -> u32 {
  debug_assert!(
    (128..=180).contains(&shift),
    "mul_shift_mod_1e9: shift out of range"
  );

  let b0: u128 = u128::from(mant) * u128::from(mul[0]);
  let b1: u128 = u128::from(mant) * u128::from(mul[1]);
  let b2: u128 = u128::from(mant) * u128::from(mul[2]);

  let r: u64 = (b0 >> 64) as u64;
  let mid: u128 = b1 + u128::from(r);

  let s: u64 = (mid >> 64) as u64;
  let s1: u128 = b2 + u128::from(s);

  u128_mod1e9(s1 >> u128::from((shift - 128i32) as u32))
}

#[inline]
fn append_n_digits(
  olen: usize,
  block: u32,
  buffer: &mut [ascii::Char]
) {
  let mut i = 0usize;
  let mut block = block;

  while block >= 10000 {
    let c: u32 = block % 10000u32;
    block /= 10000u32;

    let c0 = (c % 100) << 1u32;
    let c1 = (c / 100) << 1u32;

    let off1 = olen - i - 2usize;
    let off2 = olen - i - 4usize;

    let c0 = c0 as usize;
    let c1 = c1 as usize;
    let d1 = &DIGIT_TABLE[c0..c0 + 2];
    let d2 = &DIGIT_TABLE[c1..c1 + 2];
    buffer[off1..off1 + 2].copy_from_slice(d1);
    buffer[off2..off2 + 2].copy_from_slice(d2);

    i += 4;
  }
  if block >= 100 {
    let c: u32 = (block % 100u32) << 1u32;
    block /= 100u32;

    let off = olen - i - 2usize;

    let c = c as usize;
    let d = &DIGIT_TABLE[c..c + 2];
    buffer[off..off + 2].copy_from_slice(d);

    i += 2;
  }
  if block >= 10 {
    let c: u32 = block << 1u32;
    let off = olen - i - 2usize;
    let c = c as usize;
    let d = &DIGIT_TABLE[c..c + 2];
    buffer[off..off + 2].copy_from_slice(d);
  } else {
    let a = ('0' as u32 + block) as u8;
    buffer[0] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
  }
}

#[inline]
fn append_c_digits(
  count: usize,
  block: u32,
  buffer: &mut [ascii::Char]
) {
  let mut block = block;
  let mut i = 0usize;
  while i < count - 1 {
    let c: u32 = (block % 100) << 1u32;
    block /= 100u32;

    let off = count - i - 2usize;

    let c = c as usize;
    let d = &DIGIT_TABLE[c..c + 2];
    buffer[off..off + 2].copy_from_slice(d);

    i += 2;
  }
  if i < count {
    let a = ('0' as u32 + (block % 10)) as u8;
    buffer[count - i - 1] =
      ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
  }
}

#[inline]
fn append_nine_digits(
  block: u32,
  buffer: &mut [ascii::Char]
) {
  let mut block = block;
  if block == 0 {
    for i in 0usize..9usize {
      buffer[i] = ascii::Char::Digit0;
    }
    return;
  }
  let mut i = 0usize;
  while i < 5 {
    let c: u32 = block % 10000u32;
    block /= 10000u32;

    let c0 = (c % 100) << 1u32;
    let c1 = (c / 100) << 1u32;

    let off1 = 7usize - i;
    let off2 = 5usize - i;

    let c0 = c0 as usize;
    let c1 = c1 as usize;
    let d1 = &DIGIT_TABLE[c0..c0 + 2];
    let d2 = &DIGIT_TABLE[c1..c1 + 2];
    buffer[off1..off1 + 2].copy_from_slice(d1);
    buffer[off2..off2 + 2].copy_from_slice(d2);

    i += 4;
  }
  let a = ('0' as u32 + block) as u8;
  buffer[0] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
}

#[inline]
fn append_d_digits(
  olen: usize,
  block: u32,
  buffer: &mut [ascii::Char]
) {
  let mut i = 0usize;
  let mut block = block;

  while block >= 10000 {
    let c: u32 = block % 10000u32;
    block /= 10000u32;

    let c0 = (c % 100) << 1u32;
    let c1 = (c / 100) << 1u32;

    let off1 = olen - i - 2usize;
    let off2 = olen - i - 4usize;

    let c0 = c0 as usize;
    let c1 = c1 as usize;
    let d1 = &DIGIT_TABLE[c0..c0 + 2];
    let d2 = &DIGIT_TABLE[c1..c1 + 2];
    buffer[off1..off1 + 2].copy_from_slice(d1);
    buffer[off2..off2 + 2].copy_from_slice(d2);

    i += 4;
  }
  if block >= 100 {
    let c: u32 = (block % 100u32) << 1u32;
    block /= 100u32;

    let off = olen - i - 2usize;

    let c = c as usize;
    let d = &DIGIT_TABLE[c..c + 2];
    buffer[off..off + 2].copy_from_slice(d);
  }
  if block >= 10 {
    let c: u32 = block << 1u32;
    let c = c as usize;
    buffer[0] = DIGIT_TABLE[c];
    buffer[1] = DIGIT_TABLE[c + 1];
  } else {
    let a = ('0' as u32 + block) as u8;
    buffer[0] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
  }
}

#[inline]
fn index_for_exp(e: u32) -> u32 {
  (e + 15) / 16
}

#[inline]
fn len_for_index(idx: u32) -> u32 {
  (log10_pow2(16 * idx) + 1 + 16 + 8) / 9
}

#[inline]
pub fn format_ryu(
  value: f64,
  precision: i32,
  round: Rounding
) -> RyuReturn {
  let mantissa = value.get_explicit_mantissa();
  let exponent = value.get_explicit_exponent() - f64::FRACTION_LEN as i32;

  let mut buffer = [ascii::Char::Null; 2048];
  let mut index = 0usize;
  let mut nonzero = false;

  // Get positive block
  if exponent >= -(f64::FRACTION_LEN as i32) {
    let idx: u32 =
      if exponent < 0 { 0 } else { index_for_exp(exponent as u32) };
    let pos_exp: u32 = idx * IDX_SIZE;
    let len: i32 = len_for_index(idx) as i32;

    let mut block_index = len - 1;
    while block_index >= 0 {
      let shift_amount =
        TABLE_SHIFT_CONST + pos_exp.wrapping_sub(exponent as u32);

      let pow10_offset: usize = POW10_OFFSET[idx as usize] as usize;
      let val = POW10_SPLIT[pow10_offset + block_index as usize];

      let block: u32 =
        mul_shift_mod_1e9(mantissa << 8, val, (shift_amount + 8) as i32);

      if nonzero {
        append_nine_digits(block, &mut buffer[index..]);
        index += 9;
      } else if block != 0 {
        let olen = decimal_len_9(block);
        append_n_digits(olen, block, &mut buffer[index..]);
        index += olen;
        nonzero = true;
      }
      block_index -= 1;
    }
  }

  if !nonzero {
    buffer[index] = ascii::Char::Digit0;
    index += 1;
  }

  let mut exponenta = index as i32 - 1;

  // Get negative block
  if exponent < 0 {
    let mut round_up = 0i32;

    let idx = -exponent / IDX_SIZE as i32;

    let blocks: u32 = (precision as u32) / 9 + 1;
    let mut i: u32 = if blocks <= (MIN_BLOCK_2[idx as usize] as u32) {
      let r = blocks;
      let n = precision as usize;
      buffer[index..index + n].fill(ascii::Char::Digit0);
      index += n;
      r
    } else {
      let r = MIN_BLOCK_2[idx as usize];
      let n = 9 * (r as usize);
      buffer[index..index + n].fill(ascii::Char::Digit0);
      index += n;
      r as u32
    };

    while i < blocks {
      let shift_amount: i32 =
        (ADDITIONAL_BITS_2 as i32) + (-exponent - (IDX_SIZE as i32) * idx);

      let off: u32 = POW10_OFFSET_2[idx as usize] as u32;
      let min: u32 = MIN_BLOCK_2[idx as usize] as u32;
      let p: u32 = off + (i as u32) - min;

      if p >= POW10_OFFSET_2[idx as usize + 1] as u32 {
        let fill = (precision as u32 - 9 * i) as usize;
        buffer[index..index + fill].fill(ascii::Char::Digit0);
        index += fill;
        break;
      };

      let mut digits = mul_shift_mod_1e9(
        mantissa << 8,
        POW10_SPLIT_2[p as usize],
        shift_amount + 8
      );

      if i < blocks - 1 {
        append_nine_digits(digits, &mut buffer[index..]);
        index += 9;
      } else {
        let max: usize = ((precision as u32) - 9 * i) as usize;
        let mut last_digit = 0u32;

        let mut k = 0usize;
        while k < 9 - max {
          last_digit = digits % 10;
          digits /= 10;
          k += 1;
        }

        // 0 = don't round up; 1 = round up unconditionally; 2 = round up if odd.
        let req2: i32 = -exponent - precision - 1;
        let trailing_zeros =
          req2 <= 0 || (req2 < 60 && multiple_of_pow2(mantissa, req2 as u32));
        round_up = resolve_round_up(&round, last_digit, trailing_zeros);
        if max > 0 {
          append_c_digits(max, digits, &mut buffer[index..]);
          index += max;
        }
        break;
      }

      i += 1;
    }

    // Apply rounding rules
    if round_up != 0 {
      let mut round_index = index as i32;
      loop {
        round_index -= 1;

        if round_index == -1 {
          buffer[0] = ascii::Char::Digit1;
          exponenta += 1;
          buffer[index] = ascii::Char::Digit0;
          index += 1;
          break;
        }

        let c = buffer[round_index as usize];

        if c == ascii::Char::Digit9 {
          buffer[round_index as usize] = ascii::Char::Digit0;
          round_up = 1;
          continue;
        } else {
          if round_up == 2 && c.to_u8() % 2 == 0 {
            break;
          }
          let incremented = c.to_u8() + 1;
          buffer[round_index as usize] =
            ascii::Char::from_u8(incremented).unwrap_or(ascii::Char::Null);
          break;
        }
      }
    }
  } else {
    let n = precision as usize;
    buffer[index..index + n].fill(ascii::Char::Digit0);
    index += n;
  }

  let result = RyuReturn { digits: buffer, ndigits: index, exponenta };
  result
}

#[inline]
pub fn format_ryu_exp(
  value: f64,
  precision: i32,
  round: Rounding
) -> RyuReturn {
  let mantissa = value.get_explicit_mantissa();
  let exponent = value.get_explicit_exponent() - f64::FRACTION_LEN as i32;

  let print_decimal_pont = precision > 0;

  let mut buffer = [ascii::Char::Null; 2048];
  let mut index = 0usize;
  let mut exponenta = 0i32;
  let mut precision = precision - 1;
  let mut block: u32 = 0;
  let mut printed = 0usize;
  let mut avail = 0usize;

  precision += 1;

  // Get positive block
  if exponent >= -(f64::FRACTION_LEN as i32) {
    let idx: u32 =
      if exponent < 0 { 0 } else { index_for_exp(exponent as u32) };
    let pos_exp: u32 = idx * IDX_SIZE;
    let len: i32 = len_for_index(idx) as i32;

    let mut block_index = len - 1;
    while block_index >= 0 {
      let shift_amount =
        TABLE_SHIFT_CONST + pos_exp.wrapping_sub(exponent as u32);

      let pow10_offset: usize = POW10_OFFSET[idx as usize] as usize;
      let val = POW10_SPLIT[pow10_offset + block_index as usize];

      block = mul_shift_mod_1e9(mantissa << 8, val, (shift_amount + 8) as i32);

      if printed != 0 {
        if printed + 9 > precision as usize {
          avail = 9;
          break;
        }
        append_nine_digits(block, &mut buffer[index..]);
        index += 9;
        printed += 9;
      } else if block != 0 {
        avail = decimal_len_9(block);
        exponenta = block_index * 9 + avail as i32 - 1;
        if avail as i32 > precision {
          break;
        }
        if print_decimal_pont {
          append_d_digits(avail, block, &mut buffer[index..]);
          index += avail;
        } else {
          let a = ('0' as u32 + block) as u8;
          buffer[index] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
          index += 1;
        }
        printed = avail;
        avail = 0;
      }

      block_index -= 1;
    }
  }

  // Get negative block
  if exponent < 0 && avail == 0 {
    let idx = -exponent / IDX_SIZE as i32;

    let mut block_index: i32 = MIN_BLOCK_2[idx as usize] as i32;

    while block_index < 200 {
      let shift_amount: i32 =
        (ADDITIONAL_BITS_2 as i32) + (-exponent - (IDX_SIZE as i32) * idx);

      let off: u32 = POW10_OFFSET_2[idx as usize] as u32;
      let min: u32 = MIN_BLOCK_2[idx as usize] as u32;
      let p: u32 = off + (block_index as u32) - min;

      block = if p >= POW10_OFFSET_2[idx as usize + 1] as u32 {
        0
      } else {
        mul_shift_mod_1e9(
          mantissa << 8,
          POW10_SPLIT_2[p as usize],
          shift_amount + 8
        )
      };

      if printed != 0 {
        if printed + 9 > precision as usize {
          avail = 9;
          break;
        }
        append_nine_digits(block, &mut buffer[index..]);
        index += 9;
        printed += 9;
      } else if block != 0 {
        avail = decimal_len_9(block);
        exponenta = -(block_index + 1) * 9 + avail as i32 - 1;
        if avail > precision as usize {
          break;
        }
        if print_decimal_pont {
          append_d_digits(avail, block, &mut buffer[index..]);
          index += avail;
        } else {
          let a = ('0' as u32 + block) as u8;
          buffer[index] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
          index += 1;
        }
        printed = avail;
        avail = 0;
      }

      block_index += 1;
    }
  }

  let max: usize = (precision - printed as i32) as usize;
  if avail == 0 {
    block = 0;
  }

  let mut last_digit = 0u32;
  if avail > max {
    let mut k = 0usize;
    while k < avail - max {
      last_digit = block % 10;
      block /= 10;
      k += 1;
    }
  }

  // 0 = don't round up; 1 = round up unconditionally; 2 = round up if odd.
  let rexp: i32 = precision - exponenta;
  let req2: i32 = -exponent - rexp;
  let mut trailing_zeros =
    req2 <= 0 || (req2 < 60 && multiple_of_pow2(mantissa, req2 as u32));
  if rexp < 0 {
    let req5: i32 = -rexp;
    trailing_zeros = trailing_zeros && multiple_of_pow5(mantissa, req5 as u32);
  }
  let mut round_up = resolve_round_up(&round, last_digit, trailing_zeros);

  if printed != 0 {
    if block == 0 {
      buffer[index..index + max].fill(ascii::Char::Digit0);
    } else {
      append_c_digits(max, block, &mut buffer[index..]);
    }
    index += max;
  } else {
    if print_decimal_pont {
      append_d_digits(max, block, &mut buffer[index..]);
      index += max;
    } else {
      let a = ('0' as u32 + block) as u8;
      buffer[index] = ascii::Char::from_u8(a).unwrap_or(ascii::Char::Null);
      index += 1;
    }
  }

  // Apply rounding rules
  if round_up != 0 {
    let mut round_index = index as i32;
    loop {
      round_index -= 1;

      if round_index == -1 {
        buffer[0] = ascii::Char::Digit1;
        exponenta += 1;
        break;
      }

      let c = buffer[round_index as usize];

      if c == ascii::Char::Digit9 {
        buffer[round_index as usize] = ascii::Char::Digit0;
        round_up = 1;
        continue;
      } else {
        if round_up == 2 && c.to_u8() % 2 == 0 {
          break;
        }
        let incremented = c.to_u8() + 1;
        buffer[round_index as usize] =
          ascii::Char::from_u8(incremented).unwrap_or(ascii::Char::Null);
        break;
      }
    }
  }

  let result = RyuReturn { digits: buffer, ndigits: index, exponenta };
  result
}
