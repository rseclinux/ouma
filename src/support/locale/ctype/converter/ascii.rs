use {
  super::ConverterObject,
  crate::{c_schar, char32_t, mbstate_t, ssize_t, std::errno}
};

fn c32tomb(
  s: &mut [u8],
  c32: char32_t
) -> ssize_t {
  if c32 > c_schar::max_value() as char32_t {
    errno::set_errno(errno::EILSEQ);
    return -1;
  }
  s[0] = c32 as u8;
  1
}

fn mbtoc32(
  pc32: &mut char32_t,
  s: &[u8],
  ps: &mut mbstate_t
) -> ssize_t {
  if s.len() < 1 {
    return -2;
  }
  if s[0] > c_schar::max_value() as u8 {
    errno::set_errno(errno::EILSEQ);
    return -1;
  }
  *pc32 = s[0] as char32_t;
  ps.reset();
  1
}

pub const CONVERTER_ASCII: ConverterObject = ConverterObject {
  codeset: c"US-ASCII",
  mb_cur_max: 1,
  mbtoc32: mbtoc32,
  c32tomb: c32tomb
};
