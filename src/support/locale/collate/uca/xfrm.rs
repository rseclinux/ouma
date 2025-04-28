use {
  super::{Tailoring, ascii::fill, cea::generate_cea, normalize::make_nfd},
  crate::allocation::{vec, vec::Vec}
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct SortKey {
  pub tailoring: Tailoring,
  pub shifting: bool,
  pub tiebreak: bool,
  s_chars: Vec<u32>,
  s_cea: Vec<u32>,
  u8_cea: Vec<u8>
}

impl Default for SortKey {
  fn default() -> Self {
    Self::new(Tailoring::default(), true, true)
  }
}

impl SortKey {
  pub fn new(
    tailoring: Tailoring,
    shifting: bool,
    tiebreak: bool
  ) -> Self {
    Self {
      tailoring,
      shifting,
      tiebreak,
      s_chars: vec![0; 32],
      s_cea: vec![0; 32],
      u8_cea: vec![0; 32]
    }
  }

  pub fn get_sortkey_u8(
    &mut self,
    s: &[u8]
  ) -> &[u8] {
    let s: Vec<u32> = s.iter().map(|x| *x as u32).collect();
    let s: &[u32] = &s[..];
    let mut s_iter = s.into_iter();

    self.s_chars.clear();

    fill(&mut s_iter, &mut self.s_chars);
    make_nfd(&mut self.s_chars);
    generate_cea(
      &mut self.s_cea,
      &mut self.s_chars,
      self.shifting,
      self.tailoring
    );

    let cea_len: usize = self.s_cea.len();
    let cea_nulls: usize = self.s_cea.iter().filter(|&x| *x == 0).count();
    let trunc: usize = cea_len - cea_nulls;

    self.s_cea.truncate(trunc);

    self.u8_cea = self.s_cea.iter().map(|x| *x as u8).collect();
    &self.u8_cea[..]
  }

  pub fn get_sortkey_u32(
    &mut self,
    s: &[u32]
  ) -> &[u32] {
    let mut s_iter = s.into_iter();

    self.s_chars.clear();

    fill(&mut s_iter, &mut self.s_chars);
    make_nfd(&mut self.s_chars);
    generate_cea(
      &mut self.s_cea,
      &mut self.s_chars,
      self.shifting,
      self.tailoring
    );

    let cea_len: usize = self.s_cea.len();
    let cea_nulls: usize = self.s_cea.iter().filter(|&x| *x == 0).count();
    let trunc: usize = cea_len - cea_nulls;

    self.s_cea.truncate(trunc);

    &self.s_cea[..]
  }
}
