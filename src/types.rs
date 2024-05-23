// Basic C language types
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

// Platform dependent C language types
pub use crate::arch::types::{c_char, c_long, c_ulong, wchar_t};

// Wide character types
pub type wint_t = u32;
pub type wctype_t = c_ulong;
pub type wctrans_t = *const int32_t;
pub type char16_t = u16;
pub type char32_t = u32;

#[derive(Clone, Copy)]
pub struct MBStateStruct {
  pub surrogate: char16_t,
  pub bytesleft: c_uint,
  pub partial: char32_t,
  pub lowerbound: char32_t
}

impl MBStateStruct {
  pub const fn new() -> Self {
    Self { bytesleft: 0, partial: 0, lowerbound: 0, surrogate: 0 }
  }
}

pub type mbstate_t = MBStateStruct;
