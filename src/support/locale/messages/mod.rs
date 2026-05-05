use {
  super::{LocaleObject, is_posix_locale},
  crate::{allocation::borrow::ToOwned, c_int, support::locale::errno},
  allocation::borrow::Cow,
  core::ffi
};

mod american_english;
mod belarusian_cyrillic;
mod belarusian_latin;
mod brazilian_portugese;
mod british_english;
mod cantonese_hans;
mod cantonese_hant;
mod catalan;
mod chinese_hans;
mod chinese_hant;
mod croatian;
mod czech;
mod danish;
mod dutch;
mod estonian;
mod european_portugese;
mod finnish;
mod french;
mod galician;
mod german;
mod greek;
mod hakka;
mod hebrew;
mod hokkien_hans;
mod hokkien_hant;
mod hungarian;
mod italian;
mod japanese;
mod korean;
mod limburgish;
mod maltese;
mod mandarin_china_singapore;
mod mandarin_taiwan;
mod manx;
mod norwegian;
mod occitan;
mod polish;
mod romansh;
mod russian;
mod serbian_cyrillic;
mod serbian_latin;
mod spanish;
mod swedish;
mod ukrainian;
mod vietnamese;
mod walloon;
mod walser;
mod wuu;

#[derive(Debug, Clone)]
pub struct MessagesObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub strerror: [&'a str; 134],
  pub strsignal: [&'a str; 32],
  pub regerror: [&'a str; 14],
  pub hstrerror: [&'a str; 5],
  pub gai_strerror: [&'a str; 15],
  pub misc_messages: [&'a str; 3],
  pub yesexpr: Cow<'a, str>,
  pub noexpr: Cow<'a, str>
}

impl<'a> LocaleObject for MessagesObject<'a> {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str().map_err(|_| errno::ENOENT)?;

    if is_posix_locale(name) {
      return Ok(self.set_to_posix(locale));
    }

    // Special case 1: English
    if name.starts_with("en") {
      if name.contains("US") || name.contains("CA") {
        self.misc_messages = american_english::MISC_MESSAGES;
        self.strerror = american_english::STRERROR;
        self.strsignal = american_english::STRSIGNAL;
        self.regerror = american_english::REGERROR;
        self.hstrerror = american_english::HSTRERROR;
        self.gai_strerror = american_english::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(american_english::NOEXPR);
        self.yesexpr = Cow::Borrowed(american_english::YESEXPR);
      } else {
        self.misc_messages = british_english::MISC_MESSAGES;
        self.strerror = british_english::STRERROR;
        self.strsignal = british_english::STRSIGNAL;
        self.regerror = british_english::REGERROR;
        self.hstrerror = british_english::HSTRERROR;
        self.gai_strerror = british_english::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(british_english::NOEXPR);
        self.yesexpr = Cow::Borrowed(british_english::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 2: Chinese
    if name.starts_with("zh") {
      if name.contains("CN") || name.contains("SG") {
        self.misc_messages = chinese_hans::MISC_MESSAGES;
        self.strerror = chinese_hans::STRERROR;
        self.strsignal = chinese_hans::STRSIGNAL;
        self.regerror = chinese_hans::REGERROR;
        self.hstrerror = chinese_hans::HSTRERROR;
        self.gai_strerror = chinese_hans::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(chinese_hans::NOEXPR);
        self.yesexpr = Cow::Borrowed(chinese_hans::YESEXPR);
      } else {
        self.misc_messages = chinese_hant::MISC_MESSAGES;
        self.strerror = chinese_hant::STRERROR;
        self.strsignal = chinese_hant::STRSIGNAL;
        self.regerror = chinese_hant::REGERROR;
        self.hstrerror = chinese_hant::HSTRERROR;
        self.gai_strerror = chinese_hant::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(chinese_hant::NOEXPR);
        self.yesexpr = Cow::Borrowed(chinese_hant::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 3: Cantonese
    if name.starts_with("yue") {
      if name.contains("CN") {
        self.misc_messages = cantonese_hans::MISC_MESSAGES;
        self.strerror = cantonese_hans::STRERROR;
        self.strsignal = cantonese_hans::STRSIGNAL;
        self.regerror = cantonese_hans::REGERROR;
        self.hstrerror = cantonese_hans::HSTRERROR;
        self.gai_strerror = cantonese_hans::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(cantonese_hans::NOEXPR);
        self.yesexpr = Cow::Borrowed(cantonese_hans::YESEXPR);
      } else {
        self.misc_messages = cantonese_hant::MISC_MESSAGES;
        self.strerror = cantonese_hant::STRERROR;
        self.strsignal = cantonese_hant::STRSIGNAL;
        self.regerror = cantonese_hant::REGERROR;
        self.hstrerror = cantonese_hant::HSTRERROR;
        self.gai_strerror = cantonese_hant::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(cantonese_hant::NOEXPR);
        self.yesexpr = Cow::Borrowed(cantonese_hant::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 4: Hokkien
    if name.starts_with("nan") {
      if !(name.contains("CN") || name.contains("TW")) {
        return Err(errno::EINVAL);
      }

      if name.contains("CN") {
        self.misc_messages = hokkien_hans::MISC_MESSAGES;
        self.strerror = hokkien_hans::STRERROR;
        self.strsignal = hokkien_hans::STRSIGNAL;
        self.regerror = hokkien_hans::REGERROR;
        self.hstrerror = hokkien_hans::HSTRERROR;
        self.gai_strerror = hokkien_hans::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(hokkien_hans::NOEXPR);
        self.yesexpr = Cow::Borrowed(hokkien_hans::YESEXPR);
      } else {
        self.misc_messages = hokkien_hant::MISC_MESSAGES;
        self.strerror = hokkien_hant::STRERROR;
        self.strsignal = hokkien_hant::STRSIGNAL;
        self.regerror = hokkien_hant::REGERROR;
        self.hstrerror = hokkien_hant::HSTRERROR;
        self.gai_strerror = hokkien_hant::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(hokkien_hant::NOEXPR);
        self.yesexpr = Cow::Borrowed(hokkien_hant::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 5: Mandarin
    if name.starts_with("cmn") {
      if !(name.contains("CN") || name.contains("SG") || name.contains("TW")) {
        return Err(errno::EINVAL);
      }

      if name.contains("CN") || name.contains("SG") {
        self.misc_messages = mandarin_china_singapore::MISC_MESSAGES;
        self.strerror = mandarin_china_singapore::STRERROR;
        self.strsignal = mandarin_china_singapore::STRSIGNAL;
        self.regerror = mandarin_china_singapore::REGERROR;
        self.hstrerror = mandarin_china_singapore::HSTRERROR;
        self.gai_strerror = mandarin_china_singapore::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(mandarin_china_singapore::NOEXPR);
        self.yesexpr = Cow::Borrowed(mandarin_china_singapore::YESEXPR);
      } else {
        self.misc_messages = mandarin_taiwan::MISC_MESSAGES;
        self.strerror = mandarin_taiwan::STRERROR;
        self.strsignal = mandarin_taiwan::STRSIGNAL;
        self.regerror = mandarin_taiwan::REGERROR;
        self.hstrerror = mandarin_taiwan::HSTRERROR;
        self.gai_strerror = mandarin_taiwan::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(mandarin_taiwan::NOEXPR);
        self.yesexpr = Cow::Borrowed(mandarin_taiwan::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 6: Portugese
    if name.starts_with("pt") {
      if name.contains("BR") {
        self.misc_messages = brazilian_portugese::MISC_MESSAGES;
        self.strerror = brazilian_portugese::STRERROR;
        self.strsignal = brazilian_portugese::STRSIGNAL;
        self.regerror = brazilian_portugese::REGERROR;
        self.hstrerror = brazilian_portugese::HSTRERROR;
        self.gai_strerror = brazilian_portugese::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(brazilian_portugese::NOEXPR);
        self.yesexpr = Cow::Borrowed(brazilian_portugese::YESEXPR);
      } else {
        self.misc_messages = european_portugese::MISC_MESSAGES;
        self.strerror = european_portugese::STRERROR;
        self.strsignal = european_portugese::STRSIGNAL;
        self.regerror = european_portugese::REGERROR;
        self.hstrerror = european_portugese::HSTRERROR;
        self.gai_strerror = european_portugese::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(european_portugese::NOEXPR);
        self.yesexpr = Cow::Borrowed(european_portugese::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 7: Serbian
    if name.starts_with("sr") {
      if name.contains("@latin") {
        self.misc_messages = serbian_latin::MISC_MESSAGES;
        self.strerror = serbian_latin::STRERROR;
        self.strsignal = serbian_latin::STRSIGNAL;
        self.regerror = serbian_latin::REGERROR;
        self.hstrerror = serbian_latin::HSTRERROR;
        self.gai_strerror = serbian_latin::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(serbian_latin::NOEXPR);
        self.yesexpr = Cow::Borrowed(serbian_latin::YESEXPR);
      } else {
        self.misc_messages = serbian_cyrillic::MISC_MESSAGES;
        self.strerror = serbian_cyrillic::STRERROR;
        self.strsignal = serbian_cyrillic::STRSIGNAL;
        self.regerror = serbian_cyrillic::REGERROR;
        self.hstrerror = serbian_cyrillic::HSTRERROR;
        self.gai_strerror = serbian_cyrillic::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(serbian_cyrillic::NOEXPR);
        self.yesexpr = Cow::Borrowed(serbian_cyrillic::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 8: Belarusian
    if name.starts_with("be") {
      if name.contains("@latin") {
        self.misc_messages = belarusian_latin::MISC_MESSAGES;
        self.strerror = belarusian_latin::STRERROR;
        self.strsignal = belarusian_latin::STRSIGNAL;
        self.regerror = belarusian_latin::REGERROR;
        self.hstrerror = belarusian_latin::HSTRERROR;
        self.gai_strerror = belarusian_latin::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(belarusian_latin::NOEXPR);
        self.yesexpr = Cow::Borrowed(belarusian_latin::YESEXPR);
      } else {
        self.misc_messages = belarusian_cyrillic::MISC_MESSAGES;
        self.strerror = belarusian_cyrillic::STRERROR;
        self.strsignal = belarusian_cyrillic::STRSIGNAL;
        self.regerror = belarusian_cyrillic::REGERROR;
        self.hstrerror = belarusian_cyrillic::HSTRERROR;
        self.gai_strerror = belarusian_cyrillic::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(belarusian_cyrillic::NOEXPR);
        self.yesexpr = Cow::Borrowed(belarusian_cyrillic::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // TODO: Parse languages
    Ok(self.set_to_posix(locale))
  }

  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr {
    *self = DEFAULT_MESSAGES;

    self.name = Cow::Owned(locale.to_owned());
    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> MessagesObject<'a> {
  fn set_messages(
    &mut self,
    misc: &[&'a str; 3],
    strerror: &[&'a str; 134],
    strsignal: &[&'a str; 32],
    regerror: &[&'a str; 14],
    hstrerror: &[&'a str; 5],
    gai_strerror: &[&'a str; 15],
    noexpr: &'a str,
    yesexpr: &'a str
  ) {
    self.misc_messages = *misc;
    self.strerror = *strerror;
    self.strsignal = *strsignal;
    self.regerror = *regerror;
    self.hstrerror = *hstrerror;
    self.gai_strerror = *gai_strerror;
    self.noexpr = Cow::Borrowed(noexpr);
    self.yesexpr = Cow::Borrowed(yesexpr);
  }
}

impl<'a> Default for MessagesObject<'a> {
  fn default() -> Self {
    DEFAULT_MESSAGES
  }
}

pub const DEFAULT_MESSAGES: MessagesObject = MessagesObject {
  name: Cow::Borrowed(c"C"),
  misc_messages: american_english::MISC_MESSAGES,
  strerror: american_english::STRERROR,
  strsignal: american_english::STRSIGNAL,
  regerror: american_english::REGERROR,
  hstrerror: american_english::HSTRERROR,
  gai_strerror: american_english::GAI_STRERROR,
  noexpr: Cow::Borrowed(american_english::NOEXPR),
  yesexpr: Cow::Borrowed(american_english::YESEXPR)
};
