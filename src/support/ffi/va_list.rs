use {
  crate::arch::va_list::{ExtVaListInner, LDBLBytes, get_long_double_bytes},
  core::{
    ffi::{VaArgSafe, VaList},
    intrinsics::{va_arg, va_copy, va_end},
    marker::PhantomCovariantLifetime
  }
};

#[repr(transparent)]
pub struct ExtVaList<'a> {
  pub inner: ExtVaListInner,
  _marker: PhantomCovariantLifetime<'a>
}

impl core::fmt::Debug for ExtVaList<'_> {
  fn fmt(
    &self,
    f: &mut core::fmt::Formatter<'_>
  ) -> core::fmt::Result {
    f.debug_tuple("ExtVaList").field(&self.inner).finish()
  }
}

impl<'a> ExtVaList<'_> {
  #[inline]
  pub unsafe fn from_va_list(v: VaList<'a>) -> Self {
    unsafe { core::mem::transmute(v) }
  }
}

impl<'f> ExtVaList<'f> {
  #[inline]
  pub const unsafe fn next_arg<T: VaArgSafe>(&mut self) -> T {
    unsafe { va_arg(core::mem::transmute(self)) }
  }

  #[inline]
  pub unsafe fn get_ldbl_bytes(&mut self) -> LDBLBytes {
    unsafe { get_long_double_bytes(self) }
  }
}

impl<'f> Clone for ExtVaList<'f> {
  #[inline]
  fn clone(&self) -> Self {
    unsafe { core::mem::transmute(va_copy(core::mem::transmute(self))) }
  }
}

impl<'f> Drop for ExtVaList<'f> {
  #[inline]
  fn drop(&mut self) {
    unsafe { va_end(core::mem::transmute(self)) }
  }
}

// Check size and alignment on compile rather leaving it to panic on runtime
const _: () =
  assert!(core::mem::size_of::<VaList>() == core::mem::size_of::<ExtVaList>());
const _: () = assert!(
  core::mem::align_of::<VaList>() == core::mem::align_of::<ExtVaList>()
);
