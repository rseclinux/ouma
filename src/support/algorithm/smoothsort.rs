//
// Based off C implementation:
// https://en.wikibooks.org/wiki/Algorithm_Implementation/Sorting/Smoothsort
//

use super::sort_common::{ArrayForSort, Comparator};

#[inline]
fn up(
  a: &mut usize,
  b: &mut usize
) {
  let t = *a;
  *a = a.wrapping_add(*b).wrapping_add(1);
  *b = t;
}

#[inline]
fn down(
  a: &mut usize,
  b: &mut usize
) {
  let t = *b;
  *b = a.wrapping_sub(*b).wrapping_sub(1);
  *a = t;
}

pub struct Smoothsort<'a, 'b, C: Comparator> {
  array: &'b mut ArrayForSort<'a>,
  cmp: &'b C,
  q: usize,
  r: usize,
  p: usize,
  b: usize,
  c: usize,
  r1: usize,
  b1: usize,
  c1: usize
}

impl<'a, 'b, C: Comparator> Smoothsort<'a, 'b, C> {
  #[inline]
  pub const fn new(
    array: &'b mut ArrayForSort<'a>,
    cmp: &'b C
  ) -> Self {
    Self { array, cmp, q: 1, r: 0, p: 1, b: 1, c: 1, r1: 0, b1: 0, c1: 0 }
  }

  #[inline]
  fn sift(&mut self) {
    while self.b1 > 2 {
      let mut r2 = self.r1 + self.c1 - self.b1;
      if self.cmp.compare_array(self.array, self.r1 - 1, r2) > 0 {
        r2 = self.r1 - 1;
        down(&mut self.b1, &mut self.c1);
      }
      if self.cmp.compare_array(self.array, r2, self.r1) <= 0 {
        self.b1 = 1;
      } else {
        self.array.swap(self.r1, r2);
        self.r1 = r2;
        down(&mut self.b1, &mut self.c1);
      }
    }
  }

  #[inline]
  fn trinkle(&mut self) {
    let mut p1 = self.p;
    self.b1 = self.b;
    self.c1 = self.c;

    while p1 > 0 {
      while (p1 & 1) == 0 {
        p1 >>= 1;
        up(&mut self.b1, &mut self.c1);
      }
      let r3 = self.r1.wrapping_sub(self.b1);

      if p1 == 1 || self.cmp.compare_array(self.array, r3, self.r1) <= 0 {
        p1 = 0;
      } else {
        p1 -= 1;
        if self.b1 == 1 {
          self.array.swap(self.r1, r3);
          self.r1 = r3;
        } else if self.b1 >= 3 {
          let mut r2 = self.r1 + self.c1 - self.b1;
          if self.cmp.compare_array(self.array, self.r1 - 1, r2) > 0 {
            r2 = self.r1 - 1;
            down(&mut self.b1, &mut self.c1);
            p1 <<= 1;
          }
          if self.cmp.compare_array(self.array, r2, r3) <= 0 {
            self.array.swap(self.r1, r3);
            self.r1 = r3;
          } else {
            self.array.swap(self.r1, r2);
            self.r1 = r2;
            down(&mut self.b1, &mut self.c1);
            p1 = 0;
          }
        }
      }
    }
    self.sift();
  }

  #[inline]
  fn semitrinkle(&mut self) {
    self.r1 = self.r - self.c;
    if self.cmp.compare_array(self.array, self.r1, self.r) > 0 {
      self.array.swap(self.r, self.r1);
      self.trinkle();
    }
  }

  #[inline]
  pub fn smoothsort(&mut self) {
    let n = self.array.number_of_elements();

    while self.q < n {
      self.r1 = self.r;

      if (self.p & 7) == 3 {
        self.b1 = self.b;
        self.c1 = self.c;
        self.sift();

        self.p = (self.p + 1) >> 2;

        up(&mut self.b, &mut self.c);
        up(&mut self.b, &mut self.c);
      } else if (self.p & 3) == 1 {
        if self.q + self.c < n {
          self.b1 = self.b;
          self.c1 = self.c;

          self.sift();
        } else {
          self.trinkle();
        }

        down(&mut self.b, &mut self.c);
        self.p <<= 1;

        while self.b > 1 {
          down(&mut self.b, &mut self.c);
          self.p <<= 1;
        }

        self.p += 1;
      }

      self.r += 1;
      self.q += 1;
    }

    self.r1 = self.r;
    self.trinkle();

    while self.q > 1 {
      if self.b == 1 {
        self.r -= 1;
        self.p -= 1;

        while (self.p & 1) == 0 {
          self.p >>= 1;
          up(&mut self.b, &mut self.c);
        }
      } else if self.b > 2 {
        self.p -= 1;
        self.r = self.r + self.c - self.b;

        if self.p > 0 {
          self.semitrinkle();
        }

        down(&mut self.b, &mut self.c);

        self.p = (self.p << 1) + 1;
        self.r += self.c;
        self.semitrinkle();

        down(&mut self.b, &mut self.c);

        self.p = (self.p << 1) + 1;
      }

      self.q -= 1;
    }
  }
}
