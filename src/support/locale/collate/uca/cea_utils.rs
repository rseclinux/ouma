use {
  super::{
    Locale,
    Tailoring,
    consts::{INCLUDED_UNASSIGNED, MULT, MULT_CLDR, SING, SING_CLDR},
    tailor::{MULT_AR, MULT_AR_I, SING_AR, SING_AR_I},
    types::{MultisTable, SinglesTable},
    weights::{pack_weights, shift_weights}
  },
  crate::allocation::vec::Vec,
  once_cell::sync::Lazy,
  unicode_canonical_combining_class::get_canonical_combining_class_u32
};

pub fn ccc_sequence_ok(test_range: &[u32]) -> bool {
  let mut max_ccc = 0;

  for elem in test_range {
    let ccc = get_canonical_combining_class_u32(*elem) as u8;

    if ccc == 0 || ccc <= max_ccc {
      return false;
    }

    max_ccc = ccc;
  }

  true
}

pub fn fill_weights(
  cea: &mut [u32],
  row: &Vec<u32>,
  i: &mut usize,
  shifting: bool,
  last_variable: &mut bool
) {
  if shifting {
    for weights in row {
      cea[*i] = shift_weights(*weights, last_variable);
      *i += 1;
    }
  } else {
    for weights in row {
      cea[*i] = *weights;
      *i += 1;
    }
  }
}

pub fn get_tables(
  tailoring: Tailoring
) -> (&'static Lazy<SinglesTable>, &'static Lazy<MultisTable>) {
  match tailoring {
    | Tailoring::Cldr(Locale::ArabicScript) => (&SING_AR, &MULT_AR),
    | Tailoring::Cldr(Locale::ArabicInterleaved) => (&SING_AR_I, &MULT_AR_I),
    | Tailoring::Cldr(Locale::Root) => (&SING_CLDR, &MULT_CLDR),
    | Tailoring::Ducet => (&SING, &MULT)
  }
}

pub fn grow_vec(
  cea: &mut Vec<u32>,
  i: usize
) {
  let l = cea.len();

  if l - i < 10 {
    cea.resize(l * 2, 0);
  }
}

pub fn handle_implicit_weights(
  cea: &mut [u32],
  cp: u32,
  i: &mut usize
) {
  cea[*i] = implicit_a(cp);
  *i += 1;

  cea[*i] = implicit_b(cp);
  *i += 1;
}

pub fn handle_low_weights(
  cea: &mut [u32],
  weights: u32,
  i: &mut usize,
  shifting: bool,
  last_variable: &mut bool
) {
  if shifting {
    cea[*i] = shift_weights(weights, last_variable);
  } else {
    cea[*i] = weights;
  }

  *i += 1;
}

pub fn implicit_a(cp: u32) -> u32 {
  let aaaa = if INCLUDED_UNASSIGNED.contains(&cp) {
    0xFBC0 + (cp >> 15)
  } else {
    match cp {
      | 0x3400..=0x4DBF |
      0x20000..=0x2A6DF |
      0x2A700..=0x2EE5D |
      0x30000..=0x323AF => 0xFB80 + (cp >> 15),
      | 0x4E00..=0x9FFF | 0xF900..=0xFAFF => 0xFB40 + (cp >> 15),
      | 0x17000..=0x18AFF | 0x18D00..=0x18D8F => 0xFB00,
      | 0x18B00..=0x18CFF => 0xFB02,
      | 0x1B170..=0x1B2FF => 0xFB01,
      | _ => 0xFBC0 + (cp >> 15)
    }
  };

  #[allow(clippy::cast_possible_truncation)]
  pack_weights(false, aaaa as u16, 32, 2)
}

pub fn implicit_b(cp: u32) -> u32 {
  let mut bbbb = if INCLUDED_UNASSIGNED.contains(&cp) {
    cp & 0x7FFF
  } else {
    match cp {
      | 0x17000..=0x18AFF | 0x18D00..=0x18D8F => cp - 0x17000,
      | 0x18B00..=0x18CFF => cp - 0x18B00,
      | 0x1B170..=0x1B2FF => cp - 0x1B170,
      | _ => cp & 0x7FFF
    }
  };

  bbbb |= 0x8000;

  #[allow(clippy::cast_possible_truncation)]
  pack_weights(false, bbbb as u16, 0, 0)
}

pub fn remove_pulled(
  char_vals: &mut Vec<u32>,
  i: usize,
  input_length: &mut usize,
  try_two: bool
) {
  char_vals.remove(i);
  *input_length -= 1;

  if try_two {
    char_vals.remove(i - 1);
    *input_length -= 1;
  }
}
