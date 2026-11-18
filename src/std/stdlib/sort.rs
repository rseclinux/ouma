use {
  super::{comparison_d_fn_t, comparison_fn_t},
  crate::{
    support::algorithm::{
      smoothsort::Smoothsort,
      sort_common::{ArrayForSort, Comparator}
    },
    types::size_t
  },
  core::ffi::c_void
};

struct CCompar(comparison_fn_t);

impl Comparator for CCompar {
  #[inline]
  fn compare(
    &self,
    a: *const u8,
    b: *const u8
  ) -> i32 {
    (self.0)(a.cast(), b.cast())
  }
}

#[unsafe(no_mangle)]
extern "C" fn rs_qsort(
  base: *mut c_void,
  nmemb: size_t,
  size: size_t,
  compar: Option<comparison_fn_t>
) {
  if base.is_null() || nmemb < 2 || size == 0 {
    return;
  }
  let Some(compar) = compar else {
    return;
  };
  let comparator = CCompar(compar);
  let Some(mut array) = ArrayForSort::from_raw(base.cast(), nmemb, size) else {
    return;
  };
  Smoothsort::new(&mut array, &comparator).smoothsort();
}

struct CComparCtx {
  f: comparison_d_fn_t,
  ctx: *mut c_void
}

impl CComparCtx {
  #[inline]
  pub const fn new(
    f: comparison_d_fn_t,
    ctx: *mut c_void
  ) -> Self {
    Self { f, ctx }
  }
}

impl Comparator for CComparCtx {
  #[inline]
  fn compare(
    &self,
    a: *const u8,
    b: *const u8
  ) -> i32 {
    (self.f)(a.cast(), b.cast(), self.ctx)
  }
}

#[unsafe(no_mangle)]
extern "C" fn rs_qsort_r(
  base: *mut c_void,
  nmemb: size_t,
  size: size_t,
  compar: Option<comparison_d_fn_t>,
  ctx: *mut c_void
) {
  if base.is_null() || nmemb < 2 || size == 0 {
    return;
  }
  let Some(compar) = compar else {
    return;
  };
  let comparator = CComparCtx::new(compar, ctx);
  let Some(mut array) = ArrayForSort::from_raw(base.cast(), nmemb, size) else {
    return;
  };
  Smoothsort::new(&mut array, &comparator).smoothsort();
}
