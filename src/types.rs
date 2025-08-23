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
pub type char8_t = u8;
pub type char16_t = u16;
pub type char32_t = u32;

// Multi-Byte State
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MBState {
  pub ch: char32_t,
  pub bytesleft: usize,
  pub partial: char32_t,
  pub lowerbound: char32_t,
  pub u8_buffer: [char8_t; 4],
  pub u8_position: usize,
  pub u16_buffer: [char16_t; 2],
  pub u16_surrogate: char16_t
}

impl MBState {
  pub const fn new() -> Self {
    Self {
      ch: 0,
      bytesleft: 0,
      partial: 0,
      lowerbound: 0,
      u8_buffer: [0; 4],
      u8_position: 0,
      u16_buffer: [0; 2],
      u16_surrogate: 0
    }
  }

  pub fn is_initial(&self) -> bool {
    self.ch == 0 &&
      self.bytesleft == 0 &&
      (self.u16_surrogate < 0xd800 || self.u16_surrogate > 0xdfff)
  }

  pub fn reset(&mut self) {
    self.ch = 0;
    self.bytesleft = 0;
    self.partial = 0;
    self.lowerbound = 0;
    self.u8_buffer = [0; 4];
    self.u8_position = 0;
    self.u16_buffer = [0; 2];
    self.u16_surrogate = 0;
  }
}

pub type mbstate_t = MBState;
