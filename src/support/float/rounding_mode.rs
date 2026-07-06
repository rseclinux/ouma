#[derive(PartialEq)]
pub enum Rounding {
  ToNearest,
  Downward,
  Upward,
  TowardZero,
  Even
}

#[inline]
pub fn quick_get_round() -> Rounding {
  static X: f32 = f32::from_bits(0x3380_0000); // 0x1.0p-24f

  let y: f32 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(X)) };

  let z: f32 = core::hint::black_box(
    core::hint::black_box(f32::from_bits(0x3f80_0001) + y) +
      core::hint::black_box(-1.0_f32 - y)
  );

  if z == 0.0_f32 {
    return Rounding::Downward;
  }
  if z == f32::from_bits(0x3380_0001) {
    return Rounding::TowardZero;
  }

  let probe: f32 = core::hint::black_box(core::hint::black_box(2.0_f32 + y));
  if probe == 2.0_f32 { Rounding::ToNearest } else { Rounding::Upward }
}
