use {crate::size_t, core::cmp};

#[inline(always)]
fn greater_than<T: cmp::PartialOrd>(
  a: T,
  b: T
) -> bool {
  a > b
}

#[inline(always)]
fn less_than<T: cmp::PartialOrd>(
  a: T,
  b: T
) -> bool {
  a < b
}

#[inline(always)]
fn internal_compute_maximal_suffix<T, F>(
  needle: &[T],
  compare: F
) -> (usize, usize)
where
  T: cmp::PartialEq + cmp::PartialOrd + Copy + Clone,
  F: Fn(T, T) -> bool {
  let mut index: usize = 0;
  let mut period: usize = 1;
  let mut j: usize = 1;
  let mut k: usize = 0;

  while j + k < needle.len() {
    if compare(needle[j + k], needle[index + k]) {
      j += k + 1;
      k = 0;
      period = j - index;
    } else if needle[j + k] == needle[index + k] {
      if k == period {
        j += period;
        k = 0;
      } else {
        k += 1;
      }
    } else {
      index = j;
      j = j.wrapping_add(1);
      k = 0;
      period = 1;
    }
  }
  (index, period)
}

#[inline(always)]
fn compute_maximal_suffix<T>(needle: &[T]) -> (usize, usize)
where
  T: cmp::PartialEq + cmp::PartialOrd + Copy + Clone {
  let (i1, p1) = internal_compute_maximal_suffix(needle, less_than);
  let (i2, p2) = internal_compute_maximal_suffix(needle, greater_than);
  if i1 >= i2 { (i1, p1) } else { (i2, p2) }
}

#[inline(always)]
pub fn twoway<'a, T, F>(
  haystack: &'a [T],
  needle: &'a [T],
  compare: F
) -> Option<&'a [T]>
where
  T: cmp::PartialEq + cmp::PartialOrd + Copy + Clone,
  F: Fn(*const T, *const T, size_t) -> bool {
  let (index, period) = compute_maximal_suffix(needle);
  let mut haystack = haystack;
  let mut skip: usize = 0;

  loop {
    let mut i = if index >= skip { index } else { skip };

    while i < needle.len() && Some(needle.get(i)) == Some(haystack.get(i)) {
      i += 1
    }

    if i < needle.len() {
      if skip <= period || i - index >= skip - period {
        if let Some(x) = haystack.get(i - index + 1..) {
          haystack = x;
        } else {
          return None;
        }
      } else {
        if let Some(x) = haystack.get(skip - period + 1..) {
          haystack = x;
        } else {
          return None;
        }
      }
      skip = 0;
    } else {
      let mut j = index;

      while j > skip && needle[j - 1] == haystack[j - 1] {
        j -= 1;
      }

      if j <= skip {
        return Some(haystack);
      }

      unsafe {
        if compare(
          needle.as_ptr(),
          needle.as_ptr().offset(period as isize),
          index
        ) {
          haystack = &haystack[period..];
          skip = needle.len() - period;
        } else {
          if index > needle.len() - index {
            haystack = &haystack[index + 1..];
          } else {
            haystack = &haystack[needle.len() + index + 1..];
          }
        }
      }
    }
  }
}
