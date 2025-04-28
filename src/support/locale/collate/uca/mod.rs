use {
  super::LCCollate,
  crate::{
    c_char,
    c_int,
    size_t,
    std::{string, wchar},
    wchar_t
  },
  core::{cmp::Ordering, slice}
};

mod types;
pub use types::{Locale, Tailoring};

mod ascii;
mod cea;
mod cea_utils;
mod consts;
mod first_weight;
mod normalize;
mod prefix;
mod sort_key;
mod tailor;
mod weights;

pub mod collate;
pub mod xfrm;

fn strcoll(
  lhs: *const c_char,
  rhs: *const c_char
) -> c_int {
  let lhs: &[u8] =
    unsafe { slice::from_raw_parts(lhs as *const u8, string::rs_strlen(lhs)) };
  let rhs: &[u8] =
    unsafe { slice::from_raw_parts(rhs as *const u8, string::rs_strlen(rhs)) };

  let mut c = collate::Collator::default();
  match c.collate_u8(lhs, rhs) {
    | Ordering::Less => return -1,
    | Ordering::Equal => return 0,
    | Ordering::Greater => return 1
  };
}

pub fn strxfrm(
  dest: *mut c_char,
  src: *const c_char,
  dlen: size_t
) -> size_t {
  if src.is_null() {
    return 0;
  }
  let slen = string::rs_strlen(src);
  if dlen >= slen {
    let source: &[u8] =
      unsafe { slice::from_raw_parts(src as *const u8, slen - 1) };
    let destination: &mut [u8] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u8, dlen) };

    let mut x = xfrm::SortKey::default();
    let sk = x.get_sortkey_u8(source);

    for i in 0..sk.len() {
      destination[i] = sk[i];
    }

    destination[sk.len()] = b'\0';
  }
  slen
}

fn wcscoll(
  lhs: *const wchar_t,
  rhs: *const wchar_t
) -> c_int {
  let lhs: &[u32] =
    unsafe { slice::from_raw_parts(lhs as *const u32, wchar::rs_wcslen(lhs)) };
  let rhs: &[u32] =
    unsafe { slice::from_raw_parts(rhs as *const u32, wchar::rs_wcslen(rhs)) };

  let mut c = collate::Collator::default();
  match c.collate_u32(lhs, rhs) {
    | Ordering::Less => return -1,
    | Ordering::Equal => return 0,
    | Ordering::Greater => return 1
  };
}

fn wcsxfrm(
  dest: *mut wchar_t,
  src: *const wchar_t,
  dlen: size_t
) -> size_t {
  if src.is_null() {
    return 0;
  }
  let slen = wchar::rs_wcslen(src);
  if dlen >= slen {
    let source: &[u32] =
      unsafe { slice::from_raw_parts(src as *const u32, slen - 1) };
    let destination: &mut [u32] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u32, dlen) };

    let mut x = xfrm::SortKey::default();
    let sk = x.get_sortkey_u32(source);

    for i in 0..sk.len() {
      destination[i] = sk[i];
    }

    destination[sk.len()] = '\0' as u32;
  }
  slen
}

pub const UCA_COLLATE: LCCollate = LCCollate {
  name: c"".as_ptr(),
  strcoll: strcoll,
  strxfrm: strxfrm,
  wcscoll: wcscoll,
  wcsxfrm: wcsxfrm
};
