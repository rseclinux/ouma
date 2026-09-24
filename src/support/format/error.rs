use crate::{std::errno, types::c_int};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormatError {
  InvalidSequence,
  InvalidArg,
  Allocation,
  BadMatch,
  BadFileDesc,
  EndOfFile,
  Overflow,
  Io
}

impl FormatError {
  #[inline]
  pub fn to_errno(&self) -> c_int {
    match self {
      | Self::EndOfFile => 0,
      | Self::Allocation => errno::ENOMEM,
      | Self::InvalidArg => errno::EINVAL,
      | Self::InvalidSequence => errno::EILSEQ,
      | Self::BadMatch => 0,
      | Self::BadFileDesc => errno::EBADF,
      | Self::Overflow => errno::EOVERFLOW,
      | Self::Io => errno::EIO
    }
  }
}
