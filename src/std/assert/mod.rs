use {
  crate::types::{c_char, c_uint},
  core::ffi::CStr
};

#[unsafe(no_mangle)]
extern "C" fn rs___assert_fail(
  assertion: *const c_char,
  file: *const c_char,
  line: c_uint,
  function: *const c_char
) {
  let assertion =
    unsafe { CStr::from_ptr(assertion).to_str().unwrap_or("<assertion>") };
  let file = unsafe { CStr::from_ptr(file).to_str().unwrap_or("<file>") };
  let function =
    unsafe { CStr::from_ptr(function).to_str().unwrap_or("<function>") };
  let progname = "<progname>"; // TODO: replace it with real getprogname()
  panic!(
    "{progname}: {file}: {line}: {function}: Assertion \"`{assertion}`\" failed."
  ); // TODO: replace with safe_abort!()
}
