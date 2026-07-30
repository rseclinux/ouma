use {
  super::b36_char_to_int,
  crate::{
    std::errno,
    support::{
      locale::ctype::CtypeObject,
      traits::char::{CharToAscii, get_char_with_index}
    }
  },
  bnum::cast::CastFrom
};

#[inline]
fn has_prefix<T: Copy + Into<CharToAscii>>(
  src: &[T],
  prefix: char,
  ctype: &CtypeObject
) -> bool {
  get_char_with_index(src, 1).map(|c| (ctype.casemap.tolower)(c as u32)) ==
    Some(prefix as u32)
}

#[inline]
fn prefix_has_valid_digit<T: Copy + Into<CharToAscii>>(
  src: &[T],
  radix: i32
) -> bool {
  get_char_with_index(src, 2)
    .and_then(|c| b36_char_to_int(c))
    .map(|v| v < radix as u32)
    .unwrap_or(false)
}

#[inline]
fn is_bin_start<T: Copy + Into<CharToAscii>>(
  src: &[T],
  ctype: &CtypeObject
) -> bool {
  has_prefix(src, 'b', ctype)
}

#[inline]
fn is_oct_start<T: Copy + Into<CharToAscii>>(
  src: &[T],
  ctype: &CtypeObject
) -> bool {
  has_prefix(src, 'o', ctype)
}

#[inline]
fn is_hex_start<T: Copy + Into<CharToAscii>>(
  src: &[T],
  ctype: &CtypeObject
) -> bool {
  has_prefix(src, 'x', ctype)
}

#[inline]
fn infer_base<T: Into<CharToAscii> + Copy>(
  src: &[T],
  ctype: &CtypeObject
) -> i32 {
  if is_hex_start(src, ctype) && prefix_has_valid_digit(src, 16) {
    return 16;
  }
  if is_oct_start(src, ctype) {
    return 8;
  }
  if is_bin_start(src, ctype) {
    return 2;
  }

  if get_char_with_index(src, 0) == Some('0') {
    return 8;
  }

  10
}

#[derive(Clone, Copy, Debug)]
pub struct StrToIntResult<T: num_traits::PrimInt> {
  pub value: T,
  pub len: usize,
  pub error: i32
}

impl<T: num_traits::PrimInt> Default for StrToIntResult<T> {
  fn default() -> Self {
    Self { value: T::zero(), len: 0, error: 0 }
  }
}

#[inline]
pub fn strtoint<T: Into<CharToAscii> + Copy, I>(
  src: &[T],
  base: i32,
  ctype: &CtypeObject
) -> StrToIntResult<I>
where
  I: num_traits::PrimInt
    + CastFrom<i32>
    + CastFrom<usize>
    + CastFrom<u8>
    + num_traits::WrappingNeg {
  let min = I::min_value();
  let max = I::max_value();

  let mut result = StrToIntResult::<I>::default();
  let mut index = 0usize;
  let mut has_number = false;
  let mut has_overflow = false;

  while let Some(c) = get_char_with_index(src, index) &&
    (ctype.casemap.isspace)(c as u32)
  {
    index += 1;
  }

  let mut negative = false;
  if let Some(c) = get_char_with_index(src, index) &&
    c == '-'
  {
    index += 1;
    negative = true;
  } else if let Some(c) = get_char_with_index(src, index) &&
    c == '+'
  {
    index += 1;
  }

  let base = if base == 0 { infer_base(&src[index..], &ctype) } else { base };

  if base == 16 && is_hex_start(&src[index..], &ctype) {
    index += 2;
  } else if base == 8 && is_oct_start(&src[index..], &ctype) {
    index += 2;
  } else if base == 2 && is_bin_start(&src[index..], &ctype) {
    index += 2;
  }

  if base >= 2 && base <= 36 {
    let radix: I = I::cast_from(base);

    let (ceil, last): (I, I) = if negative && min != I::zero() {
      let ceil = (min / radix).wrapping_neg();
      let last = (min % radix).wrapping_neg();
      (ceil, last)
    } else {
      let ceil = max / radix;
      let last = max % radix;
      (ceil, last)
    };

    let mut value: I = I::zero();

    loop {
      let digit: u8;

      if let Some(c) = get_char_with_index(src, index) &&
        c >= '0' &&
        c <= '9'
      {
        let c: u32 = c as u32;
        digit = (c - '0' as u32) as u8;
      } else if let Some(c) = get_char_with_index(src, index) &&
        c >= 'A' &&
        c <= 'Z'
      {
        let c: u32 = c as u32;
        digit = (c - 'A' as u32 + 10) as u8;
      } else if let Some(c) = get_char_with_index(src, index) &&
        c >= 'a' &&
        c <= 'z'
      {
        let c: u32 = c as u32;
        digit = (c - 'a' as u32 + 10) as u8;
      } else {
        break;
      }

      if digit as i32 >= base {
        break;
      }

      index += 1;

      has_number = true;
      if value > ceil || (value == ceil && I::cast_from(digit) > last) {
        has_overflow = true;
      } else {
        let digit = I::cast_from(digit as i32);
        value =
          match value.checked_mul(&radix).and_then(|v| v.checked_add(&digit)) {
            | Some(v) => v,
            | None => {
              if negative {
                min
              } else {
                max
              }
            },
          };
      }

      if has_overflow {
        result.value = if negative { min } else { max };
      } else {
        result.value = if negative { value.wrapping_neg() } else { value };
      }
    }
  };

  if !has_number {
    result.error = errno::EINVAL;
    result.len = 0;
  } else {
    result.len = index;
  }
  if has_overflow {
    result.error = errno::ERANGE;
  }

  result
}
