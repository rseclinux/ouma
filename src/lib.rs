#![no_std]
#![allow(nonstandard_style, unused_macros, dead_code)]
#![feature(
  thread_local,
  cstr_display,
  allocator_api,
  ascii_char,
  ascii_char_variants,
  f128
)]

#[macro_use]
mod macros;

extern crate alloc as allocation;

mod alloc;
mod arch;
mod panic;
mod std;
mod support;
mod types;

pub use types::*;
