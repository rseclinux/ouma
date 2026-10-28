use {
  super::{
    Emitter,
    FormatError,
    LengthModifier,
    PrintfArgument,
    integer_format,
    string_format,
    values::{CPointer, Unsigned}
  },
  crate::support::locale::{ctype::CtypeObject, numeric::NumericObject}
};

#[inline]
pub fn format_pointer<E: Emitter>(
  emitter: &mut E,
  value: &CPointer,
  arg: &PrintfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  match value {
    | CPointer::NonNull(v) => {
      let mut arg = arg.clone();
      arg.modifier = LengthModifier::Ptrdiff;
      arg.flags.alternate_form = true;
      arg.specifier = 'x';
      integer_format::format_unsigned(
        emitter,
        Unsigned::Pointer(*v),
        &arg,
        ctype,
        numeric
      )
    },
    | CPointer::Null(v) => string_format::format_narrow_string(emitter, v, arg)
  }
}
