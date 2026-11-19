use {
  crate::{
    std::stdlib::comparison_fn_t,
    types::{c_char, c_int}
  },
  allocator_api2::boxed::Box,
  core::{ffi::c_void, ptr}
};

pub type free_fn_t = extern "C" fn(*mut c_void);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct entry {
  pub key: *mut c_char,
  pub data: *mut c_void
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ACTION {
  FIND,
  ENTER
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum VISIT {
  endorder,
  leaf,
  postorder,
  preorder
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct posix_tnode {
  pub key: *mut c_void,
  pub left: *mut posix_tnode,
  pub right: *mut posix_tnode
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tdelete(
  key: *const c_void,
  rootp: *mut *mut posix_tnode,
  compar: Option<comparison_fn_t>
) -> *mut c_void {
  unsafe {
    if rootp.is_null() || (*rootp).is_null() {
      return ptr::null_mut();
    }
  }

  let Some(compar) = compar else {
    return ptr::null_mut();
  };

  let mut p: *mut c_void = 1 as *mut c_void;
  let mut l = rootp;
  let mut r = unsafe { *rootp };
  loop {
    unsafe {
      let cmp = compar(key, (*r).key.cast_const());
      if cmp == 0 {
        break;
      }

      p = r as *mut c_void;
      l = if cmp < 0 { &mut (*r).left } else { &mut (*r).right };
      r = *l;

      if r.is_null() {
        return ptr::null_mut();
      }
    }
  }

  let (mut right, mut q) = unsafe { ((*r).right, (*r).left) };
  if q.is_null() {
    q = right;
  } else if !right.is_null() {
    unsafe {
      if (*right).left.is_null() {
        (*right).left = q;
        q = right;
      } else {
        q = (*right).left;
        while !(*q).left.is_null() {
          right = q;
          q = (*right).left;
        }
        (*right).left = (*q).right;
        (*q).left = (*r).left;
        (*q).right = (*r).right;
      }
    }
  }

  unsafe {
    drop(Box::from_raw(r));
    *l = q;
  }
  p
}

#[inline]
extern "C" fn nothing_to_free(_: *mut c_void) {}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tdestroy(
  root: *mut posix_tnode,
  free_node: Option<free_fn_t>
) {
  let free_node = free_node.unwrap_or(nothing_to_free);
  if root.is_null() {
    return;
  }

  let root = unsafe { Box::from_raw(root) };

  rs_tdestroy(root.left, Some(free_node));
  free_node(root.key);
  rs_tdestroy(root.right, Some(free_node));
  drop(root);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tfind(
  key: *const c_void,
  rootp: *const *mut posix_tnode,
  compar: Option<comparison_fn_t>
) -> *mut posix_tnode {
  if rootp.is_null() {
    return ptr::null_mut();
  }
  let Some(compar) = compar else {
    return ptr::null_mut();
  };

  let mut root: *mut posix_tnode = unsafe { *rootp };

  while !root.is_null() {
    let cmp = compar(key, unsafe { (*root).key });
    if cmp < 0 {
      root = unsafe { (*root).left };
    } else if cmp > 0 {
      root = unsafe { (*root).right };
    } else {
      return root;
    }
  }

  ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_tsearch(
  key: *const c_void,
  rootp: *mut *mut posix_tnode,
  compar: Option<comparison_fn_t>
) -> *mut posix_tnode {
  if rootp.is_null() {
    return ptr::null_mut();
  }

  let Some(compar) = compar else {
    return ptr::null_mut();
  };

  let mut l = rootp;
  unsafe {
    while !(*l).is_null() {
      let n = *l;
      let cmp = compar(key, (*n).key.cast_const());

      if cmp == 0 {
        return n;
      }

      l = if cmp < 0 { &mut (*n).left } else { &mut (*n).right }
    }
  }

  let Ok(mut result) = Box::try_new(posix_tnode {
    key: ptr::null_mut(),
    left: ptr::null_mut(),
    right: ptr::null_mut()
  }) else {
    return ptr::null_mut();
  };
  result.key = key.cast_mut();

  let result = Box::into_raw(result);

  unsafe { *l = result };

  result
}

trait WalkAction {
  fn action(
    &self,
    root: *const posix_tnode,
    visit: VISIT,
    depth: i32
  );

  #[inline]
  fn walk(
    &self,
    root: *const posix_tnode,
    depth: i32
  ) {
    if root.is_null() {
      return;
    }

    let root = unsafe { &*root };

    if root.left.is_null() && root.right.is_null() {
      self.action(root, VISIT::leaf, depth);
    } else {
      self.action(root, VISIT::preorder, depth);
      self.walk(root.left, depth + 1);

      self.action(root, VISIT::postorder, depth);
      self.walk(root.right, depth + 2);

      self.action(root, VISIT::endorder, depth);
    }
  }
}

struct Walk {
  pub actionfn: extern "C" fn(*const posix_tnode, VISIT, c_int)
}

impl WalkAction for Walk {
  #[inline]
  fn action(
    &self,
    root: *const posix_tnode,
    visit: VISIT,
    depth: i32
  ) {
    (self.actionfn)(root, visit, depth)
  }
}

struct WalkR {
  pub actionfn: extern "C" fn(*const posix_tnode, VISIT, *mut c_void),
  pub closure: *mut c_void
}

impl WalkAction for WalkR {
  #[inline]
  fn action(
    &self,
    root: *const posix_tnode,
    visit: VISIT,
    _: i32
  ) {
    (self.actionfn)(root, visit, self.closure)
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_twalk(
  root: *const posix_tnode,
  action: Option<extern "C" fn(*const posix_tnode, VISIT, c_int)>
) {
  let Some(action) = action else {
    return;
  };

  let walk = Walk { actionfn: action };
  walk.walk(root, 0);
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_twalk_r(
  root: *const posix_tnode,
  action: Option<extern "C" fn(*const posix_tnode, VISIT, *mut c_void)>,
  closure: *mut c_void
) {
  let Some(action) = action else {
    return;
  };

  let walk = WalkR { actionfn: action, closure };
  walk.walk(root, 0);
}
