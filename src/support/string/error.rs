use {
  super::cbuf::CBufWriter,
  crate::{c_int, std::errno, support::locale::messages::MessagesObject},
  core::fmt::Write
};

#[inline]
pub fn get_error_string<'a>(
  buffer: &'a mut [u8],
  num: c_int,
  messages: &MessagesObject
) -> Result<&'a mut [u8], c_int> {
  let misc = messages.misc_messages;
  let unknown = misc[0];
  let errors = messages.strerror;
  let mut writer = CBufWriter::new(buffer);

  if let Some(e) = errors.get(num as usize) {
    if write!(&mut writer, "{}\0", e).is_err() {
      buffer[buffer.len() - 1] = b'\0';
      Err(errno::ERANGE)
    } else {
      Ok(buffer)
    }
  } else {
    if write!(&mut writer, "{} {num}\0", unknown).is_err() {
      buffer[buffer.len() - 1] = b'\0';
    }
    Err(errno::EINVAL)
  }
}
