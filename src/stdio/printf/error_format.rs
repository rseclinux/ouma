use {
  super::{Argument, integer_format},
  crate::{
    std::{errno, string},
    stdio::{
      format::{FormatError, LengthModifier, Signed},
      printf::Emitter
    },
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
  arg: &Argument,
  emitter: &mut E,
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
      arg.modifier = LengthModifier::Int;
      integer_format::format_signed(
        emitter,
        Signed::Int(errno),
        &arg,
        ctype,
        numeric
      )
    } else {
      let error_name = unsafe { CStr::from_ptr(error_name) };
      emitter.emit_u8_slice(error_name.to_bytes())
    }
  } else {
    let unknown_error = messages.misc_messages[0];
    // TODO: replace 255 with NL_TEXTMAX
    let mut buffer = [0u8; 255];

    let ret = error::get_error_string(&mut buffer, errno, messages);
    if let Ok(s) = ret {
      emitter.emit_u8_slice(s)
    } else {
      emitter.emit_u8_slice(unknown_error.as_bytes())
    }
  }
}
