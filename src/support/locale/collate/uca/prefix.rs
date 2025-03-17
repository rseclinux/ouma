use {
  super::consts::{NEED_THREE, NEED_TWO, VARIABLE},
  crate::allocation::vec::Vec
};

pub fn trim_prefix(
  a: &mut Vec<u32>,
  b: &mut Vec<u32>,
  shifting: bool
) {
  let prefix_len = find_prefix(a, b);

  if prefix_len > 0 {
    if shifting && VARIABLE.contains(&a[prefix_len - 1]) {
      if prefix_len > 1 {
        if VARIABLE.contains(&a[prefix_len - 2]) {
          return;
        }

        a.drain(0..prefix_len - 1);
        b.drain(0..prefix_len - 1);
      }

      return;
    }

    a.drain(0..prefix_len);
    b.drain(0..prefix_len);
  }
}

fn find_prefix(
  a: &[u32],
  b: &[u32]
) -> usize {
  a.iter()
    .zip(b.iter())
    .take_while(|(x, y)| {
      x == y && !NEED_TWO.contains(x) && !NEED_THREE.contains(x)
    })
    .count()
}
