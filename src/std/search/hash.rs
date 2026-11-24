use {
  super::{ACTION, ENTRY},
  crate::{
    std::errno::{self, set_errno},
    types::{c_int, c_uint, size_t}
  },
  allocator_api2::{boxed::Box, vec::Vec},
  core::{cell::SyncUnsafeCell, ffi, ptr, slice}
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct __hsearch {
  pub used: size_t,
  pub entry: ENTRY
}

impl __hsearch {
  #[inline]
  pub const fn new() -> Self {
    Self { used: 0, entry: ENTRY::new() }
  }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct hsearch_data {
  pub table: *mut __hsearch,
  pub size: size_t,
  pub filled: size_t
}

unsafe impl Sync for hsearch_data {}
unsafe impl Send for hsearch_data {}

impl hsearch_data {
  #[inline]
  pub const fn new() -> Self {
    Self { table: ptr::null_mut(), size: 0, filled: 0 }
  }
}

#[inline]
fn is_prime(num: usize) -> bool {
  let mut div: usize = 3;
  while div <= num / div {
    if num % div == 0 {
      return false;
    }
    div += 2;
  }
  true
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hcreate_r(
  nel: size_t,
  htab: *mut hsearch_data
) -> c_int {
  if htab.is_null() {
    errno::set_errno(errno::EINVAL);
    return 0;
  }

  let htab = unsafe { &mut *htab };
  let mut nel = nel;

  if !htab.table.is_null() {
    return 0;
  }

  if nel < 3 {
    return 0;
  }

  nel |= 1;
  loop {
    if (c_uint::MAX as usize - 2) < nel {
      errno::set_errno(errno::ENOMEM);
      return 0;
    }
    if is_prime(nel) {
      break;
    }
    nel += 2;
  }

  htab.size = nel;
  htab.filled = 0;

  let mut table_vector: Vec<__hsearch> = Vec::with_capacity(htab.size + 1);
  table_vector.resize_with(nel + 1, || __hsearch::new());

  let boxed_table = table_vector.into_boxed_slice();

  htab.table = Box::leak(boxed_table).as_mut_ptr();

  1
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hdestroy_r(htab: *mut hsearch_data) {
  if htab.is_null() {
    errno::set_errno(errno::EINVAL);
    return;
  }

  let htab = unsafe { &mut *htab };
  if htab.table.is_null() {
    return;
  }
  drop(unsafe {
    Box::from_raw(slice::from_raw_parts_mut(htab.table, htab.size + 1))
  });
  htab.table = ptr::null_mut();
  htab.size = 0;
  htab.filled = 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hsearch_r(
  item: ENTRY,
  action: ACTION,
  reval: *mut *mut ENTRY,
  htab: *mut hsearch_data
) -> c_int {
  if htab.is_null() {
    errno::set_errno(errno::EINVAL);
    return 0;
  }

  let key = unsafe { ffi::CStr::from_ptr(item.key).to_bytes() };
  let htab = unsafe { &mut *htab };

  if htab.table.is_null() {
    errno::set_errno(errno::EINVAL);
    return 0;
  }

  let mut hval = key.len();
  let mut count = key.len();
  while count > 0 {
    count -= 1;
    hval <<= 4;
    hval += key[count] as usize;
  }
  if hval == 0 {
    hval += 1;
  }

  let mut idx = hval % htab.size + 1;

  let table = unsafe { slice::from_raw_parts_mut(htab.table, htab.size + 1) };

  if table[idx].used != 0 {
    let entry_key =
      unsafe { ffi::CStr::from_ptr(table[idx].entry.key).to_bytes() };
    if table[idx].used == hval && key == entry_key {
      unsafe { *reval = &mut table[idx].entry };
      return 1;
    }

    let hval2 = 1 + hval % (htab.size - 2);
    let first_idx = idx;

    loop {
      if idx <= hval2 {
        idx = htab.size + idx - hval2;
      } else {
        idx -= hval2;
      }

      if idx == first_idx {
        break;
      }

      if table[idx].used == hval {
        let ek =
          unsafe { ffi::CStr::from_ptr(table[idx].entry.key).to_bytes() };
        if key == ek {
          unsafe { *reval = &mut table[idx].entry };
          return 1;
        }
      }

      if table[idx].used == 0 {
        break;
      }
    }
  }

  if action == ACTION::ENTER {
    if htab.filled == htab.size {
      errno::set_errno(errno::ENOMEM);
      unsafe { *reval = ptr::null_mut() };
      return 0;
    }

    table[idx].used = hval;
    table[idx].entry = item;

    htab.filled += 1;

    unsafe { *reval = &mut table[idx].entry };
    return 1;
  }

  set_errno(errno::ESRCH);
  unsafe { *reval = ptr::null_mut() };
  0
}

#[thread_local]
static GLOBAL_HTAB: SyncUnsafeCell<hsearch_data> =
  SyncUnsafeCell::new(hsearch_data::new());

#[unsafe(no_mangle)]
pub extern "C" fn rs_hcreate(nel: size_t) -> c_int {
  rs_hcreate_r(nel, GLOBAL_HTAB.get())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hdestroy() {
  rs_hdestroy_r(GLOBAL_HTAB.get());
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hsearch(
  item: ENTRY,
  action: ACTION
) -> *mut ENTRY {
  let mut result: *mut ENTRY = ptr::null_mut();
  if rs_hsearch_r(item, action, &mut result, GLOBAL_HTAB.get()) == 0 {
    return ptr::null_mut();
  }
  result
}
