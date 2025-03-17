use {
  super::{
    consts::{MULT_CLDR_DATA, SING_CLDR_DATA},
    types::{MultisTable, SinglesTable}
  },
  once_cell::sync::Lazy
};

const SING_AR_DATA: &[u8; 20_580] =
  include_bytes!("bincode/tailoring/arabic_script_sing");
pub static SING_AR: Lazy<SinglesTable> = Lazy::new(|| {
  let (mut sing, _): (SinglesTable, _) = bincode::serde::decode_from_slice(
    SING_CLDR_DATA,
    bincode::config::legacy()
  )
  .unwrap();
  let (extension, _): (SinglesTable, _) =
    bincode::serde::decode_from_slice(SING_AR_DATA, bincode::config::legacy())
      .unwrap();

  sing.extend(extension);
  sing
});

const MULT_AR_DATA: &[u8; 148] =
  include_bytes!("bincode/tailoring/arabic_script_multi");
pub static MULT_AR: Lazy<MultisTable> = Lazy::new(|| {
  let (mut mult, _): (MultisTable, _) = bincode::serde::decode_from_slice(
    MULT_CLDR_DATA,
    bincode::config::legacy()
  )
  .unwrap();
  let (extension, _): (MultisTable, _) =
    bincode::serde::decode_from_slice(MULT_AR_DATA, bincode::config::legacy())
      .unwrap();

  mult.extend(extension);
  mult
});

const SING_AR_I_DATA: &[u8; 14_652] =
  include_bytes!("bincode/tailoring/arabic_interleaved_sing");
pub static SING_AR_I: Lazy<SinglesTable> = Lazy::new(|| {
  let (mut sing, _): (SinglesTable, _) = bincode::serde::decode_from_slice(
    SING_CLDR_DATA,
    bincode::config::legacy()
  )
  .unwrap();
  let (extension, _): (SinglesTable, _) = bincode::serde::decode_from_slice(
    SING_AR_I_DATA,
    bincode::config::legacy()
  )
  .unwrap();

  sing.extend(extension);
  sing
});

const MULT_AR_I_DATA: &[u8; 92] =
  include_bytes!("bincode/tailoring/arabic_interleaved_multi");
pub static MULT_AR_I: Lazy<MultisTable> = Lazy::new(|| {
  let (mut mult, _): (MultisTable, _) = bincode::serde::decode_from_slice(
    MULT_CLDR_DATA,
    bincode::config::legacy()
  )
  .unwrap();
  let (extension, _): (MultisTable, _) = bincode::serde::decode_from_slice(
    MULT_AR_I_DATA,
    bincode::config::legacy()
  )
  .unwrap();

  mult.extend(extension);
  mult
});
