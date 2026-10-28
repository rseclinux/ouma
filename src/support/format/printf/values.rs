use {
  crate::{
    c_longdouble,
    std::wchar,
    support::{ffi::va_list::ExtVaList, format::length::LengthModifier},
    types::{
      c_char,
      c_double,
      c_int,
      c_long,
      c_longlong,
      c_schar,
      c_short,
      c_uchar,
      c_uint,
      c_ulong,
      c_ulonglong,
      c_ushort,
      int_fast8_t,
      int_fast16_t,
      int_fast32_t,
      int_fast64_t,
      intmax_t,
      ptrdiff_t,
      size_t,
      ssize_t,
      uint_fast8_t,
      uint_fast16_t,
      uint_fast32_t,
      uint_fast64_t,
      uintmax_t,
      uintptr_t,
      wchar_t
    }
  },
  core::{ffi::CStr, mem, slice}
};

#[derive(Debug, Clone, Copy)]
pub enum Signed {
  Byte(c_schar),
  Short(c_short),
  Int(c_int),
  Long(c_long),
  LongLong(c_longlong),
  Intmax(intmax_t),
  Size(ssize_t),
  Pointer(ptrdiff_t)
}

impl Signed {
  #[inline]
  pub unsafe fn from_raw(
    lm: LengthModifier,
    args: &mut ExtVaList
  ) -> Option<Self> {
    match lm {
      | LengthModifier::Byte => {
        Some(Signed::Byte(unsafe { args.next_arg::<c_int>() as c_schar }))
      },
      | LengthModifier::Short => {
        Some(Signed::Short(unsafe { args.next_arg::<c_int>() as c_short }))
      },
      | LengthModifier::Default => {
        Some(Signed::Int(unsafe { args.next_arg() }))
      },
      | LengthModifier::Long => Some(Signed::Long(unsafe { args.next_arg() })),
      | LengthModifier::LongLong => {
        Some(Signed::LongLong(unsafe { args.next_arg() }))
      },
      | LengthModifier::Intmax => {
        Some(Signed::LongLong(unsafe { args.next_arg() }))
      },
      | LengthModifier::Size => Some(Signed::Size(unsafe { args.next_arg() })),
      | LengthModifier::Ptrdiff => {
        Some(Signed::Pointer(unsafe { args.next_arg() }))
      },
      | LengthModifier::Bit(x) => {
        if x < 2 || x > 64 {
          return None;
        }
        let r: intmax_t = if x as u32 <= c_uint::BITS {
          unsafe { args.next_arg::<c_int>() as intmax_t }
        } else if x as u32 <= c_long::BITS {
          unsafe { args.next_arg::<c_long>() as intmax_t }
        } else {
          unsafe { args.next_arg::<c_longlong>() as intmax_t }
        };
        let shift = 64 - x as u32;
        Some(Self::Intmax((r << shift) >> shift))
      },
      | LengthModifier::BitFast(x) => {
        let bits: u32 = match x {
          | 8 => (mem::size_of::<int_fast8_t>() * 8) as u32,
          | 16 => (mem::size_of::<int_fast16_t>() * 8) as u32,
          | 32 => (mem::size_of::<int_fast32_t>() * 8) as u32,
          | 64 => (mem::size_of::<int_fast64_t>() * 8) as u32,
          | _ => return None
        };
        let r: intmax_t = if bits <= c_uint::BITS {
          unsafe { args.next_arg::<c_int>() as intmax_t }
        } else {
          unsafe { args.next_arg::<c_long>() as intmax_t }
        };
        Some(Self::Intmax(r))
      },
      | _ => None
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Unsigned {
  Byte(c_uchar),
  Short(c_ushort),
  Int(c_uint),
  Long(c_ulong),
  LongLong(c_ulonglong),
  Intmax(uintmax_t),
  Size(size_t),
  Pointer(usize)
}

impl Unsigned {
  #[inline]
  pub unsafe fn from_raw(
    lm: LengthModifier,
    args: &mut ExtVaList
  ) -> Option<Self> {
    match lm {
      | LengthModifier::Byte => Some(Unsigned::Byte(unsafe {
        args.next_arg::<c_uint>() as c_uchar & c_uchar::MAX
      })),
      | LengthModifier::Short => Some(Unsigned::Short(unsafe {
        args.next_arg::<c_uint>() as c_ushort & c_ushort::MAX
      })),
      | LengthModifier::Default => {
        Some(Unsigned::Int(unsafe { args.next_arg::<c_uint>() & c_uint::MAX }))
      },
      | LengthModifier::Long => Some(Unsigned::Long(unsafe {
        args.next_arg::<c_ulong>() & c_ulong::MAX
      })),
      | LengthModifier::LongLong => Some(Unsigned::LongLong(unsafe {
        args.next_arg::<c_ulonglong>() & c_ulonglong::MAX
      })),
      | LengthModifier::Intmax => Some(Unsigned::LongLong(unsafe {
        args.next_arg::<uintmax_t>() & uintmax_t::MAX
      })),
      | LengthModifier::Size => {
        Some(Unsigned::Size(unsafe { args.next_arg::<size_t>() & size_t::MAX }))
      },
      | LengthModifier::Ptrdiff => Some(Unsigned::Pointer(unsafe {
        args.next_arg::<usize>() & usize::MAX
      })),
      | LengthModifier::Bit(x) => {
        if x < 2 || x > 64 {
          return None;
        }
        let r: uintmax_t = if x as u32 <= c_uint::BITS {
          unsafe { args.next_arg::<c_uint>() as uintmax_t }
        } else if x as u32 <= c_ulong::BITS {
          unsafe { args.next_arg::<c_ulong>() as uintmax_t }
        } else {
          unsafe { args.next_arg::<c_ulonglong>() as uintmax_t }
        };
        let mask: uintmax_t =
          if x == 64 { !0 } else { ((1 as uintmax_t) << x) - 1 };
        Some(Self::Intmax(r & mask))
      },
      | LengthModifier::BitFast(x) => {
        let bits: u32 = match x {
          | 8 => (mem::size_of::<uint_fast8_t>() * 8) as u32,
          | 16 => (mem::size_of::<uint_fast16_t>() * 8) as u32,
          | 32 => (mem::size_of::<uint_fast32_t>() * 8) as u32,
          | 64 => (mem::size_of::<uint_fast64_t>() * 8) as u32,
          | _ => return None
        };
        let r: uintmax_t = if bits <= c_uint::BITS {
          unsafe { args.next_arg::<c_uint>() as uintmax_t }
        } else {
          unsafe { args.next_arg::<c_ulong>() as uintmax_t }
        };
        Some(Self::Intmax(r))
      },
      | _ => None
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum FloatValue {
  Double(c_double),
  LongDouble(c_longdouble)
}

impl FloatValue {
  #[inline]
  pub unsafe fn from_raw(
    lm: LengthModifier,
    args: &mut ExtVaList
  ) -> Option<Self> {
    if lm == LengthModifier::LongFloat {
      return Some(FloatValue::LongDouble(unsafe {
        args.get_long_double_value()
      }));
    }
    if lm == LengthModifier::Default || lm == LengthModifier::Long {
      return Some(FloatValue::Double(unsafe { args.next_arg() }));
    }
    None
  }
}

#[derive(Debug, Clone, Copy)]
pub enum CPointer<'a> {
  Null(&'a [u8]),
  NonNull(uintptr_t)
}

impl<'a> CPointer<'a> {
  #[inline]
  pub unsafe fn from_raw(args: &mut ExtVaList) -> Self {
    let p: *const () = unsafe { args.next_arg() };
    if p.is_null() {
      Self::Null(b"(nil)")
    } else {
      Self::NonNull(p as uintptr_t)
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum CChar {
  Narrow(u8),
  Wide(u32)
}

impl CChar {
  #[inline]
  pub unsafe fn from_raw(
    lm: LengthModifier,
    args: &mut ExtVaList
  ) -> Option<Self> {
    if lm == LengthModifier::Long {
      return Some(CChar::Wide(unsafe { args.next_arg() }));
    }
    if lm == LengthModifier::Default {
      return Some(CChar::Narrow(unsafe { args.next_arg::<u32>() as u8 }));
    }
    None
  }
}

#[derive(Debug, Clone, Copy)]
pub enum CString<'a> {
  Narrow(&'a [u8]),
  Wide(&'a [u32])
}

impl<'a> CString<'a> {
  #[inline]
  pub unsafe fn from_raw(
    lm: LengthModifier,
    args: &mut ExtVaList
  ) -> Option<Self> {
    if lm == LengthModifier::Long {
      let p: *const wchar_t = unsafe { args.next_arg() };
      if p.is_null() {
        return Some(CString::Wide(u32str_from_ascii!(b"(null)")));
      } else {
        let slen = wchar::rs_wcslen(p);
        return Some(CString::Wide(unsafe {
          slice::from_raw_parts(p as *const u32, slen)
        }));
      }
    }
    if lm == LengthModifier::Default {
      let p: *const c_char = unsafe { args.next_arg() };
      if p.is_null() {
        return Some(CString::Narrow(b"(null)"));
      } else {
        return Some(CString::Narrow(unsafe { CStr::from_ptr(p).to_bytes() }));
      }
    }
    None
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ArgumentValues<'a> {
  Signed(Signed),
  Unsigned(Unsigned),
  Float(FloatValue),
  Pointer(CPointer<'a>),
  Char(CChar),
  String(CString<'a>),
  NotSpecified
}
