use {
  super::{Emitter, FormatError, PrintfArgument, values::CString},
  core::ascii
};

#[inline]
pub fn format_narrow_string<E: Emitter>(
  emitter: &mut E,
  value: &[u8],
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  let str_len = match arg.precision {
    | Some(prec) => value.len().min(prec as usize),
    | None => value.len()
  };

  if !arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, str_len)?;
  }

  emitter.emit_u8_slice(&value[..str_len])?;

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, str_len)?;
  }

  Ok(())
}

#[inline]
fn format_wide_string<E: Emitter>(
  emitter: &mut E,
  value: &[u32],
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  let str_len = match arg.precision {
    | Some(prec) => value.len().min(prec as usize),
    | None => value.len()
  };

  if !arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, str_len)?;
  }

  emitter.emit_u32_slice(&value[..str_len])?;

  if arg.flags.left_align {
    emitter.pad_to(arg, ascii::Char::Space, str_len)?;
  }

  Ok(())
}

#[inline]
pub fn format_string<E: Emitter>(
  emitter: &mut E,
  value: &CString,
  arg: &PrintfArgument
) -> Result<(), FormatError> {
  match value {
    | CString::Narrow(v) => format_narrow_string(emitter, v, arg),
    | CString::Wide(v) => format_wide_string(emitter, v, arg)
  }
}
