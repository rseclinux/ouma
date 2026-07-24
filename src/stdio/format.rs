use {
  crate::{
    c_char,
    c_int,
    c_long,
    c_longlong,
    c_uint,
    c_ulong,
    c_ulonglong,
    intmax_t,
    std::{string, wchar},
    support::{
      ffi::va_list::ExtVaList,
      locale::ctype::CtypeObject,
      string::conversion::strtoint::strtoint,
      traits::char::{CharToAscii, get_char_with_index}
    },
    uintmax_t,
    wchar_t
  },
  core::slice,
  num_traits::One
};

#[derive(Debug, Clone, Copy)]
pub enum FormatError {
  WriteIo,
  InvalidArg,
  NumberConversion,
  InvalidSequence,
  Allocation,
  Overflow
}

#[derive(Debug, Clone, Copy)]
pub enum Signed {
  Byte(crate::types::c_schar),
  Short(crate::types::c_short),
  Int(crate::types::c_int),
  Long(crate::types::c_long),
  LongLong(crate::types::c_longlong),
  Size(crate::types::ssize_t),
  Intmax(crate::types::intmax_t),
  Ptrdiff(crate::types::ptrdiff_t)
}

#[derive(Debug, Clone, Copy)]
pub enum Unsigned {
  Byte(crate::types::c_uchar),
  Short(crate::types::c_ushort),
  Int(crate::types::c_uint),
  Long(crate::types::c_ulong),
  LongLong(crate::types::c_ulonglong),
  Size(crate::types::size_t),
  Intmax(crate::types::uintmax_t),
  Ptrdiff(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum CChar {
  Narrow(u8),
  Wide(u32)
}

#[derive(Debug, Clone, Copy)]
pub enum CString<'a> {
  Narrow(&'a [u8]),
  Wide(&'a [u32])
}

trait CharToInt {
  type IntType;
}

impl CharToInt for crate::types::c_schar {
  type IntType = crate::types::c_int;
}

impl CharToInt for crate::types::c_uchar {
  type IntType = crate::types::c_uint;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthModifier {
  Byte,
  Short,
  Int,
  Long,
  LongLong,
  LongFloat,
  Size,
  Intmax,
  Ptrdiff,
  Bit(usize),
  BitFast(usize)
}

impl Default for LengthModifier {
  #[inline]
  fn default() -> Self {
    Self::Int
  }
}

// (null)
const NULL_STR_WIDE: &'static [u32] =
  &['(' as u32, 'n' as u32, 'u' as u32, 'l' as u32, 'l' as u32, ')' as u32];

#[inline]
fn get_num_mask_from_bitwidth(bw: uintmax_t) -> uintmax_t {
  let m: uintmax_t;
  if bw == 0 {
    m = 0;
  } else if bw < (size_of::<uintmax_t>() as uintmax_t) * 8 {
    m = (uintmax_t::one() << bw).wrapping_sub(1);
  } else {
    m = uintmax_t::MAX;
  }
  m
}

impl LengthModifier {
  #[inline]
  pub unsafe fn parse_signed(
    self,
    va: &mut ExtVaList
  ) -> Result<Signed, FormatError> {
    match self {
      | LengthModifier::Byte => Ok(Signed::Byte(unsafe {
        va.next_arg::<crate::types::c_int>() as crate::types::c_schar &
          crate::types::c_schar::MAX
      })),
      | LengthModifier::Short => Ok(Signed::Short(unsafe {
        va.next_arg::<crate::types::c_int>() as crate::types::c_short &
          crate::types::c_short::MAX
      })),
      | LengthModifier::Int => Ok(Signed::Int(
        unsafe { va.next_arg::<crate::types::c_int>() } &
          crate::types::c_int::MAX
      )),
      | LengthModifier::Long => Ok(Signed::Long(
        unsafe { va.next_arg::<crate::types::c_long>() } &
          crate::types::c_long::MAX
      )),
      | LengthModifier::LongLong => Ok(Signed::LongLong(
        unsafe { va.next_arg::<crate::types::c_longlong>() } &
          crate::types::c_longlong::MAX
      )),
      | LengthModifier::Size => Ok(Signed::Size(
        unsafe { va.next_arg::<crate::types::ssize_t>() } &
          crate::types::ssize_t::MAX
      )),
      | LengthModifier::Intmax => Ok(Signed::Intmax(
        unsafe { va.next_arg::<crate::types::intmax_t>() } &
          crate::types::intmax_t::MAX
      )),
      | LengthModifier::Ptrdiff => Ok(Signed::Ptrdiff(
        unsafe { va.next_arg::<crate::types::ptrdiff_t>() } &
          crate::types::ptrdiff_t::MAX
      )),
      | LengthModifier::Bit(x) => {
        let r: intmax_t = if c_int::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_int>() as intmax_t }
        } else if c_long::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_long>() as intmax_t }
        } else if c_longlong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_longlong>() as intmax_t }
        } else {
          unsafe { va.next_arg::<crate::types::intmax_t>() }
        };
        let mask = get_num_mask_from_bitwidth(x as uintmax_t) as intmax_t;
        Ok(Signed::Intmax(r & mask))
      },
      | LengthModifier::BitFast(x) => {
        let r: intmax_t = if c_int::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_int>() as intmax_t }
        } else if c_long::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_long>() as intmax_t }
        } else if c_longlong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_longlong>() as intmax_t }
        } else {
          unsafe { va.next_arg::<crate::types::intmax_t>() }
        };
        let mask = get_num_mask_from_bitwidth(x as uintmax_t) as intmax_t;
        Ok(Signed::Intmax(r & mask))
      },
      | _ => Err(FormatError::InvalidArg)
    }
  }

  #[inline]
  pub unsafe fn parse_unsigned(
    self,
    va: &mut ExtVaList
  ) -> Result<Unsigned, FormatError> {
    match self {
      | LengthModifier::Byte => Ok(Unsigned::Byte(
        unsafe {
          va.next_arg::<crate::types::c_uint>() as crate::types::c_uchar
        } & crate::types::c_uchar::MAX
      )),
      | LengthModifier::Short => Ok(Unsigned::Short(
        unsafe {
          va.next_arg::<crate::types::c_uint>() as crate::types::c_ushort
        } & crate::types::c_ushort::MAX
      )),
      | LengthModifier::Int => Ok(Unsigned::Int(
        unsafe { va.next_arg::<crate::types::c_uint>() } &
          crate::types::c_uint::MAX
      )),
      | LengthModifier::Long => Ok(Unsigned::Long(
        unsafe { va.next_arg::<crate::types::c_ulong>() } &
          crate::types::c_ulong::MAX
      )),
      | LengthModifier::LongLong => Ok(Unsigned::LongLong(
        unsafe { va.next_arg::<crate::types::c_ulonglong>() } &
          crate::types::c_ulonglong::MAX
      )),
      | LengthModifier::Size => Ok(Unsigned::Size(
        unsafe { va.next_arg::<crate::types::size_t>() } &
          crate::types::size_t::MAX
      )),
      | LengthModifier::Intmax => Ok(Unsigned::Intmax(
        unsafe { va.next_arg::<crate::types::uintmax_t>() } &
          crate::types::uintmax_t::MAX
      )),
      | LengthModifier::Ptrdiff => {
        Ok(Unsigned::Ptrdiff(unsafe { va.next_arg::<usize>() } & usize::MAX))
      },
      | LengthModifier::Bit(x) => {
        let r: uintmax_t = if c_uint::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_uint>() as uintmax_t }
        } else if c_ulong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_ulong>() as uintmax_t }
        } else if c_ulonglong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_ulonglong>() as uintmax_t }
        } else {
          unsafe { va.next_arg::<crate::types::uintmax_t>() }
        };
        let mask = get_num_mask_from_bitwidth(x as uintmax_t);
        Ok(Unsigned::Intmax(r & mask))
      },
      | LengthModifier::BitFast(x) => {
        let r: uintmax_t = if c_uint::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_uint>() as uintmax_t }
        } else if c_ulong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_ulong>() as uintmax_t }
        } else if c_ulonglong::BITS <= x as u32 {
          unsafe { va.next_arg::<crate::types::c_ulonglong>() as uintmax_t }
        } else {
          unsafe { va.next_arg::<crate::types::uintmax_t>() }
        };
        let mask = get_num_mask_from_bitwidth(x as uintmax_t);
        Ok(Unsigned::Intmax(r & mask))
      },
      | _ => Err(FormatError::InvalidArg)
    }
  }

  #[inline]
  pub unsafe fn parse_cchar(
    self,
    va: &mut ExtVaList
  ) -> Result<CChar, FormatError> {
    match self {
      | LengthModifier::Long => Ok(CChar::Wide(unsafe { va.next_arg() })),
      | _ => Ok(CChar::Narrow(unsafe {
        va.next_arg::<<crate::types::c_char as CharToInt>::IntType>()
      } as u8))
    }
  }

  #[inline]
  pub unsafe fn parse_cstr<'a>(
    self,
    va: &mut ExtVaList
  ) -> Result<CString<'a>, FormatError> {
    match self {
      | LengthModifier::Long => {
        let ptr: *const wchar_t = unsafe { va.next_arg() };
        if ptr.is_null() {
          return Ok(CString::Wide(NULL_STR_WIDE));
        }
        let slice = unsafe {
          slice::from_raw_parts(ptr as *const u32, wchar::rs_wcslen(ptr))
        };
        Ok(CString::Wide(slice))
      },
      | _ => {
        let ptr: *const c_char = unsafe { va.next_arg() };
        if ptr.is_null() {
          return Ok(CString::Narrow(b"(null)"));
        }
        let slice = unsafe {
          slice::from_raw_parts(ptr as *const u8, string::rs_strlen(ptr))
        };
        Ok(CString::Narrow(slice))
      }
    }
  }
}

#[inline]
pub fn parse_length_modifier<'a, T: Copy + Into<CharToAscii>>(
  fmt: &[T],
  index: &mut usize,
  ctype: &CtypeObject<'a>
) -> LengthModifier {
  let one = get_char_with_index(fmt, *index);
  let two = get_char_with_index(fmt, *index + 1);
  let mut lm = match (one, two) {
    | (Some('h'), Some('h')) => {
      *index += 2;
      LengthModifier::Byte
    },
    | (Some('l'), Some('l')) => {
      *index += 2;
      LengthModifier::LongLong
    },
    | (Some('h'), _) => {
      *index += 1;
      LengthModifier::Short
    },
    | (Some('l'), _) => {
      *index += 1;
      LengthModifier::Long
    },
    | (Some('j'), _) => {
      *index += 1;
      LengthModifier::Intmax
    },
    | (Some('z'), _) => {
      *index += 1;
      LengthModifier::Size
    },
    | (Some('t'), _) => {
      *index += 1;
      LengthModifier::Ptrdiff
    },
    | (Some('L'), _) => {
      *index += 1;
      LengthModifier::LongFloat
    },
    | _ => LengthModifier::Int
  };
  if get_char_with_index(fmt, *index) == Some('w') {
    let is_fast = get_char_with_index(fmt, *index + 1) == Some('f');
    *index += if is_fast { 2 } else { 1 };
    if let Some(ch) = get_char_with_index(fmt, *index) &&
      (ctype.casemap.isdigit)(ch as u32)
    {
      let result = strtoint::<T, usize>(&fmt[*index..], 10, ctype);
      *index += result.len;
      let width = core::cmp::max(0, result.value);
      lm = if is_fast {
        LengthModifier::BitFast(width)
      } else {
        LengthModifier::Bit(width)
      };
    }
  }
  lm
}
