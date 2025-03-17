use {crate::allocation::vec::Vec, hashbrown::HashMap};

pub type SinglesTable = HashMap<u32, Vec<u32>>;
pub type MultisTable = HashMap<Vec<u32>, Vec<u32>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Tailoring {
  Cldr(Locale),
  Ducet
}

impl Default for Tailoring {
  fn default() -> Self {
    Self::Cldr(Locale::default())
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum Locale {
  ArabicScript,
  ArabicInterleaved,
  #[default]
  Root
}
