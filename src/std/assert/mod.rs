use {
  crate::types::{c_char, c_int},
  core::ffi::CStr
};

#[unsafe(no_mangle)]
extern "C" fn ouma_assertion_failed(
  func: *const c_char,
  file: *const c_char,
  line: c_int,
  expr: *const c_char
) -> ! {
  let func = unsafe { CStr::from_ptr(func).to_str().unwrap_or("<function>") };
  let file = unsafe { CStr::from_ptr(file).to_str().unwrap_or("<file>") };
  let expr = unsafe { CStr::from_ptr(expr).to_str().unwrap_or("<expression>") };
  panic!("{file}: {line}: assertion failed in {func}() at {expr}")
}
