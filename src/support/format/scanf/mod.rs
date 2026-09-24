use {
  super::{
    error::FormatError,
    get_number,
    get_numbered_arg,
    length::{LengthModifier, parse_length_modifier}
  },
  crate::{
    std::wchar::UnicodeBitset,
    support::{
      locale::{self},
      traits::char::{
        CharToAscii,
        CharToUnicode,
        get_ascii_char,
        get_ascii_char_with_index
      }
    }
  },
  core::{ffi::VaList, ptr},
  num_traits::ConstZero
};

pub mod integer_format;
pub mod read_format;
pub mod utils;

#[derive(Default, Debug, Clone, Copy)]
pub struct ScanfArgument {
  pub suppress: bool,
  pub allocate: bool,
  pub width: usize,
  pub modifier: LengthModifier,
  pub scan_set: Option<UnicodeBitset>,
  pub specifier: char
}

pub trait Consumer {
  type FormatChar: Into<CharToAscii>
    + CharToUnicode
    + PartialEq
    + Copy
    + num_traits::ConstZero;

  fn get_read(&self) -> usize;
  fn get_converted(&self) -> usize;
  fn increase_converted(&mut self);

  fn consume(&mut self) -> Result<Self::FormatChar, FormatError>;
  fn vomit(
    &mut self,
    ch: Self::FormatChar
  ) -> Result<(), FormatError>;

  fn consume_u32(&mut self) -> Result<u32, FormatError>;
  fn vomit_u32(
    &mut self,
    ch: u32
  ) -> Result<(), FormatError>;
  fn consume_u8(&mut self) -> Result<u8, FormatError>;
  fn vomit_u8(
    &mut self,
    ch: u8
  ) -> Result<(), FormatError>;

  #[inline]
  fn consume_unicode_char(&mut self) -> Result<char, FormatError> {
    let ch = self.consume_u32()?;

    if let Some(c) = char::from_u32(ch) {
      Ok(c)
    } else {
      Err(FormatError::InvalidSequence)
    }
  }

  #[inline]
  fn vomit_unicode_char(
    &mut self,
    ch: char
  ) -> Result<(), FormatError> {
    self.vomit_u32(ch.into())
  }
}

#[inline]
pub fn scanf_inner<T: Consumer>(
  locale: &locale::Locale,
  consumer: &mut T,
  format: &[T::FormatChar],
  vlist: &mut VaList
) -> Result<(), FormatError> {
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();
  let _numeric = locale::get_slot(&locale.numeric).unwrap_or_default();
  let numargs = vlist.clone();

  let mut index = 0usize;

  while index < format.len() {
    let ch = format.get(index).copied().unwrap_or(T::FormatChar::ZERO);
    index += 1;

    if get_ascii_char(ch).to_char() == '%' {
      // Escaped percent
      if get_ascii_char_with_index(format, index) == Some('%') {
        index += 1;

        let c_cur = consumer.consume()?;
        if get_ascii_char(c_cur).to_char() != '%' {
          consumer.vomit(c_cur)?;
          return Err(FormatError::BadMatch);
        }

        continue;
      }

      let mut numarg = get_numbered_arg(format, &mut index, &ctype);

      let suppress = if get_ascii_char_with_index(format, index) == Some('*') {
        index += 1;
        true
      } else {
        false
      };

      let width = get_number(format, &mut index, &ctype);

      let allocate = if get_ascii_char_with_index(format, index) == Some('m') {
        index += 1;
        true
      } else {
        false
      };

      let lm = parse_length_modifier(format, &mut index, &ctype);

      let specifier: char =
        get_ascii_char_with_index(format, index).unwrap_or('\0');
      index += 1;

      let mut scan_set: Option<UnicodeBitset> = None;

      if specifier == '[' {
        let mut set = UnicodeBitset::new();

        let invert = if get_ascii_char_with_index(format, index) == Some('^') {
          index += 1;
          true
        } else {
          false
        };

        let start = index;
        let mut leading_bracket = false;

        if let Some(c) = get_ascii_char_with_index(format, index) &&
          c == ']'
        {
          if set.try_insert(']' as usize).is_err() {
            return Err(FormatError::BadMatch);
          }

          index += 1;
          leading_bracket = true;
        }

        while let Some(ch) = CharToUnicode::get_unicode_char(format, index) &&
          ch != ']'
        {
          let after_leading_bracket = leading_bracket && index == start + 1;

          if ch == '-' &&
            index != start &&
            !after_leading_bracket &&
            get_ascii_char_with_index(format, index + 1) != Some(']') &&
            get_ascii_char_with_index(format, index + 1).is_some()
          {
            let from = match get_ascii_char_with_index(format, index - 1) {
              | Some(c) => c as usize,
              | None => return Err(FormatError::BadMatch)
            };

            let to = match get_ascii_char_with_index(format, index + 1) {
              | Some(c) => c as usize,
              | None => return Err(FormatError::BadMatch)
            };

            if from <= to {
              if to >= UnicodeBitset::capacity() {
                return Err(FormatError::BadMatch);
              }
              for c in from..=to {
                if set.try_insert(c as usize).is_err() {
                  return Err(FormatError::BadMatch);
                }
              }

              index += 2;
            } else {
              if set.try_insert('-' as usize).is_err() {
                return Err(FormatError::BadMatch);
              }

              index += 1;
            }
          } else {
            if set.try_insert(ch as usize).is_err() {
              return Err(FormatError::BadMatch);
            }

            index += 1;
          }
        }

        if invert {
          let mut inner = set.into_inner();

          for word in inner.as_mut() {
            *word = !*word;
          }

          set = UnicodeBitset::from(inner);
        }

        if get_ascii_char_with_index(format, index) == Some(']') {
          scan_set = Some(set);
          index += 1;
        } else {
          return Err(FormatError::BadMatch);
        }
      }

      let mut arg = ScanfArgument {
        suppress,
        allocate,
        width,
        modifier: lm,
        scan_set,
        specifier
      };

      let mut argument: *mut u8 = ptr::null_mut();
      if !arg.suppress {
        if numarg > 0 {
          let mut n = numargs.clone();
          while numarg > 0 {
            argument = unsafe { n.next_arg() };
            numarg -= 1;
          }
        } else {
          argument = unsafe { vlist.next_arg() };
        }
      }

      match arg.specifier {
        | 'a' | 'A' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G' => {
          todo!("float format")
        },
        | 'd' | 'i' | 'b' | 'B' | 'u' | 'o' | 'x' | 'X' => {
          integer_format::format_integer(consumer, argument, &arg, &ctype)?
        },
        | 'p' => {
          arg.modifier = LengthModifier::Ptrdiff;
          integer_format::format_integer(consumer, argument, &arg, &ctype)?
        },
        | 'n' => read_format::format_read(consumer, argument, &arg)?,
        | 'c' => todo!("char format"),
        | 'C' => {
          //arg.modifier = LengthModifier::Long;
          todo!("char format")
        },
        | 'S' => {
          //arg.modifier = LengthModifier::Long;
          todo!("string format")
        },
        | '[' | 's' => todo!("string format"),
        | _ => {
          return Err(FormatError::BadMatch);
        }
      }
    } else {
      if (ctype.casemap.isspace)(get_ascii_char(ch).into()) {
        loop {
          let c = consumer.consume_unicode_char()?;
          if !(ctype.casemap.isspace)(c.into()) {
            break;
          }
        }
        index += 1;
        continue;
      }

      let c_cur = consumer.consume()?;
      if c_cur != ch {
        consumer.vomit(c_cur)?;
        return Err(FormatError::BadMatch);
      }
    }
  }

  Ok(())
}
