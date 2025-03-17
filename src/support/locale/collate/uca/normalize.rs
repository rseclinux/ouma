use {
  super::consts::{DECOMP, FCD, JAMO_LV},
  crate::allocation::{vec, vec::Vec},
  unicode_canonical_combining_class::get_canonical_combining_class_u32
};

const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = 588;

pub fn make_nfd(input: &mut Vec<u32>) {
  if fcd(input) {
    return;
  }

  decompose(input);
  reorder(input);
}

fn fcd(input: &[u32]) -> bool {
  let mut curr_lead_cc: u8;
  let mut curr_trail_cc: u8;

  let mut prev_trail_cc: u8 = 0;

  for &c in input {
    if c < 0x00C0 {
      prev_trail_cc = 0;
      continue;
    }

    if c == 0x0F81 || (0xAC00..=0xD7A3).contains(&c) {
      return false;
    }

    if let Some(vals) = FCD.get(&c) {
      [curr_lead_cc, curr_trail_cc] = vals.to_be_bytes();
    } else {
      curr_lead_cc = get_canonical_combining_class_u32(c) as u8;
      curr_trail_cc = curr_lead_cc;
    }

    if curr_lead_cc != 0 && curr_lead_cc < prev_trail_cc {
      return false;
    }

    prev_trail_cc = curr_trail_cc;
  }

  true
}

fn decompose(input: &mut Vec<u32>) {
  let mut i: usize = 0;

  while i < input.len() {
    let code_point = input[i];

    if code_point < 0x00C0 {
      i += 1;
      continue;
    }

    if (0xAC00..=0xD7A3).contains(&code_point) {
      let rep = decompose_jamo(code_point);
      let n = rep.len();

      input.splice(i..=i, rep);

      i += n;
      continue;
    }

    if let Some(rep) = DECOMP.get(&code_point) {
      input.splice(i..=i, rep.clone());

      i += rep.len();
      continue;
    }

    i += 1;
  }
}

fn decompose_jamo(s: u32) -> Vec<u32> {
  let s_index = s - S_BASE;

  let lv = JAMO_LV.contains(&s);

  let l_index = s_index / N_COUNT;
  let v_index = (s_index % N_COUNT) / T_COUNT;

  if lv {
    let l_part = L_BASE + l_index;
    let v_part = V_BASE + v_index;

    vec![l_part, v_part]
  } else {
    let t_index = s_index % T_COUNT;

    let l_part = L_BASE + l_index;
    let v_part = V_BASE + v_index;
    let t_part = T_BASE + t_index;

    vec![l_part, v_part, t_part]
  }
}

fn reorder(input: &mut [u32]) {
  let mut n = input.len();

  while n > 1 {
    let mut new_n = 0;
    let mut i = 1;

    while i < n {
      let ccc_b = get_canonical_combining_class_u32(input[i]) as u8;
      if ccc_b == 0 {
        i += 2;
        continue;
      }

      let ccc_a = get_canonical_combining_class_u32(input[i - 1]) as u8;
      if ccc_a == 0 || ccc_a <= ccc_b {
        i += 1;
        continue;
      }

      input.swap(i - 1, i);

      new_n = i;
      i += 1;
    }

    n = new_n;
  }
}
