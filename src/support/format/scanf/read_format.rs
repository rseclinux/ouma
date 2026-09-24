use {
  super::{Consumer, ScanfArgument, utils::write_unsigned_integer},
  crate::{support::format::error::FormatError, types::uintmax_t}
};

pub fn format_read<C: Consumer>(
  consumer: &mut C,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  if arg.allocate {
    return Err(FormatError::BadMatch);
  }

  if !arg.suppress {
    let read = consumer.get_read() as uintmax_t;

    write_unsigned_integer(read, ptr, arg)
  } else {
    Ok(())
  }
}
