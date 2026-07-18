use {
  super::{IsSigned, b36_char_to_int},
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
    + IsSigned
    + CastFrom<i32>
    + CastFrom<usize>
    + num_traits::WrappingNeg,
  usize: CastFrom<I>,
  u8: CastFrom<I> {
  let allow_neg = I::IS_SIGNED;
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

  let negative = if let Some(c) = get_char_with_index(src, index) &&
    c == '-' &&
    allow_neg
  {
    index += 1;
    true
  } else {
    if let Some(c) = get_char_with_index(src, index) &&
      c == '+'
    {
      index += 1;
    }
    false
  };

  let base = if base == 0 { infer_base(src, &ctype) } else { base };

  if base == 16 && is_hex_start(src, &ctype) {
    index += 2;
  } else if base == 8 && is_oct_start(src, &ctype) {
    index += 2;
  } else if base == 2 && is_bin_start(src, &ctype) {
    index += 2;
  }

  if base >= 2 && base <= 36 {
    let radix: I = I::cast_from(base);

    let (ceil, last): (usize, u8) = if negative {
      let ceil: usize = usize::cast_from((min / radix).wrapping_neg());
      let last: u8 = u8::cast_from((min % radix).wrapping_neg());
      (ceil, last)
    } else {
      let ceil: usize = usize::cast_from(max / radix);
      let last: u8 = u8::cast_from(max % radix);
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
      if value > I::cast_from(ceil) ||
        value == I::cast_from(ceil) && digit > last
      {
        has_overflow = true;
      } else {
        value = value * radix + I::cast_from(digit as i32);
      }

      if has_overflow {
        result.value = if !allow_neg || !negative { max } else { min };
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
