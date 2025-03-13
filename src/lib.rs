#![no_std]
#![allow(
  unused_macros,
  non_camel_case_types,
  non_upper_case_globals,
  non_snake_case,
  static_mut_refs,
  dead_code
)]
#![feature(thread_local)]

#[macro_use]
mod macros;

extern crate alloc as allocation;
extern crate cbitset;
extern crate critical_section;
extern crate dlmalloc;
extern crate once_cell;

mod alloc;
mod arch;
mod panic;
mod std;
mod support;
mod types;

pub use types::*;
