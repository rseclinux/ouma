#![no_std]
#![allow(
  unused_macros,
  non_camel_case_types,
  non_upper_case_globals,
  non_snake_case,
  dead_code
)]
#![feature(thread_local)]

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
