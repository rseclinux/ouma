use crate::{c_uint, char8_t, char16_t, char32_t, mbstate_t};

#[inline(always)]
pub fn mbstate_set_init(mbs: *mut mbstate_t) {
  if !mbs.is_null() {
    unsafe {
      *mbs = mbstate_t::new();
    }
  }
}

#[inline(always)]
pub fn mbstate_get_init(mbs: *const mbstate_t) -> bool {
  return unsafe {
    mbs.is_null() ||
      ((*mbs).surrogate < 0xd800 || (*mbs).surrogate > 0xdfff) &&
        (*mbs).bytesleft == 0
  };
}

#[inline(always)]
pub fn mbstate_set_multibyte(
  mbs: *mut mbstate_t,
  bytesleft: c_uint,
  partial: char32_t,
  lowerbound: char32_t
) {
  unsafe {
    (*mbs).bytesleft = bytesleft;
    (*mbs).partial = partial;
    (*mbs).lowerbound = lowerbound;
  }
}

#[inline(always)]
pub fn mbstate_get_multibyte(
  mbs: *const mbstate_t,
  bytesleft: *mut c_uint,
  partial: *mut char32_t,
  lowerbound: *mut char32_t
) {
  unsafe {
    *bytesleft = (*mbs).bytesleft;
    *partial = (*mbs).partial;
    *lowerbound = (*mbs).lowerbound;
  }
}

#[inline(always)]
pub fn mbstate_set_surrogate(
  mbs: *mut mbstate_t,
  surrogate: char16_t
) {
  assert_eq!(surrogate >= 0xd800 && surrogate <= 0xdfff, true);
  unsafe { (*mbs).surrogate = surrogate };
}

#[inline(always)]
pub fn mbstate_get_surrogate(
  mbs: *const mbstate_t,
  surrogate: *mut char16_t
) -> bool {
  unsafe {
    if (*mbs).surrogate < 0xd800 || (*mbs).surrogate > 0xdfff {
      return false;
    }
    *surrogate = (*mbs).surrogate;
  }
  return true;
}

#[inline(always)]
pub fn mbstate_set_codeunit(
  mbs: *mut mbstate_t,
  value: char8_t,
  index: usize
) {
  unsafe {
    (*mbs).codeunits[index] = value;
  }
}

#[inline(always)]
pub fn mbstate_get_codeunit(
  mbs: *const mbstate_t,
  value: *mut char8_t,
  index: usize
) {
  unsafe {
    *value = (*mbs).codeunits[index];
  }
}
