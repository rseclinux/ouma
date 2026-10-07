use {
  super::{Consumer, ScanfArgument},
  crate::{
    support::{
      format::{error::FormatError, length::LengthModifier},
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::{StrToError, strtofloat},
      traits::float::Float
    },
    types::{
      c_double,
      c_float,
      c_int,
      c_long,
      c_longdouble,
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
      uintmax_t
    }
  },
  core::{mem::size_of, slice}
};

#[inline]
fn fast_bits(sz: usize) -> usize {
  match sz {
    | 8 => size_of::<int_fast8_t>() * 8,
    | 16 => size_of::<int_fast16_t>() * 8,
    | 32 => size_of::<int_fast32_t>() * 8,
    | 64 => size_of::<int_fast64_t>() * 8,
    | _ => sz
  }
}

#[inline]
pub fn write_signed_integer(
  value: intmax_t,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  unsafe {
    match arg.modifier {
      | LengthModifier::Byte => *(ptr as *mut c_schar) = value as c_schar,
      | LengthModifier::Short => *(ptr as *mut c_short) = value as c_short,
      | LengthModifier::Int => *(ptr as *mut c_int) = value as c_int,
      | LengthModifier::Long => *(ptr as *mut c_long) = value as c_long,
      | LengthModifier::LongLong => {
        *(ptr as *mut c_longlong) = value as c_longlong
      },
      | LengthModifier::Size => *(ptr as *mut ssize_t) = value as ssize_t,
      | LengthModifier::Intmax => *(ptr as *mut intmax_t) = value,
      | LengthModifier::Ptrdiff => {
        *(ptr as *mut ptrdiff_t) = value as ptrdiff_t
      },
      | LengthModifier::Bit(sz) | LengthModifier::BitFast(sz) => {
        let sz = if matches!(arg.modifier, LengthModifier::BitFast(_)) {
          fast_bits(sz)
        } else {
          sz
        };
        let mask = if sz >= intmax_t::BITS as usize {
          intmax_t::MAX
        } else {
          (1 as intmax_t).wrapping_shl(sz as u32) - 1
        };
        let val = value & mask;
        if sz <= 8 {
          *(ptr as *mut i8) = val as i8;
        } else if sz <= 16 {
          *(ptr as *mut i16) = val as i16;
        } else if sz <= 32 {
          *(ptr as *mut i32) = val as i32;
        } else if sz <= 64 {
          *(ptr as *mut i64) = val as i64;
        } else {
          return Err(FormatError::BadMatch);
        }
      },
      | LengthModifier::LongFloat => return Err(FormatError::BadMatch)
    }
  }
  Ok(())
}

#[inline]
pub fn write_unsigned_integer(
  value: uintmax_t,
  ptr: *mut u8,
  arg: &ScanfArgument
) -> Result<(), FormatError> {
  unsafe {
    match arg.modifier {
      | LengthModifier::Byte => *ptr = value as c_uchar,
      | LengthModifier::Short => *(ptr as *mut c_ushort) = value as c_ushort,
      | LengthModifier::Int => *(ptr as *mut c_uint) = value as c_uint,
      | LengthModifier::Long => *(ptr as *mut c_ulong) = value as c_ulong,
      | LengthModifier::LongLong => {
        *(ptr as *mut c_ulonglong) = value as c_ulonglong
      },
      | LengthModifier::Size => *(ptr as *mut size_t) = value as size_t,
      | LengthModifier::Intmax => *(ptr as *mut uintmax_t) = value,
      | LengthModifier::Ptrdiff => *(ptr as *mut usize) = value as usize,
      | LengthModifier::Bit(sz) | LengthModifier::BitFast(sz) => {
        let sz = if matches!(arg.modifier, LengthModifier::BitFast(_)) {
          fast_bits(sz)
        } else {
          sz
        };
        let mask = if sz >= uintmax_t::BITS as usize {
          uintmax_t::MAX
        } else {
          (1 as uintmax_t).wrapping_shl(sz as u32) - 1
        };
        let val = value & mask;
        if sz <= 8 {
          *(ptr as *mut u8) = val as u8;
        } else if sz <= 16 {
          *(ptr as *mut u16) = val as u16;
        } else if sz <= 32 {
          *(ptr as *mut u32) = val as u32;
        } else if sz <= 64 {
          *(ptr as *mut u64) = val as u64;
        } else {
          return Err(FormatError::BadMatch);
        }
      },
      | LengthModifier::LongFloat => return Err(FormatError::BadMatch)
    }
  }
  Ok(())
}

#[inline]
pub fn write_floating_point_value<C: Consumer>(
  consumer: &mut C,
  s: &[u32],
  ptr: *mut u8,
  arg: &ScanfArgument,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError> {
  let arg = arg.clone();
  match arg.modifier {
    | LengthModifier::LongFloat => {
      let result: strtofloat::StrToFloatResult<c_longdouble> =
        strtofloat::strtofloat(s, ctype, numeric);
      if result.error == Some(StrToError::InvalidNumber) {
        return Err(FormatError::BadMatch);
      } else if !arg.suppress {
        consumer.increase_converted();
        let out = unsafe {
          slice::from_raw_parts_mut(ptr, c_longdouble::SIZE_IN_BYTES)
        };
        let bytes = result.value.to_ne_bytes();
        out.copy_from_slice(&bytes);
        Ok(())
      } else {
        Ok(())
      }
    },
    | LengthModifier::Long => {
      let result: strtofloat::StrToFloatResult<c_double> =
        strtofloat::strtofloat(s, ctype, numeric);
      if result.error == Some(StrToError::InvalidNumber) {
        return Err(FormatError::BadMatch);
      } else if !arg.suppress {
        consumer.increase_converted();
        let out =
          unsafe { slice::from_raw_parts_mut(ptr, c_double::SIZE_IN_BYTES) };
        let bytes = result.value.to_ne_bytes();
        out.copy_from_slice(&bytes);
        Ok(())
      } else {
        Ok(())
      }
    },
    | _ => {
      let result: strtofloat::StrToFloatResult<c_float> =
        strtofloat::strtofloat(s, ctype, numeric);
      if result.error == Some(StrToError::InvalidNumber) {
        return Err(FormatError::BadMatch);
      } else if !arg.suppress {
        consumer.increase_converted();
        let out =
          unsafe { slice::from_raw_parts_mut(ptr, c_float::SIZE_IN_BYTES) };
        let bytes = result.value.to_ne_bytes();
        out.copy_from_slice(&bytes);
        Ok(())
      } else {
        Ok(())
      }
    }
  }
}
