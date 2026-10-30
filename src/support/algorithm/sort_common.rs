use core::slice;

pub trait Comparator {
  fn compare(
    &self,
    a: *const u8,
    b: *const u8
  ) -> i32;

  #[inline]
  fn compare_array(
    &self,
    array: &ArrayForSort,
    j: usize,
    k: usize
  ) -> i32 {
    match (array.get(j), array.get(k)) {
      | (Some(a), Some(b)) => self.compare(a.as_ptr(), b.as_ptr()),
      | _ => 0
    }
  }
}

#[derive(Debug)]
pub struct ArrayForSort<'a> {
  array: &'a mut [u8],
  element_size: usize
}

impl<'a> ArrayForSort<'a> {
  #[inline]
  pub const fn from_raw(
    ptr: *mut u8,
    count: usize,
    element_size: usize
  ) -> Option<Self> {
    if count == 0 || element_size == 0 {
      return None;
    }
    let Some(total_size) = count.checked_mul(element_size) else {
      return None;
    };
    let array = unsafe { slice::from_raw_parts_mut(ptr, total_size) };
    Some(Self { array, element_size })
  }

  #[inline]
  pub fn exact_length(&self) -> usize {
    self.array.len()
  }

  #[inline]
  pub fn element_size(&self) -> usize {
    self.element_size
  }

  #[inline]
  pub fn number_of_elements(&self) -> usize {
    self.array.len() / self.element_size
  }

  #[inline]
  pub fn get(
    &self,
    index: usize
  ) -> Option<&[u8]> {
    let start = index.checked_mul(self.element_size)?;
    let end = start.checked_add(self.element_size)?;
    self.array.get(start..end)
  }

  #[inline]
  pub fn get_mut(
    &mut self,
    index: usize
  ) -> Option<&mut [u8]> {
    let start = index.checked_mul(self.element_size)?;
    let end = start.checked_add(self.element_size)?;
    self.array.get_mut(start..end)
  }

  #[inline]
  pub fn make_array(
    &mut self,
    index: usize,
    count: usize
  ) -> Option<Self> {
    let start = index.checked_mul(self.element_size())?;
    let total_count = count.checked_mul(self.element_size())?;
    let end = start.checked_add(total_count)?;
    let slice = self.array.get_mut(start..end)?;
    Self::from_raw(slice.as_mut_ptr(), count, self.element_size())
  }

  #[inline]
  pub fn swap(
    &mut self,
    i: usize,
    j: usize
  ) {
    let elemsz = self.element_size();

    if i == j {
      return;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    if hi >= self.number_of_elements() {
      return;
    }

    let (head, tail) = self.array.split_at_mut(hi * elemsz);
    let ei = &mut head[lo * elemsz..(lo + 1) * elemsz];
    let ej = &mut tail[..elemsz];

    ei.swap_with_slice(ej);
  }
}
