use {crate::allocation::vec::Vec, core::cmp::Ordering};

pub fn fill<'a>(
  a_iter: &mut impl Iterator<Item = &'a u32>,
  a_chars: &mut Vec<u32>
) {
  loop {
    let Some(a) = a_iter.next() else { break };
    a_chars.push(*a);

    if !ascii_alphanumeric(*a) {
      break;
    }
  }

  a_chars.extend(a_iter);
}

pub fn fill_and_check<'a>(
  a_iter: &mut impl Iterator<Item = &'a u32>,
  b_iter: &mut impl Iterator<Item = &'a u32>,
  a_chars: &mut Vec<u32>,
  b_chars: &mut Vec<u32>
) -> Option<Ordering> {
  let mut backup: Option<Ordering> = None;
  let mut bad = false;

  loop {
    let Some(a) = a_iter.next() else { break };
    a_chars.push(*a);

    if !ascii_alphanumeric(*a) {
      bad = true;
      break;
    }

    let Some(b) = b_iter.next() else { break };
    b_chars.push(*b);

    if !ascii_alphanumeric(*b) {
      bad = true;
      break;
    }

    if a == b {
      continue;
    }

    let a_folded = if *a > 0x005A { a - 0x20 } else { *a };
    let b_folded = if *b > 0x005A { b - 0x20 } else { *b };

    if a_folded == b_folded {
      if backup.is_none() {
        backup = Some(b.cmp(&a));
      }

      continue;
    }

    return Some(a_folded.cmp(&b_folded));
  }

  a_chars.extend(a_iter);
  b_chars.extend(b_iter);

  if bad {
    return None;
  }

  if a_chars.len() != b_chars.len() {
    return Some(a_chars.len().cmp(&b_chars.len()));
  }

  backup
}

// TODO: replace with isalnum
fn ascii_alphanumeric(c: u32) -> bool {
  (0x0030..=0x007A).contains(&c) &&
    !(0x003A..=0x0040).contains(&c) &&
    !(0x005B..=0x0060).contains(&c)
}
