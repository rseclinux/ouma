#[derive(Debug, Clone, Copy)]
pub struct NumericGrouping<'a> {
  base: &'a [u8],
  pos: usize,
  steps: usize,
  repetitions: usize,
  pub width: usize
}

impl<'a> NumericGrouping<'a> {
  #[inline]
  pub fn new(
    g: &'a [u8],
    ndigits: usize
  ) -> Self {
    let mut result =
      Self { base: g, pos: 0, steps: 0, repetitions: 0, width: 0 };
    let mut ndigits = ndigits;
    if ndigits <= 1 || g.is_empty() {
      result.steps = usize::MAX;
      result.width = 0;
      return result;
    }
    let mut n = 0usize;
    let mut idx = 0usize;
    loop {
      let cur = g[idx] as usize;
      if ndigits <= cur || cur == 0 {
        result.pos = idx;
        result.steps = ndigits;
        result.repetitions = 0;
        result.width = n;
        break;
      } else if idx == g.len() - 1 {
        result.pos = idx;
        result.steps = ndigits.wrapping_sub(1) % cur + 1;
        result.repetitions = ndigits.wrapping_sub(1) / cur;
        result.width = n + result.repetitions;
        break;
      } else {
        ndigits -= cur;
        idx += 1;
        n += 1;
      }
    }
    result
  }

  #[inline]
  pub fn step(&mut self) -> bool {
    if self.steps != 0 {
      self.steps -= 1;
      return false;
    }
    if self.repetitions != 0 {
      self.repetitions -= 1;
    } else {
      match self.pos.checked_sub(1) {
        | Some(p) => self.pos = p,
        | None => {
          debug_assert!(
            false,
            "numeric_grouping_step called past the last group"
          );
          self.pos = 0;
        }
      }
    }
    self.steps = (self.base[self.pos] as usize).saturating_sub(1);
    true
  }
}
