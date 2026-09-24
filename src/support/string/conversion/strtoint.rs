use {
  super::StrToError,
  crate::support::{
    locale::ctype::CtypeObject,
    traits::char::{CharToAscii, get_ascii_char_with_index}
  }
};

#[derive(Clone, Copy, Debug)]
pub struct StrToIntResult<T: num_traits::PrimInt> {
  pub value: T,
  pub len: usize,
  pub error: Option<StrToError>
}

impl<T: num_traits::PrimInt> Default for StrToIntResult<T> {
  #[inline]
  fn default() -> Self {
    Self { value: T::zero(), len: 0, error: None }
  }
}

#[inline]
fn validate_base_prefix<T: Into<CharToAscii> + Copy>(
  src: &[T],
  index: &mut usize,
  spec: char,
  base: u32
) -> bool {
  if get_ascii_char_with_index(src, *index) == Some('0') &&
    get_ascii_char_with_index(src, *index + 1)
      .map(|c| c.to_ascii_lowercase() == spec)
      .unwrap_or(false) &&
    get_ascii_char_with_index(src, *index + 2)
      .and_then(|c| c.to_digit(base))
      .is_some()
  {
    *index += 2;
    true
  } else {
    false
  }
}

#[inline]
pub fn strtoint<T: Into<CharToAscii> + Copy, I>(
  src: &[T],
  base: i32,
  ctype: &CtypeObject
) -> StrToIntResult<I>
where
  I: num_traits::PrimInt + num_traits::WrappingNeg + core::ops::DivAssign {
  let min_value = I::min_value();
  let max_value = I::max_value();
  let is_unsigned = min_value == I::zero();

  let mut result = StrToIntResult::<I>::default();
  let mut index = 0usize;
  let mut base = base;
  let mut has_number = false;
  let mut has_overflow = false;

  while let Some(c) = get_ascii_char_with_index(src, index) &&
    (ctype.casemap.isspace)(c as u32)
  {
    index += 1;
  }

  let mut negative = false;
  if let Some(c) = get_ascii_char_with_index(src, index) &&
    c == '-'
  {
    index += 1;
    negative = true;
  } else if let Some(c) = get_ascii_char_with_index(src, index) &&
    c == '+'
  {
    index += 1;
  }

  if base < 0 || base == 1 || base > 36 {
    result.error = Some(StrToError::InvalidNumber);
    return result;
  }

  if (base == 0 || base == 16) && validate_base_prefix(src, &mut index, 'x', 16)
  {
    base = 16;
  } else if (base == 0 || base == 8) &&
    validate_base_prefix(src, &mut index, 'o', 8)
  {
    base = 8;
  } else if (base == 0 || base == 2) &&
    validate_base_prefix(src, &mut index, 'b', 2)
  {
    base = 2;
  }

  if base == 0 {
    if get_ascii_char_with_index(src, index) == Some('0') {
      base = 8;
    } else {
      base = 10;
    }
  }

  let radix = match I::from(base) {
    | Some(radix) => radix,
    | None => {
      result.error = Some(StrToError::InvalidNumber);
      return result;
    }
  };

  let mut cutoff: I;
  let cutlim: I;

  if is_unsigned {
    cutoff = max_value / radix;
    cutlim = max_value % radix;
  } else {
    cutoff = if negative {
      let prod = min_value.saturating_add(max_value);
      let prod = prod.wrapping_neg();
      prod.saturating_add(max_value)
    } else {
      max_value
    };
    cutlim = cutoff % radix;
    cutoff /= radix;
  }

  while let Some(ch) = get_ascii_char_with_index(src, index) {
    let digit;

    if ch >= '0' && ch <= '9' {
      let c: u32 = ch as u32;
      digit = I::from(c - ('0' as u32)).unwrap_or(I::zero());
    } else if ch >= 'A' && ch <= 'Z' {
      let c: u32 = ch as u32;
      digit = I::from((c - ('A' as u32)) + 10).unwrap_or(I::zero());
    } else if ch >= 'a' && ch <= 'z' {
      let c: u32 = ch as u32;
      digit = I::from((c - ('a' as u32)) + 10).unwrap_or(I::zero());
    } else {
      break;
    }

    if digit >= radix {
      break;
    }

    has_number = true;

    if has_overflow ||
      result.value > cutoff ||
      (result.value == cutoff && digit > cutlim)
    {
      has_overflow = true;
    } else {
      result.value = result.value * radix;
      result.value = result.value + digit;
    }

    index += 1;
  }

  if !has_number {
    result.error = Some(StrToError::InvalidNumber);
    result.len = 0;
  } else {
    result.len = index;

    if has_overflow {
      if negative && min_value != I::zero() {
        result.value = min_value;
      } else {
        result.value = max_value;
      }
      result.error = Some(StrToError::Range);
    } else if negative {
      result.value = result.value.wrapping_neg();
    }
  }

  result
}
