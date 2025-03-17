pub const fn pack_weights(
  variable: bool,
  primary: u16,
  secondary: u16,
  tertiary: u16
) -> u32 {
  let upper = (primary as u32) << 16;

  let v_int = variable as u16;
  let lower = (v_int << 15) | (tertiary << 9) | secondary;

  upper | (lower as u32)
}

pub const fn primary(weights: u32) -> u16 {
  (weights >> 16) as u16
}

pub const fn secondary(weights: u32) -> u16 {
  ((weights & 0xFFFF) & 0b1_1111_1111) as u16
}

pub const fn shift_weights(
  weights: u32,
  last_variable: &mut bool
) -> u32 {
  let (variable, primary, _, tertiary) = unpack_weights(weights);

  if variable {
    *last_variable = true;
    pack_weights(true, primary, 0, 0)
  } else if primary == 0 && (tertiary == 0 || *last_variable) {
    0
  } else {
    *last_variable = false;
    weights
  }
}

pub const fn tertiary(weights: u32) -> u16 {
  (((weights & 0xFFFF) >> 9) & 0b11_1111) as u16
}

pub const fn unpack_weights(packed: u32) -> (bool, u16, u16, u16) {
  let primary = (packed >> 16) as u16;

  let lower = (packed & 0xFFFF) as u16;
  let variable = lower >> 15 == 1;
  let secondary = lower & 0b1_1111_1111;
  let tertiary = (lower >> 9) & 0b11_1111;

  (variable, primary, secondary, tertiary)
}

pub const fn variability(weights: u32) -> bool {
  weights & (1 << 15) != 0
}
