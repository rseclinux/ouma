use {
  super::{Emitter, FormatError, PrintfArgument, values::CChar},
  core::ascii
};

#[inline]
fn format_narrow_char<E: Emitter>(
  emitter: &mut E,
  value: &u8,
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  if !arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, 1)?;
  }

  emitter.emit_u8_slice(&[*value])?;

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, 1)?;
  }

  Ok(())
}

#[inline]
fn format_wide_char<E: Emitter>(
  emitter: &mut E,
  value: &u32,
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  if !arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, 1)?;
  }

  emitter.emit_u32_slice(&[*value])?;

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, 1)?;
  }

  Ok(())
}

#[inline]
pub fn format_char<E: Emitter>(
  emitter: &mut E,
  value: &CChar,
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  match value {
    | CChar::Narrow(v) => format_narrow_char(emitter, v, arg),
    | CChar::Wide(v) => format_wide_char(emitter, v, arg)
  }
}
