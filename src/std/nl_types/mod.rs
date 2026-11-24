use {
  crate::{
    std::errno,
    types::{c_char, c_int}
  },
  core::ffi::c_void
};

pub type nl_catd = *mut c_void;
pub type nl_item = c_int;

pub const NL_CAT_LOCALE: c_int = 1;
pub const NL_SETD: c_int = 1;

#[unsafe(no_mangle)]
extern "C" fn rs_catopen(
  _: *const c_char,
  _: c_int
) -> nl_catd {
  -1isize as nl_catd
}

#[unsafe(no_mangle)]
extern "C" fn rs_catclose(_: nl_catd) -> c_int {
  errno::set_errno(errno::EBADF);
  -1
}

#[unsafe(no_mangle)]
extern "C" fn rs_catgets(
  _: nl_catd,
  _: c_int,
  _: c_int,
  message: *const c_char
) -> *mut c_char {
  errno::set_errno(errno::EBADF);
  message.cast_mut()
}
