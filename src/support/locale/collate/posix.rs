use {
  super::LCCollate,
  crate::{
    c_char,
    c_int,
    size_t,
    std::{string, wchar},
    wchar_t
  }
};

fn strcoll(
  lhs: *const c_char,
  rhs: *const c_char
) -> c_int {
  string::rs_strcmp(lhs, rhs)
}

fn strxfrm(
  dest: *mut c_char,
  src: *const c_char,
  dlen: size_t
) -> size_t {
  if src.is_null() {
    return 0;
  }
  let l = string::rs_strlen(src);
  if dlen >= l {
    string::rs_strncpy(dest, src, dlen);
  }
  l
}

fn wcscoll(
  lhs: *const wchar_t,
  rhs: *const wchar_t
) -> c_int {
  wchar::rs_wcscmp(lhs, rhs)
}

fn wcsxfrm(
  dest: *mut wchar_t,
  src: *const wchar_t,
  dlen: size_t
) -> size_t {
  if src.is_null() {
    return 0;
  }
  let l = wchar::rs_wcslen(src);
  if dlen >= l {
    wchar::rs_wcsncpy(dest, src, dlen);
  }
  l
}

pub const POSIX_COLLATE: LCCollate = LCCollate {
  name: c"".as_ptr(),
  strcoll: strcoll,
  strxfrm: strxfrm,
  wcscoll: wcscoll,
  wcsxfrm: wcsxfrm
};
