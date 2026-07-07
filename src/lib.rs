//    A slow-burn, bone-chilling, spine-tingling, genre-redefining hardened
//    libc implementation for GNU/Linux opeating systems.
//
//    Copyright (C) 2023-2026 rsec GNU/Linux-libre
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU Affero General Public License as
//    published by the Free Software Foundation, either version 3 of the
//    License, or (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the GNU Affero General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

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
