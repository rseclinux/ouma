use {
  super::{
    Tailoring,
    cea_utils::{
      ccc_sequence_ok,
      fill_weights,
      get_tables,
      grow_vec,
      handle_implicit_weights,
      handle_low_weights,
      remove_pulled
    },
    consts::{LOW, LOW_CLDR, NEED_THREE, NEED_TWO}
  },
  crate::allocation::{vec, vec::Vec},
  unicode_canonical_combining_class::get_canonical_combining_class_u32
};

pub fn generate_cea(
  cea: &mut Vec<u32>,
  char_vals: &mut Vec<u32>,
  shifting: bool,
  tailoring: Tailoring
) {
  let mut input_length = char_vals.len();

  let cldr = tailoring != Tailoring::Ducet;
  let low = if cldr { &LOW_CLDR } else { &LOW };
  let (singles, multis) = get_tables(tailoring);

  let mut left: usize = 0;
  let mut cea_idx: usize = 0;
  let mut last_variable = false;

  'outer: while left < input_length {
    let left_val = char_vals[left];

    grow_vec(cea, cea_idx);

    if left_val < 0x00B7 && left_val != 0x006C && left_val != 0x004C {
      let weights = low[&left_val];
      handle_low_weights(
        cea,
        weights,
        &mut cea_idx,
        shifting,
        &mut last_variable
      );
      left += 1;
      continue;
    }

    let lookahead: usize = match left_val {
      | x if NEED_THREE.contains(&x) => 3,
      | x if NEED_TWO.contains(&x) => 2,
      | _ => 1
    };

    let check_multi = lookahead > 1 && (input_length - left > 1);

    if !check_multi {
      if let Some(row) = singles.get(&left_val) {
        fill_weights(cea, row, &mut cea_idx, shifting, &mut last_variable);
        left += 1;
        continue;
      }

      handle_implicit_weights(cea, left_val, &mut cea_idx);

      left += 1;
      continue;
    }

    let mut right = input_length.min(left + lookahead);

    while right > left {
      if right - left == 1 {
        let row = &singles[&left_val];

        let mut max_right = match input_length - right {
          | 3.. => right + 2,
          | 2 => right + 1,
          | _ => right
        };

        let mut try_two = (max_right - right == 2) && cldr;

        while max_right > right {
          let test_range = &char_vals[right..=max_right];

          if !ccc_sequence_ok(test_range) {
            try_two = false;
            max_right -= 1;
            continue;
          }

          let new_subset = if try_two {
            vec![left_val, char_vals[max_right - 1], char_vals[max_right]]
          } else {
            vec![left_val, char_vals[max_right]]
          };

          if let Some(new_row) = multis.get(&new_subset) {
            fill_weights(
              cea,
              new_row,
              &mut cea_idx,
              shifting,
              &mut last_variable
            );
            remove_pulled(char_vals, max_right, &mut input_length, try_two);

            left += 1;
            continue 'outer;
          }

          if try_two {
            try_two = false;
          } else {
            max_right -= 1;
          }
        }

        fill_weights(cea, row, &mut cea_idx, shifting, &mut last_variable);
        left += 1;
        continue 'outer;
      }

      let subset = &char_vals[left..right];

      if let Some(row) = multis.get(subset) {
        let try_discont = subset.len() == 2 && (right + 1 < input_length);

        if try_discont {
          let ccc_a = get_canonical_combining_class_u32(char_vals[right]) as u8;
          let ccc_b =
            get_canonical_combining_class_u32(char_vals[right + 1]) as u8;

          if ccc_a > 0 && ccc_b > ccc_a {
            let new_subset = vec![subset[0], subset[1], char_vals[right + 1]];

            if let Some(new_row) = multis.get(&new_subset) {
              fill_weights(
                cea,
                new_row,
                &mut cea_idx,
                shifting,
                &mut last_variable
              );
              remove_pulled(char_vals, right + 1, &mut input_length, false);

              left += right - left;
              continue 'outer;
            }
          }
        }

        fill_weights(cea, row, &mut cea_idx, shifting, &mut last_variable);
        left += right - left; // NB, we increment here by a variable amount
        continue 'outer;
      }
      right -= 1;
    }

    unreachable!();
  }

  cea[cea_idx] = u32::MAX;
}
