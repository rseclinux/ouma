pub mod ext;

use {
  crate::{c_int, size_t, wchar_t},
  cbitset::BitSet256,
  core::ptr
};

#[no_mangle]
pub extern "C" fn rs_wmemccpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
  let mut i = n;
  while i != 0 {
    unsafe {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
      if *d.offset(-1) == c {
        return d as *mut wchar_t;
      }
    }
    i -= 1;
  }
  ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn rs_wmemchr(
  s: *const wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut s1 = s;
  let mut i = n;
  while i != 0 {
    unsafe {
      if *s1 == c {
        return s1 as *mut wchar_t;
      }
    }
    s1 = s1.wrapping_offset(1);
    i -= 1;
  }
  ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn rs_wmemcmp(
  left: *const wchar_t,
  right: *const wchar_t,
  n: size_t
) -> c_int {
  let l = left;
  let r = right;
  let mut i = 0;
  while i < n {
    let a = unsafe { *l.offset(i as isize) };
    let b = unsafe { *r.offset(i as isize) };
    if a != b {
      return a as c_int - b as c_int;
    }
    i += 1;
  }
  0
}

#[no_mangle]
pub extern "C" fn rs_wmemcpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
  let mut i = 0;
  while i < n {
    unsafe {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
    }
    i += 1;
  }
  dest
}

#[no_mangle]
pub extern "C" fn rs_wmemmove(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
  if (d as *const wchar_t) < s {
    let mut i = 0;
    while i < n {
      unsafe {
        *d = *s;
        d = d.offset(1);
        s = s.offset(1);
      }
      i += 1;
    }
  } else if (d as *const wchar_t) > s {
    let mut i = n;
    unsafe {
      s = s.offset(i as isize);
      d = d.offset(i as isize);

      while i != 0 {
        d = d.offset(-1);
        s = s.offset(-1);
        *d = *s;
        i -= 1;
      }
    }
  }
  dest
}

#[no_mangle]
pub extern "C" fn rs_wmemset(
  s: *mut wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut s1 = s;
  let mut i = 0;
  while i < n {
    unsafe {
      *s1 = c;
      s1 = s1.offset(1);
    }
    i += 1;
  }
  s
}
