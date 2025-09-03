pub mod ascii;
pub mod utf8;

use {
  crate::{c_char, c_int, char32_t, mbstate_t, ssize_t},
  core::ffi
};

#[derive(Clone)]
pub struct ConverterObject<'a> {
  pub codeset: &'a ffi::CStr,
  pub mb_cur_max: c_int,
  pub c32tomb: fn(*mut c_char, char32_t) -> ssize_t,
  pub mbtoc32: fn(&mut char32_t, &[u8], &mut mbstate_t) -> ssize_t
}

pub struct AvailableConverters<'a> {
  pub name: &'a str,
  pub converter: ConverterObject<'a>
}

pub const AVAILABLE_CONVERTERS: [AvailableConverters; 4] = [
  AvailableConverters { name: "ASCII", converter: ascii::CONVERTER_ASCII },
  AvailableConverters { name: "US-ASCII", converter: ascii::CONVERTER_ASCII },
  AvailableConverters { name: "UTF8", converter: utf8::CONVERTER_UTF8 },
  AvailableConverters { name: "UTF-8", converter: utf8::CONVERTER_UTF8 }
];
