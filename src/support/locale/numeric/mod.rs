use {
  super::{LocaleObject, is_posix_locale},
  crate::{c_int, std::errno},
  allocation::{
    borrow::{Cow, ToOwned},
    vec
  },
  core::ffi,
  icu_decimal::{DecimalFormatter, input::Decimal},
  icu_locale::Locale,
  writeable::Writeable
};

fn get_decimal_sep<'a>(s: &str) -> Option<Cow<'a, [u8]>> {
  let mut last = None;
  for (i, ch) in s.char_indices() {
    if !ch.is_ascii_digit() && !ch.is_whitespace() {
      last = Some(i);
    }
  }
  match last {
    | Some(i) => {
      let mut buffer: vec::Vec<u8> = vec::Vec::new();

      let sep = s[i..].chars().next()?;

      let mut b = [0; 4];
      let encoded = sep.encode_utf8(&mut b).as_bytes();

      buffer.extend_from_slice(encoded);
      buffer.push(b'\0');

      Some(Cow::Owned(buffer))
    },
    | None => None
  }
}

fn get_grouping_sep<'a>(s: &str) -> Option<Cow<'a, [u8]>> {
  let mut buffer: vec::Vec<u8> = vec::Vec::new();

  for ch in s.chars() {
    if !ch.is_ascii_digit() && !ch.is_whitespace() {
      let mut b = [0; 4];
      let encoded = ch.encode_utf8(&mut b).as_bytes();

      buffer.extend_from_slice(encoded);
      buffer.push(b'\0');

      return Some(Cow::Owned(buffer));
    }
  }

  None
}

fn get_posix_grouping<'a>(s: &str) -> Option<Cow<'a, [u8]>> {
  let mut buf: vec::Vec<u8> = vec::Vec::new();
  let mut cur = 0;
  for ch in s.chars().rev() {
    if ch.is_ascii_digit() {
      cur += 1;
    } else if cur > 0 {
      buf.push(cur);
      cur = 0;
    }
  }

  if cur > 0 {
    buf.push(cur);
  }
  if buf.len() < 2 {
    return None;
  }

  let primary: u8 = buf[0];
  let secondary: Option<u8> = buf.get(1).copied();

  let result = if let Some(s) = secondary {
    vec![primary, s, b'\0']
  } else {
    vec![primary, b'\0']
  };

  Some(Cow::Owned(result))
}

pub struct NumericObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub decimal_point: Cow<'a, [u8]>,
  pub thousands_sep: Cow<'a, [u8]>,
  pub grouping: Cow<'a, [u8]>
}

impl<'a> LocaleObject for NumericObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str();
    let name = match name {
      | Ok(s) => s,
      | Err(_) => return Err(errno::EINVAL)
    };

    if is_posix_locale(name) {
      return Ok(self.set_to_posix());
    }

    let mut parts = name.split('.');

    if let Some(lang) = parts.next() &&
      !lang.is_empty()
    {
      let icu_locale = Locale::try_from_str(&lang.replace("_", "-"));
      let icu_locale = match icu_locale {
        | Ok(icu_locale) => icu_locale,
        | Err(_) => return Err(errno::EINVAL)
      };

      let formatter =
        DecimalFormatter::try_new(icu_locale.into(), Default::default());
      let formatter = match formatter {
        | Ok(formatter) => formatter,
        | Err(_) => return Err(errno::EINVAL)
      };

      let mut frac = Decimal::from(1234);
      frac.multiply_pow10(-2);
      let s_frac = formatter.format(&frac);
      let s_frac = s_frac.write_to_string();

      // fallback to POSIX
      let decimal_point =
        get_decimal_sep(&s_frac).unwrap_or(Cow::Borrowed(&[b'.', b'\0']));

      let big = Decimal::from(1234567890123u128);
      let s_int = formatter.format(&big);
      let s_int = s_int.write_to_string();

      // fallback to POSIX
      let thousands_sep =
        get_grouping_sep(&s_int).unwrap_or(Cow::Borrowed(&[b'\0']));

      // fallback to POSIX
      let grouping =
        get_posix_grouping(&s_int).unwrap_or(Cow::Borrowed(&[b'\0']));

      self.name = Cow::Owned(locale.to_owned());
      self.decimal_point = decimal_point;
      self.thousands_sep = thousands_sep;
      self.grouping = grouping;

      return Ok(self.name.as_ref());
    }

    Err(errno::EINVAL)
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    self.name = Cow::Borrowed(c"C");

    self.decimal_point = Cow::Borrowed(&[b'.', b'\0']);
    self.thousands_sep = Cow::Borrowed(&[b'\0']);
    self.grouping = Cow::Borrowed(&[b'\0']);

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for NumericObject<'a> {
  fn default() -> Self {
    DEFAULT_NUMERIC
  }
}

pub const DEFAULT_NUMERIC: NumericObject = NumericObject {
  name: Cow::Borrowed(c"C"),
  decimal_point: Cow::Borrowed(&[b'.', b'\0']),
  thousands_sep: Cow::Borrowed(&[b'\0']),
  grouping: Cow::Borrowed(&[b'\0'])
};
