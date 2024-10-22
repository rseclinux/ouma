use {
  super::LCCtype,
  crate::{
    c_char,
    c_schar,
    c_uchar,
    char32_t,
    mbstate_t,
    size_t,
    ssize_t,
    std::errno,
    support::mbstate
  },
  core::ptr
};

fn mbtoc32(
  pc32: *mut char32_t,
  s: *const c_char,
  n: size_t,
  ps: *mut mbstate_t
) -> ssize_t {
  static mut PRIV: mbstate_t = mbstate_t::new();
  let state = if !ps.is_null() {
    unsafe { &mut *ps }
  } else {
    // TODO: mutex lock
    ptr::addr_of_mut!(PRIV)
  };
  if n < 1 {
    return -2;
  }
  let uc: c_uchar = unsafe { *s as c_uchar };
  if uc > c_schar::max_value() as u8 {
    errno::set_errno(errno::EILSEQ);
    return -1;
  }
  unsafe { *pc32 = uc as char32_t };
  mbstate::mbstate_set_init(state);
  1
}

fn c32tomb(
  s: *mut c_char,
  wc: char32_t,
  _: *mut mbstate_t
) -> ssize_t {
  if wc > c_schar::max_value() as char32_t {
    errno::set_errno(errno::EILSEQ);
    return -1;
  }
  unsafe { *s = wc as c_char };
  1
}

pub const CTYPE_ASCII: LCCtype = LCCtype {
  codeset: c"US-ASCII",
  mbtoc32: mbtoc32,
  c32tomb: c32tomb,
  mb_cur_max: 1
};
