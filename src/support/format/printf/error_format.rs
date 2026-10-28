use {
  super::{
    Emitter,
    FormatError,
    PrintfArgument,
    integer_format,
    length::LengthModifier,
    string_format,
    values::Signed
  },
  crate::{
    std::{errno, string},
    support::{
      locale::{
        ctype::CtypeObject,
        messages::MessagesObject,
        numeric::NumericObject
      },
      string::error
    }
  },
  core::ffi::CStr
};

#[inline]
pub fn format_error<E: Emitter>(
  emitter: &mut E,
  arg: &PrintfArgument,
  messages: &MessagesObject,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let mut arg = arg.clone();
  let errno = errno::get_errno();

  if arg.flags.alternate_form {
    let error_name = string::ext::rs_strerrorname_np(errno);
    if error_name.is_null() {
      arg.specifier = 'd';
      arg.modifier = LengthModifier::Default;
      integer_format::format_signed(
        emitter,
        Signed::Int(errno),
        &arg,
        ctype,
        numeric
      )
    } else {
      let error_name = unsafe { CStr::from_ptr(error_name) };
      string_format::format_narrow_string(emitter, error_name.to_bytes(), &arg)
    }
  } else {
    let errors = messages.strerror;
    let unknown_error = messages.misc_messages[0];
    if let Some(e) = errors.get(errno as usize) {
      string_format::format_narrow_string(emitter, e.as_bytes(), &arg)?;
    } else {
      let mut buf = [0u8; 256];
      let result =
        error::get_unknown_error_for_printf(&mut buf, errno, unknown_error);
      match result {
        | Ok(x) => string_format::format_narrow_string(emitter, x, &arg)?,
        | Err(_) => string_format::format_narrow_string(
          emitter,
          unknown_error.as_bytes(),
          &arg
        )?
      }
    }
    Ok(())
  }
}
