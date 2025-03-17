use {
  super::{
    Tailoring,
    ascii::fill_and_check,
    cea::generate_cea,
    first_weight::try_initial,
    normalize::make_nfd,
    prefix::trim_prefix,
    sort_key::compare_incremental
  },
  crate::allocation::{vec, vec::Vec},
  core::cmp::Ordering
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Collator {
  pub tailoring: Tailoring,
  pub shifting: bool,
  pub tiebreak: bool,
  a_chars: Vec<u32>,
  b_chars: Vec<u32>,
  a_cea: Vec<u32>,
  b_cea: Vec<u32>
}

impl Default for Collator {
  fn default() -> Self {
    Self::new(Tailoring::default(), true, true)
  }
}

impl Collator {
  pub fn new(
    tailoring: Tailoring,
    shifting: bool,
    tiebreak: bool
  ) -> Self {
    Self {
      tailoring,
      shifting,
      tiebreak,
      a_chars: vec![0; 32],
      b_chars: vec![0; 32],
      a_cea: vec![0; 32],
      b_cea: vec![0; 32]
    }
  }

  pub fn collate_u8(
    &mut self,
    a: &[u8],
    b: &[u8]
  ) -> Ordering {
    let a: Vec<u32> = a.iter().map(|x| *x as u32).collect();
    let a: &[u32] = &a[..];
    let b: Vec<u32> = b.iter().map(|x| *x as u32).collect();
    let b: &[u32] = &b[..];

    self.collate_u32(a, b)
  }

  pub fn collate_u32(
    &mut self,
    a: &[u32],
    b: &[u32]
  ) -> Ordering {
    if a == b {
      return Ordering::Equal;
    }

    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    self.a_chars.clear();
    self.b_chars.clear();

    if let Some(o) = fill_and_check(
      &mut a_iter,
      &mut b_iter,
      &mut self.a_chars,
      &mut self.b_chars
    ) {
      return o;
    }

    make_nfd(&mut self.a_chars);
    make_nfd(&mut self.b_chars);

    if self.a_chars == self.b_chars {
      if self.tiebreak {
        return a.cmp(b);
      }

      return Ordering::Equal;
    }

    let shifting = self.shifting;
    trim_prefix(&mut self.a_chars, &mut self.b_chars, shifting);

    if self.a_chars.is_empty() || self.b_chars.is_empty() {
      return self.a_chars.cmp(&self.b_chars);
    }

    if let Some(o) = try_initial(self, &self.a_chars, &self.b_chars) {
      return o;
    }

    let tailoring = self.tailoring;
    generate_cea(&mut self.a_cea, &mut self.a_chars, shifting, tailoring);
    generate_cea(&mut self.b_cea, &mut self.b_chars, shifting, tailoring);

    let comparison = compare_incremental(&self.a_cea, &self.b_cea, shifting);

    if comparison == Ordering::Equal && self.tiebreak {
      return a.cmp(b);
    }

    comparison
  }
}
