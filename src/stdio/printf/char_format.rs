use {
  super::{Argument, Emitter},
  crate::stdio::format::{CChar, FormatError},
  core::ascii
};

#[inline]
fn format_narrow_char<E: Emitter>(
  emitter: &mut E,
  v: u8,
  arg: &Argument
) -> Result<(), FormatError> {
  // 1 is because we are passing ONE character
  // emitter.emit_u8_slice will convert it
  // according to the emitter implementation
  let pad: usize = if arg.width > 1 { arg.width - 1 } else { 0 };

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  emitter.emit_u8_slice(&[v])?;

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  Ok(())
}

#[inline]
fn format_wide_char<E: Emitter>(
  emitter: &mut E,
  v: u32,
  arg: &Argument
) -> Result<(), FormatError> {
  // 1 is because we are passing ONE character
  // emitter.emit_u32_slice will convert it
  // according to the emitter implementation
  let pad: usize = if arg.width > 1 { arg.width - 1 } else { 0 };

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  emitter.emit_u32_slice(&[v])?;

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  Ok(())
}

#[inline]
pub fn format_char<E: Emitter>(
  emitter: &mut E,
  v: CChar,
  arg: &Argument
) -> Result<(), FormatError> {
  match v {
    | CChar::Narrow(x) => format_narrow_char(emitter, x, arg),
    | CChar::Wide(x) => format_wide_char(emitter, x, arg)
  }
}
