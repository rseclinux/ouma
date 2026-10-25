use super::{
  heap_sort::heap_sort,
  sort_common::{ArrayForSort, Comparator}
};

#[inline]
fn median_of_three<'a, C: Comparator>(
  array: &'a mut ArrayForSort,
  comparator: &C,
  a: usize,
  b: usize,
  c: usize
) -> usize {
  return if comparator.compare_array(array, a, b) < 0 {
    if comparator.compare_array(array, b, c) < 0 {
      b
    } else {
      if comparator.compare_array(array, a, c) < 0 { c } else { a }
    }
  } else {
    if comparator.compare_array(array, b, c) > 0 {
      b
    } else {
      if comparator.compare_array(array, a, c) > 0 { c } else { a }
    }
  };
}

#[inline]
fn get_pivot<'a, C: Comparator>(
  array: &'a mut ArrayForSort,
  comparator: &C
) -> usize {
  let n = array.number_of_elements();
  if n == 0 {
    return 0;
  }

  let mut l = 0;
  let mut m = n / 2;
  let mut h = n - 1;

  if n > 40 {
    let len = n / 8;
    l = median_of_three(array, comparator, l, l + len, l + 2 * len);
    m = median_of_three(array, comparator, m - len, m, m + len);
    h = median_of_three(array, comparator, h - 2 * len, h - len, h);
  }

  median_of_three(array, comparator, l, m, h)
}

#[inline]
fn partition<'a, C: Comparator>(
  array: &'a mut ArrayForSort,
  pivot_index: usize,
  compare: &C
) -> usize {
  if pivot_index != 0 {
    array.swap(0, pivot_index);
  }

  let array_len = array.number_of_elements();
  let Some(mut without_pivot) = array.make_array(1, array_len - 1) else {
    return 0;
  };

  let pivot = array.get(0).unwrap_or(&[]);

  let mut left = 0usize;
  let mut right = without_pivot.number_of_elements();

  loop {
    while left < right {
      let i = without_pivot.get(left).unwrap_or(&[]);
      if compare.compare(i.as_ptr(), pivot.as_ptr()) >= 0 {
        break;
      }
      left += 1;
    }

    while left < right {
      let k = without_pivot.get(right - 1).unwrap_or(&[]);
      if compare.compare(k.as_ptr(), pivot.as_ptr()) <= 0 {
        break;
      }
      right -= 1;
    }

    if left >= right {
      break;
    }

    without_pivot.swap(left, right - 1);
    left += 1;
    right -= 1;
  }

  array.swap(0, left);

  left
}

#[inline]
pub fn quick_sort<'a, C: Comparator>(
  array: &'a mut ArrayForSort,
  comparator: &C
) {
  if array.number_of_elements() < 8 {
    heap_sort(array, comparator);
    return;
  }

  loop {
    let array_size = array.number_of_elements();
    if array_size <= 1 {
      return;
    }

    let pivot_idx = get_pivot(array, comparator);
    let split = partition(array, pivot_idx, comparator);
    if array_size == 2 {
      return;
    }

    let right_start = split + 1;
    let right_end = array_size - right_start;

    if split < right_end {
      let Some(mut left) = array.make_array(0, split) else {
        return;
      };
      quick_sort(&mut left, comparator);
      array.reset_bounds(right_start, right_end);
    } else {
      let Some(mut right) = array.make_array(right_start, right_end) else {
        return;
      };
      quick_sort(&mut right, comparator);
      array.reset_bounds(0, split);
    }
  }
}
