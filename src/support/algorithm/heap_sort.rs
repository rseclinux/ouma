use super::sort_common::{ArrayForSort, Comparator};

#[inline]
pub fn heap_sort<'a, C: Comparator>(
  array: &'a mut ArrayForSort,
  comparator: &C
) {
  let mut end = array.number_of_elements();
  let mut start = end / 2;

  let left_child = |i: usize| -> usize { 2 * i + 1 };

  while end > 1 {
    if start > 0 {
      start -= 1;
    } else {
      end -= 1;
      array.swap(0, end);
    }

    let mut root = start;
    while left_child(root) < end {
      let mut child = left_child(root);

      if (child + 1 < end) &&
        comparator.compare_array(array, child, child + 1) < 0
      {
        child += 1;
      }

      if comparator.compare_array(array, root, child) >= 0 {
        break;
      }

      array.swap(root, child);
      root = child;
    }
  }
}
