use {
  super::{LocaleObject, is_posix_locale},
  crate::{allocation::borrow::ToOwned, c_int, support::locale::errno},
  allocation::{borrow::Cow, string::ToString},
  core::ffi
};

mod american_english;
mod arabic;
mod basque;
mod belarusian_cyrillic;
mod belarusian_latin;
mod brazilian_portugese;
mod breton;
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
mod icelandic;
mod irish;
mod italian;
mod japanese;
mod korean;
mod limburgish;
mod luxemburgish;
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
mod welsh;
mod wuu;

pub struct Messages<'a> {
  pub strerror: [&'a str; 134],
  pub strsignal: [&'a str; 32],
  pub regerror: [&'a str; 14],
  pub hstrerror: [&'a str; 5],
  pub gai_strerror: [&'a str; 15],
  pub misc_messages: [&'a str; 3],
  pub yesexpr: Cow<'a, str>,
  pub noexpr: Cow<'a, str>
}

struct AvailableMessages<'a> {
  pub name: &'a str,
  pub messages: Messages<'a>
}

const AVAILABLE_MESSAGES: [AvailableMessages; 39] = [
  AvailableMessages { name: "ar", messages: arabic::MESSAGES },
  AvailableMessages { name: "br", messages: breton::MESSAGES },
  AvailableMessages { name: "ca", messages: catalan::MESSAGES },
  AvailableMessages { name: "cs", messages: czech::MESSAGES },
  AvailableMessages { name: "cy", messages: welsh::MESSAGES },
  AvailableMessages { name: "da", messages: danish::MESSAGES },
  AvailableMessages { name: "de", messages: german::MESSAGES },
  AvailableMessages { name: "el", messages: greek::MESSAGES },
  AvailableMessages { name: "es", messages: spanish::MESSAGES },
  AvailableMessages { name: "et", messages: estonian::MESSAGES },
  AvailableMessages { name: "eu", messages: basque::MESSAGES },
  AvailableMessages { name: "fi", messages: finnish::MESSAGES },
  AvailableMessages { name: "fr", messages: french::MESSAGES },
  AvailableMessages { name: "gl", messages: galician::MESSAGES },
  AvailableMessages { name: "ga", messages: irish::MESSAGES },
  AvailableMessages { name: "gv", messages: manx::MESSAGES },
  AvailableMessages { name: "hak", messages: hakka::MESSAGES },
  AvailableMessages { name: "he", messages: hebrew::MESSAGES },
  AvailableMessages { name: "hr", messages: croatian::MESSAGES },
  AvailableMessages { name: "hu", messages: hungarian::MESSAGES },
  AvailableMessages { name: "is", messages: icelandic::MESSAGES },
  AvailableMessages { name: "it", messages: italian::MESSAGES },
  AvailableMessages { name: "ja", messages: japanese::MESSAGES },
  AvailableMessages { name: "ko", messages: korean::MESSAGES },
  AvailableMessages { name: "lb", messages: luxemburgish::MESSAGES },
  AvailableMessages { name: "li", messages: limburgish::MESSAGES },
  AvailableMessages { name: "mt", messages: maltese::MESSAGES },
  AvailableMessages { name: "nb", messages: norwegian::MESSAGES },
  AvailableMessages { name: "nl", messages: dutch::MESSAGES },
  AvailableMessages { name: "oc", messages: occitan::MESSAGES },
  AvailableMessages { name: "pl", messages: polish::MESSAGES },
  AvailableMessages { name: "rm", messages: romansh::MESSAGES },
  AvailableMessages { name: "ru", messages: russian::MESSAGES },
  AvailableMessages { name: "sv", messages: swedish::MESSAGES },
  AvailableMessages { name: "uk", messages: ukrainian::MESSAGES },
  AvailableMessages { name: "vi", messages: vietnamese::MESSAGES },
  AvailableMessages { name: "wae", messages: walser::MESSAGES },
  AvailableMessages { name: "wa", messages: walloon::MESSAGES },
  AvailableMessages { name: "wuu", messages: wuu::MESSAGES }
];

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
  #[inline]
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int> {
    let name = locale.to_str().map_err(|_| errno::EINVAL)?;

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
        return Err(errno::ENOENT);
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
        return Err(errno::ENOENT);
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

    let mut parts = name.split(['-', '_']);
    let lang = parts.next().unwrap_or("");
    if lang.is_empty() {
      return Err(errno::ENOENT);
    }

    for m in AVAILABLE_MESSAGES {
      if lang == m.name {
        self.name = Cow::Owned(locale.to_owned());
        self.misc_messages = m.messages.misc_messages;
        self.strerror = m.messages.strerror;
        self.strsignal = m.messages.strsignal;
        self.regerror = m.messages.regerror;
        self.hstrerror = m.messages.hstrerror;
        self.gai_strerror = m.messages.gai_strerror;
        self.noexpr = Cow::Owned(m.messages.noexpr.to_string());
        self.yesexpr = Cow::Owned(m.messages.yesexpr.to_string());
        return Ok(self.name.as_ref());
      }
    }

    Err(errno::ENOENT)
  }

  #[inline]
  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr {
    *self = Self::new();

    self.name = Cow::Owned(locale.to_owned());
    self.name.as_ref()
  }

  #[inline]
  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> MessagesObject<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      name: Cow::Borrowed(c"C"),
      misc_messages: american_english::MISC_MESSAGES,
      strerror: american_english::STRERROR,
      strsignal: american_english::STRSIGNAL,
      regerror: american_english::REGERROR,
      hstrerror: american_english::HSTRERROR,
      gai_strerror: american_english::GAI_STRERROR,
      noexpr: Cow::Borrowed(american_english::NOEXPR),
      yesexpr: Cow::Borrowed(american_english::YESEXPR)
    }
  }
}

impl<'a> Default for MessagesObject<'a> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
