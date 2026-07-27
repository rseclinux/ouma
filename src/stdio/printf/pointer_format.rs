use {
  super::{Argument, Emitter, integer_format},
  crate::{
    stdio::format::{FormatError, LengthModifier, Unsigned},
    support::locale::{ctype::CtypeObject, numeric::NumericObject}
  },
  core::{ascii, ffi::c_void}
};

const NULL: &'static [ascii::Char] = ascii_str!(b"(null)");

#[inline]
pub fn format_pointer<E: Emitter>(
  emitter: &mut E,
  value: *const c_void,
  arg: &Argument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let mut arg = arg.clone();
  if value.is_null() {
    emitter.emit_ascii_slice(NULL)
  } else {
    arg.specifier = 'x';
    arg.flags.alternate_form = true;
    arg.modifier = LengthModifier::Ptrdiff;
    let num = Unsigned::Ptrdiff(value as usize);
    integer_format::format_unsigned(emitter, num, &arg, ctype, numeric)
  }
}
