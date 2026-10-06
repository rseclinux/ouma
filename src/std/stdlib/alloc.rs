use {
  core::{ffi::c_void, ptr},
  allocation::alloc::{self, Layout}
};
use crate::types::c_int;
use crate::types::max_align_t;
use crate::types::size_t;
use crate::std::errno;

#[derive(Debug)]
#[repr(C)]
struct Tag {
  size: usize,
  alignment: usize
}

const TAG_HEADER_SIZE: usize = core::mem::size_of::<Tag>();
const TAG_HEADER_ALIGNMENT: usize = core::mem::align_of::<Tag>();
const PTR_ALIGNMENT: usize = core::mem::align_of::<max_align_t>();

#[inline]
fn round_up(
  n: usize,
  align: usize
) -> usize {
  (n + align - 1) & !(align - 1)
}

#[inline]
fn header_offset(alignment: usize) -> usize {
  round_up(TAG_HEADER_SIZE, alignment.max(TAG_HEADER_ALIGNMENT))
}

#[inline]
fn tagged_alloc(
  size: usize,
  alignment: usize,
  zeroize: bool
) -> *mut u8 {
  let tag = Tag { size, alignment };
  let offset = header_offset(tag.alignment);

  let Some(total) = tag.size.checked_add(offset) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let Ok(layout) =
    Layout::from_size_align(total, tag.alignment.max(TAG_HEADER_ALIGNMENT))
  else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let ptr = unsafe {
    if zeroize { alloc::alloc_zeroed(layout) } else { alloc::alloc(layout) }
  };
  if ptr.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  unsafe {
    let result = ptr.add(offset);
    ptr::write(result.sub(TAG_HEADER_SIZE) as *mut Tag, tag);
    result.cast()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_malloc(size: size_t) -> *mut c_void {
  tagged_alloc(size, PTR_ALIGNMENT, false).cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_calloc(
  n: size_t,
  size: size_t
) -> *mut c_void {
  let Some(total) = n.checked_mul(size) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };
  tagged_alloc(total, PTR_ALIGNMENT, true).cast()
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
  tagged_alloc(size, alignment, false).cast()
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

  let ptr = tagged_alloc(size, alignment, false);
  if ptr.is_null() {
    return errno::ENOMEM;
  }

  unsafe { *memptr = ptr.cast() };

  0
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

  let user = ptr as *mut u8;
  let tag: Tag = unsafe { ptr::read(user.sub(TAG_HEADER_SIZE) as *const Tag) };

  let align = tag.alignment.max(TAG_HEADER_ALIGNMENT);
  let offset = header_offset(tag.alignment);

  let Some(old_total) = tag.size.checked_add(offset) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };
  let Ok(old) = Layout::from_size_align(old_total, align) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };
  let Some(new_total) = new_size.checked_add(offset) else {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  };

  let oldptr = unsafe { user.sub(offset) };
  let new = unsafe { alloc::realloc(oldptr, old, new_total) };
  if new.is_null() {
    errno::set_errno(errno::ENOMEM);
    return ptr::null_mut();
  }

  let new_tag = Tag { size: new_size, alignment: tag.alignment };
  let result = unsafe { new.add(offset) };

  unsafe { ptr::write(result.sub(TAG_HEADER_SIZE) as *mut Tag, new_tag) };

  result.cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_reallocarray(
  ptr: *mut c_void,
  nelem: size_t,
  size: size_t
) -> *mut c_void {
  let Some(total) = nelem.checked_mul(size) else {
    errno::set_errno(errno::ENOMEM);
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

  let offset = header_offset(tag.alignment);
  let Ok(layout) = Layout::from_size_align(
    tag.size + offset,
    tag.alignment.max(TAG_HEADER_ALIGNMENT)
  ) else {
    return;
  };

  let raw = unsafe { ptr.sub(offset - TAG_HEADER_SIZE) };
  unsafe { alloc::dealloc(raw, layout) };
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

  let offset = header_offset(tag.alignment);
  let Ok(layout) = Layout::from_size_align(
    tag.size + offset,
    tag.alignment.max(TAG_HEADER_ALIGNMENT)
  ) else {
    return;
  };

  let raw = unsafe { ptr.sub(offset - TAG_HEADER_SIZE) };
  unsafe { alloc::dealloc(raw, layout) };
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

  let offset = header_offset(tag.alignment);
  let Ok(layout) = Layout::from_size_align(
    tag.size + offset,
    tag.alignment.max(TAG_HEADER_ALIGNMENT)
  ) else {
    return;
  };

  let raw = unsafe { ptr.sub(offset - TAG_HEADER_SIZE) };
  unsafe { alloc::dealloc(raw, layout) };
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memalignment(ptr: *const c_void) -> size_t {
  let ptr = ptr as isize;
  (ptr & -ptr) as size_t
}
