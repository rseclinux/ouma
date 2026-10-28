use {
  super::{CastToDouble, Emitter, PrintfArgument, float_format},
  crate::{
    size_t,
    std::stdlib,
    support::{
      format::{error::FormatError, get_number},
      locale::{ctype::CtypeObject, numeric::NumericObject},
      string::conversion::ftoa
    },
    types::c_char
  },
  core::slice
};

#[derive(Debug)]
pub struct StrFromEmitter<'a> {
  buffer: &'a mut [u8],
  written: usize,
  ctype: &'a CtypeObject<'a>
}

impl<'a> StrFromEmitter<'a> {
  #[inline]
  pub const fn new(
    buf: *mut c_char,
    sz: size_t,
    ctype: &'a CtypeObject<'a>
  ) -> Self {
    let buffer = if sz == 0 {
      &mut []
    } else {
      unsafe { slice::from_raw_parts_mut(buf as *mut u8, sz) }
    };
    Self { buffer, written: 0, ctype }
  }

  #[inline]
  pub fn terminate(&mut self) {
    if !self.buffer.is_empty() {
      let i = self.written.min(self.buffer.len() - 1);
      self.buffer[i] = b'\0';
    }
  }

  #[inline]
  pub fn emit_u8(
    &mut self,
    ch: u8
  ) -> Result<(), FormatError> {
    if self.written + 1 < self.buffer.len() {
      self.buffer[self.written] = ch;
    }
    self.written += 1;
    Ok(())
  }

  #[inline]
  pub fn emit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError> {
    let mut buf = [0u8; stdlib::constants::MB_LEN_MAX];
    let ret = (self.ctype.converter.c32tomb)(&mut buf, ch);
    if ret < 0 {
      return Err(FormatError::InvalidSequence);
    }
    for i in 0..(ret as usize) {
      self.emit_u8(buf[i])?;
    }
    Ok(())
  }
}

impl<'a> Emitter for StrFromEmitter<'a> {
  type FormatChar = u8;

  #[inline]
  fn get_written(&self) -> usize {
    self.written
  }

  #[inline]
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u8(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_ascii_slice(
    &mut self,
    s: &[core::ascii::Char]
  ) -> Result<(), FormatError> {
    for i in s {
      let c = *i;
      self.emit_u8(c.to_u8())?;
    }
    Ok(())
  }

  #[inline]
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError> {
    for i in s {
      self.emit_u32(*i)?;
    }
    Ok(())
  }

  #[inline]
  fn emit_format_char(
    &mut self,
    c: Self::FormatChar
  ) -> Result<(), FormatError> {
    self.emit_u8(c)
  }

  #[inline]
  fn get_unicode_char_len(c: char) -> usize {
    c.len_utf8()
  }
}

#[inline]
pub fn strfromfloat<T: ftoa::DragonFloat + CastToDouble>(
  emitter: &mut StrFromEmitter,
  format: &[u8],
  value: T,
  ctype: &CtypeObject,
  numeric: &NumericObject
) -> Result<(), FormatError>
where
  T::Bn: ftoa::DragonInt + bnum::cast::CastFrom<T::StorageType>,
  u8: bnum::cast::CastFrom<T::Bn> + bnum::cast::CastFrom<T::StorageType> {
  let mut arg = PrintfArgument::default();
  let mut index = 0usize;

  if format.get(index) != Some(&b'%') {
    return Err(FormatError::InvalidArg);
  }
  index += 1;

  if format.get(index) == Some(&b'.') {
    index += 1;
    let num = get_number(format, &mut index, &ctype);
    arg.precision = Some(num as u32);
  }

  let Some(&spec) = format.get(index) else {
    return Err(FormatError::InvalidArg);
  };
  arg.specifier = spec.into();
  if !matches!(arg.specifier, 'a' | 'A' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
    return Err(FormatError::InvalidArg);
  }

  if index + 1 != format.len() {
    return Err(FormatError::InvalidArg);
  }

  if !value.is_finite() {
    return float_format::format_non_finite(emitter, value, &arg, ctype);
  }

  let arg_double = value.cast_to_double();

  let e = value.get_explicit_exponent();
  let use_ryu = (f64::MIN_EXP - 1..=f64::MAX_EXP - 1).contains(&e);

  match arg.specifier {
    | 'a' | 'A' => float_format::format_float_hexadecimal_typed(
      emitter, value, &arg, ctype, numeric
    ),
    | 'f' | 'F' => {
      if use_ryu {
        float_format::format_float_ryu(
          emitter,
          arg_double,
          float_format::FloatConv::Fixed,
          &arg,
          ctype,
          numeric
        )
      } else {
        float_format::format_float_dragon(
          emitter,
          value,
          float_format::FloatConv::Fixed,
          &arg,
          ctype,
          numeric
        )
      }
    },
    | 'g' | 'G' => {
      if use_ryu {
        float_format::format_float_ryu(
          emitter,
          arg_double,
          float_format::FloatConv::Automatic,
          &arg,
          ctype,
          numeric
        )
      } else {
        float_format::format_float_dragon(
          emitter,
          value,
          float_format::FloatConv::Automatic,
          &arg,
          ctype,
          numeric
        )
      }
    },
    | 'e' | 'E' => {
      if use_ryu {
        float_format::format_float_ryu(
          emitter,
          arg_double,
          float_format::FloatConv::Scientific,
          &arg,
          ctype,
          numeric
        )
      } else {
        float_format::format_float_dragon(
          emitter,
          value,
          float_format::FloatConv::Scientific,
          &arg,
          ctype,
          numeric
        )
      }
    },
    | _ => Err(FormatError::InvalidArg)
  }
}
