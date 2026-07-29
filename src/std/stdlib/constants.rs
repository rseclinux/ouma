#![allow(dead_code)]

use once_cell::sync::Lazy;

pub const MB_LEN_MAX: usize = 16;
pub static MB_CUR_MAX: Lazy<usize> =
  Lazy::new(|| super::__oumainternal_get_mb_cur_max());
