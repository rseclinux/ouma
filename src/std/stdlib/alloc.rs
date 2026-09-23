use {
  crate::{
    std::errno,
    types::{c_int, max_align_t, size_t}
  },
  allocation::alloc,
  core::{ffi::c_void, ptr}
};

#[derive(Debug)]
#[repr(C)]
struct Tag {
  size: usize,
  alignment: usize
}

const TAG_HEADER_SIZE: usize = core::mem::size_of::<Tag>();
const PTR_ALIGNMENT: usize = core::mem::align_of::<max_align_t>();

#[unsafe(no_mangle)]
pub extern "C" fn rs_malloc(size: size_t) -> *mut c_void {
  let tag = Tag { size, alignment: PTR_ALIGNMENT };

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  unsafe {
    ptr::write(ptr as *mut Tag, tag);

    ptr.add(TAG_HEADER_SIZE).cast()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_aligned_alloc(
  alignment: size_t,
  size: size_t
) -> *mut c_void {
  if !alignment.is_power_of_two() || size % alignment != 0 {
    errno::set_errno(errno::EINVAL);
    return ptr::null_mut();
  }

  let tag = Tag { size, alignment };

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  unsafe {
    ptr::write(ptr as *mut Tag, tag);

    ptr.add(TAG_HEADER_SIZE).cast()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_calloc(
  n: size_t,
  size: size_t
) -> *mut c_void {
  let Some(total) = n.checked_mul(size) else {
    errno::set_errno(errno::EINVAL);
    return ptr::null_mut();
  };

  let tag = Tag { size: total, alignment: PTR_ALIGNMENT };

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  unsafe {
    ptr::write(ptr as *mut Tag, tag);

    ptr.add(TAG_HEADER_SIZE).cast()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_realloc(
  ptr: *mut c_void,
  new_size: size_t
) -> *mut c_void {
  if ptr.is_null() {
    return rs_malloc(new_size);
  } else if new_size == 0 {
    errno::set_errno(errno::EINVAL);
    rs_free(ptr);
    return ptr::null_mut();
  }

  let ptr = ptr as *mut u8;
  let ptr = unsafe { ptr.sub(TAG_HEADER_SIZE) };
  let tag: Tag = unsafe { ptr::read(ptr as *const Tag) };

  let Ok(old_layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let ptr =
    unsafe { alloc::realloc(ptr, old_layout, new_size + TAG_HEADER_SIZE) };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  let new_tag = Tag { size: new_size, alignment: PTR_ALIGNMENT };

  unsafe {
    ptr::write(ptr as *mut Tag, new_tag);

    ptr.add(TAG_HEADER_SIZE).cast()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_reallocarray(
  ptr: *mut c_void,
  nelem: size_t,
  size: size_t
) -> *mut c_void {
  let Some(total) = nelem.checked_mul(size) else {
    errno::set_errno(errno::EINVAL);
    return ptr::null_mut();
  };
  rs_realloc(ptr, total)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_free(ptr: *mut c_void) {
  if ptr.is_null() {
    return;
  }

  let ptr = ptr as *mut u8;
  let ptr = unsafe { ptr.sub(TAG_HEADER_SIZE) };
  let tag: Tag = unsafe { ptr::read(ptr as *const Tag) };

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    return;
  };

  unsafe { alloc::dealloc(ptr, layout) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_free_sized(
  ptr: *mut c_void,
  size: size_t
) {
  if ptr.is_null() {
    return;
  }

  let ptr = ptr as *mut u8;
  let ptr = unsafe { ptr.sub(TAG_HEADER_SIZE) };
  let tag: Tag = unsafe { ptr::read(ptr as *const Tag) };

  if size != tag.size {
    return;
  }

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    return;
  };

  unsafe { alloc::dealloc(ptr, layout) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_free_aligned_sized(
  ptr: *mut c_void,
  align: size_t,
  size: size_t
) {
  if ptr.is_null() {
    return;
  }

  let ptr = ptr as *mut u8;
  let ptr = unsafe { ptr.sub(TAG_HEADER_SIZE) };
  let tag: Tag = unsafe { ptr::read(ptr as *const Tag) };

  if size != tag.size {
    return;
  }
  if align != tag.alignment {
    return;
  }

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    return;
  };

  unsafe { alloc::dealloc(ptr, layout) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memalignment(ptr: *const c_void) -> size_t {
  let ptr = ptr as isize;
  (ptr & -ptr) as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_posix_memalign(
  memptr: *mut *mut c_void,
  alignment: size_t,
  size: size_t
) -> c_int {
  if !alignment.is_power_of_two() ||
    alignment < core::mem::size_of::<*const c_void>()
  {
    return errno::EINVAL;
  }

  let tag = Tag { size, alignment };

  let Ok(layout) =
    alloc::Layout::from_size_align(tag.size + TAG_HEADER_SIZE, tag.alignment)
  else {
    return errno::ENOMEM;
  };

  let ptr = unsafe { alloc::alloc(layout) };
  if ptr.is_null() {
    return errno::ENOMEM;
  }

  unsafe {
    ptr::write(ptr as *mut Tag, tag);

    *memptr = ptr.add(TAG_HEADER_SIZE).cast();
  }

  0
}
