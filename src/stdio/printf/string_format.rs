use {
  super::{Argument, Emitter},
  crate::stdio::format::{CString, FormatError},
  core::ascii
};

#[inline]
fn format_narrow_string<E: Emitter>(
  emitter: &mut E,
  v: &[u8],
  arg: &Argument
) -> Result<(), FormatError> {
  let has_prec = arg.precision.is_some();
  let precision = arg.precision.unwrap_or(0) as usize;
  let mut slen = v.len();

  if has_prec && precision < slen {
    slen = precision;
  }

  let pad: usize = if arg.width > slen { arg.width - slen } else { 0 };

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  emitter.emit_u8_slice(&v[..slen])?;

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  Ok(())
}

#[inline]
fn format_wide_string<E: Emitter>(
  emitter: &mut E,
  v: &[u32],
  arg: &Argument
) -> Result<(), FormatError> {
  let has_prec = arg.precision.is_some();
  let precision = arg.precision.unwrap_or(0) as usize;
  let mut slen = v.len();

  if has_prec && precision < slen {
    slen = precision;
  }

  let pad: usize = if arg.width > slen { arg.width - slen } else { 0 };

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  emitter.emit_u32_slice(&v[..slen])?;

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  Ok(())
}

#[inline]
fn format_ascii_string<E: Emitter>(
  emitter: &mut E,
  v: &[ascii::Char],
  arg: &Argument
) -> Result<(), FormatError> {
  let has_prec = arg.precision.is_some();
  let precision = arg.precision.unwrap_or(0) as usize;
  let mut slen = v.len();

  if has_prec && precision < slen {
    slen = precision;
  }

  let pad: usize = if arg.width > slen { arg.width - slen } else { 0 };

  if pad > 0 && !arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  emitter.emit_ascii_slice(&v[..slen])?;

  if pad > 0 && arg.flags.left_align {
    emitter.pad_to(ascii::Char::Space, pad)?;
  }

  Ok(())
}

#[inline]
pub fn format_string<E: Emitter>(
  emitter: &mut E,
  v: CString,
  arg: &Argument
) -> Result<(), FormatError> {
  match v {
    | CString::Narrow(x) => format_narrow_string(emitter, x, arg),
    | CString::Wide(x) => format_wide_string(emitter, x, arg),
    | CString::Ascii(x) => format_ascii_string(emitter, x, arg)
  }
}
