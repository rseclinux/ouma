use {
  super::weights::{primary, secondary, tertiary, variability},
  core::cmp::Ordering
};

pub fn compare_incremental(
  a_cea: &[u32],
  b_cea: &[u32],
  shifting: bool
) -> Ordering {
  if shifting {
    if let Some(o) = compare_primary_shifting(a_cea, b_cea) {
      return o;
    }
  } else if let Some(o) = compare_primary(a_cea, b_cea) {
    return o;
  }

  if let Some(o) = compare_secondary(a_cea, b_cea) {
    return o;
  }

  if let Some(o) = compare_tertiary(a_cea, b_cea) {
    return o;
  }

  if !shifting {
    return Ordering::Equal;
  }

  if let Some(o) = compare_quaternary(a_cea, b_cea) {
    return o;
  }

  Ordering::Equal
}

fn compare_primary(
  a_cea: &[u32],
  b_cea: &[u32]
) -> Option<Ordering> {
  let mut a_filter = a_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| primary(*w))
    .filter(|p| *p != 0);

  let mut b_filter = b_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| primary(*w))
    .filter(|p| *p != 0);

  loop {
    let a_p = a_filter.next().unwrap_or_default();
    let b_p = b_filter.next().unwrap_or_default();

    if a_p != b_p {
      return Some(a_p.cmp(&b_p));
    }

    if a_p == 0 {
      return None;
    }
  }
}

fn compare_primary_shifting(
  a_cea: &[u32],
  b_cea: &[u32]
) -> Option<Ordering> {
  let mut a_filter = a_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .filter(|w| !variability(**w))
    .map(|w| primary(*w))
    .filter(|p| *p != 0);

  let mut b_filter = b_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .filter(|w| !variability(**w))
    .map(|w| primary(*w))
    .filter(|p| *p != 0);

  loop {
    let a_p = a_filter.next().unwrap_or_default();
    let b_p = b_filter.next().unwrap_or_default();

    if a_p != b_p {
      return Some(a_p.cmp(&b_p));
    }

    if a_p == 0 {
      return None;
    }
  }
}

fn compare_secondary(
  a_cea: &[u32],
  b_cea: &[u32]
) -> Option<Ordering> {
  let mut a_filter = a_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| secondary(*w))
    .filter(|s| *s != 0);

  let mut b_filter = b_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| secondary(*w))
    .filter(|s| *s != 0);

  loop {
    let a_s = a_filter.next().unwrap_or_default();
    let b_s = b_filter.next().unwrap_or_default();

    if a_s != b_s {
      return Some(a_s.cmp(&b_s));
    }

    if a_s == 0 {
      return None;
    }
  }
}

fn compare_tertiary(
  a_cea: &[u32],
  b_cea: &[u32]
) -> Option<Ordering> {
  let mut a_filter = a_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| tertiary(*w))
    .filter(|t| *t != 0);

  let mut b_filter = b_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .map(|w| tertiary(*w))
    .filter(|t| *t != 0);

  loop {
    let a_t = a_filter.next().unwrap_or_default();
    let b_t = b_filter.next().unwrap_or_default();

    if a_t != b_t {
      return Some(a_t.cmp(&b_t));
    }

    if a_t == 0 {
      return None;
    }
  }
}

fn compare_quaternary(
  a_cea: &[u32],
  b_cea: &[u32]
) -> Option<Ordering> {
  let mut a_filter = a_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .filter(|w| variability(**w) || tertiary(**w) != 0)
    .map(|w| primary(*w))
    .filter(|q| *q != 0);

  let mut b_filter = b_cea
    .iter()
    .take_while(|x| **x < u32::MAX)
    .filter(|w| variability(**w) || tertiary(**w) != 0)
    .map(|w| primary(*w))
    .filter(|q| *q != 0);

  loop {
    let a_p = a_filter.next().unwrap_or_default();
    let b_p = b_filter.next().unwrap_or_default();

    if a_p != b_p {
      return Some(a_p.cmp(&b_p));
    }

    if a_p == 0 {
      return None;
    }
  }
}
