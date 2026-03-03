use {
  super::{LocaleObject, is_posix_locale},
  crate::{allocation::borrow::ToOwned, c_int, support::locale::errno},
  allocation::borrow::Cow,
  core::ffi
};

mod american_english;
mod amish;
mod aussie;
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
mod flemish;
mod french;
mod german;
mod greek;
mod hakka;
mod hebrew;
mod hokkien;
mod italian;
mod japanese;
mod korean;
mod maltese;
mod norwegian;
mod polish;
mod romansh;
mod russian;
mod serbian_cyrillic;
mod serbian_latin;
mod swedish;
mod ukrainian;
mod vietnamese;
mod walloon;
mod walser;
mod wuu;

#[derive(Debug)]
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
      return Ok(self.set_to_posix());
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
      } else if name.contains("AU") {
        self.misc_messages = aussie::MISC_MESSAGES;
        self.strerror = aussie::STRERROR;
        self.strsignal = aussie::STRSIGNAL;
        self.regerror = aussie::REGERROR;
        self.hstrerror = aussie::HSTRERROR;
        self.gai_strerror = aussie::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(aussie::NOEXPR);
        self.yesexpr = Cow::Borrowed(aussie::YESEXPR);
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

    // Special case 2: Amish
    if name.starts_with("de") && name.contains("US") {
      self.misc_messages = amish::MISC_MESSAGES;
      self.strerror = amish::STRERROR;
      self.strsignal = amish::STRSIGNAL;
      self.regerror = amish::REGERROR;
      self.hstrerror = amish::HSTRERROR;
      self.gai_strerror = amish::GAI_STRERROR;
      self.noexpr = Cow::Borrowed(amish::NOEXPR);
      self.yesexpr = Cow::Borrowed(amish::YESEXPR);

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 3: Chinese
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

    // Special case 4: Cantonese
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

    // Special case 5: Portugese
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

    // Special case 6: Serbian
    if name.starts_with("sr") {
      if name.ends_with("@latin") {
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

    // Special case 7: Flemish
    if name.starts_with("nl") {
      if name.contains("BE") {
        self.misc_messages = flemish::MISC_MESSAGES;
        self.strerror = flemish::STRERROR;
        self.strsignal = flemish::STRSIGNAL;
        self.regerror = flemish::REGERROR;
        self.hstrerror = flemish::HSTRERROR;
        self.gai_strerror = flemish::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(flemish::NOEXPR);
        self.yesexpr = Cow::Borrowed(flemish::YESEXPR);
      } else {
        self.misc_messages = dutch::MISC_MESSAGES;
        self.strerror = dutch::STRERROR;
        self.strsignal = dutch::STRSIGNAL;
        self.regerror = dutch::REGERROR;
        self.hstrerror = dutch::HSTRERROR;
        self.gai_strerror = dutch::GAI_STRERROR;
        self.noexpr = Cow::Borrowed(dutch::NOEXPR);
        self.yesexpr = Cow::Borrowed(dutch::YESEXPR);
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    let mut parts = name.split(['_', '-']);
    let lang = parts.next().unwrap_or("");
    if lang.is_empty() {
      return Err(errno::ENOENT);
    }

    match lang {
      | "ca" => self.set_messages(
        &catalan::MISC_MESSAGES,
        &catalan::STRERROR,
        &catalan::STRSIGNAL,
        &catalan::REGERROR,
        &catalan::HSTRERROR,
        &catalan::GAI_STRERROR,
        catalan::NOEXPR,
        catalan::YESEXPR
      ),
      | "hr" => self.set_messages(
        &croatian::MISC_MESSAGES,
        &croatian::STRERROR,
        &croatian::STRSIGNAL,
        &croatian::REGERROR,
        &croatian::HSTRERROR,
        &croatian::GAI_STRERROR,
        croatian::NOEXPR,
        croatian::YESEXPR
      ),
      | "cs" => self.set_messages(
        &czech::MISC_MESSAGES,
        &czech::STRERROR,
        &czech::STRSIGNAL,
        &czech::REGERROR,
        &czech::HSTRERROR,
        &czech::GAI_STRERROR,
        czech::NOEXPR,
        czech::YESEXPR
      ),
      | "da" => self.set_messages(
        &danish::MISC_MESSAGES,
        &danish::STRERROR,
        &danish::STRSIGNAL,
        &danish::REGERROR,
        &danish::HSTRERROR,
        &danish::GAI_STRERROR,
        danish::NOEXPR,
        danish::YESEXPR
      ),
      | "et" => self.set_messages(
        &estonian::MISC_MESSAGES,
        &estonian::STRERROR,
        &estonian::STRSIGNAL,
        &estonian::REGERROR,
        &estonian::HSTRERROR,
        &estonian::GAI_STRERROR,
        estonian::NOEXPR,
        estonian::YESEXPR
      ),
      | "fi" => self.set_messages(
        &finnish::MISC_MESSAGES,
        &finnish::STRERROR,
        &finnish::STRSIGNAL,
        &finnish::REGERROR,
        &finnish::HSTRERROR,
        &finnish::GAI_STRERROR,
        finnish::NOEXPR,
        finnish::YESEXPR
      ),
      | "fr" => self.set_messages(
        &french::MISC_MESSAGES,
        &french::STRERROR,
        &french::STRSIGNAL,
        &french::REGERROR,
        &french::HSTRERROR,
        &french::GAI_STRERROR,
        french::NOEXPR,
        french::YESEXPR
      ),
      | "de" => self.set_messages(
        &german::MISC_MESSAGES,
        &german::STRERROR,
        &german::STRSIGNAL,
        &german::REGERROR,
        &german::HSTRERROR,
        &german::GAI_STRERROR,
        german::NOEXPR,
        german::YESEXPR
      ),
      | "el" => self.set_messages(
        &greek::MISC_MESSAGES,
        &greek::STRERROR,
        &greek::STRSIGNAL,
        &greek::REGERROR,
        &greek::HSTRERROR,
        &greek::GAI_STRERROR,
        greek::NOEXPR,
        greek::YESEXPR
      ),
      | "hak" => self.set_messages(
        &hakka::MISC_MESSAGES,
        &hakka::STRERROR,
        &hakka::STRSIGNAL,
        &hakka::REGERROR,
        &hakka::HSTRERROR,
        &hakka::GAI_STRERROR,
        hakka::NOEXPR,
        hakka::YESEXPR
      ),
      | "he" => self.set_messages(
        &hebrew::MISC_MESSAGES,
        &hebrew::STRERROR,
        &hebrew::STRSIGNAL,
        &hebrew::REGERROR,
        &hebrew::HSTRERROR,
        &hebrew::GAI_STRERROR,
        hebrew::NOEXPR,
        hebrew::YESEXPR
      ),
      | "it" => self.set_messages(
        &italian::MISC_MESSAGES,
        &italian::STRERROR,
        &italian::STRSIGNAL,
        &italian::REGERROR,
        &italian::HSTRERROR,
        &italian::GAI_STRERROR,
        italian::NOEXPR,
        italian::YESEXPR
      ),
      | "ja" => self.set_messages(
        &japanese::MISC_MESSAGES,
        &japanese::STRERROR,
        &japanese::STRSIGNAL,
        &japanese::REGERROR,
        &japanese::HSTRERROR,
        &japanese::GAI_STRERROR,
        japanese::NOEXPR,
        japanese::YESEXPR
      ),
      | "ko" => self.set_messages(
        &korean::MISC_MESSAGES,
        &korean::STRERROR,
        &korean::STRSIGNAL,
        &korean::REGERROR,
        &korean::HSTRERROR,
        &korean::GAI_STRERROR,
        korean::NOEXPR,
        korean::YESEXPR
      ),
      | "mt" => self.set_messages(
        &maltese::MISC_MESSAGES,
        &maltese::STRERROR,
        &maltese::STRSIGNAL,
        &maltese::REGERROR,
        &maltese::HSTRERROR,
        &maltese::GAI_STRERROR,
        maltese::NOEXPR,
        maltese::YESEXPR
      ),
      | "han" => self.set_messages(
        &hokkien::MISC_MESSAGES,
        &hokkien::STRERROR,
        &hokkien::STRSIGNAL,
        &hokkien::REGERROR,
        &hokkien::HSTRERROR,
        &hokkien::GAI_STRERROR,
        hokkien::NOEXPR,
        hokkien::YESEXPR
      ),
      | "nb" => self.set_messages(
        &norwegian::MISC_MESSAGES,
        &norwegian::STRERROR,
        &norwegian::STRSIGNAL,
        &norwegian::REGERROR,
        &norwegian::HSTRERROR,
        &norwegian::GAI_STRERROR,
        norwegian::NOEXPR,
        norwegian::YESEXPR
      ),
      | "pdc" => self.set_messages(
        &amish::MISC_MESSAGES,
        &amish::STRERROR,
        &amish::STRSIGNAL,
        &amish::REGERROR,
        &amish::HSTRERROR,
        &amish::GAI_STRERROR,
        amish::NOEXPR,
        amish::YESEXPR
      ),
      | "pl" => self.set_messages(
        &polish::MISC_MESSAGES,
        &polish::STRERROR,
        &polish::STRSIGNAL,
        &polish::REGERROR,
        &polish::HSTRERROR,
        &polish::GAI_STRERROR,
        polish::NOEXPR,
        polish::YESEXPR
      ),
      | "rm" => self.set_messages(
        &romansh::MISC_MESSAGES,
        &romansh::STRERROR,
        &romansh::STRSIGNAL,
        &romansh::REGERROR,
        &romansh::HSTRERROR,
        &romansh::GAI_STRERROR,
        romansh::NOEXPR,
        romansh::YESEXPR
      ),
      | "ru" => self.set_messages(
        &russian::MISC_MESSAGES,
        &russian::STRERROR,
        &russian::STRSIGNAL,
        &russian::REGERROR,
        &russian::HSTRERROR,
        &russian::GAI_STRERROR,
        russian::NOEXPR,
        russian::YESEXPR
      ),
      | "sv" => self.set_messages(
        &swedish::MISC_MESSAGES,
        &swedish::STRERROR,
        &swedish::STRSIGNAL,
        &swedish::REGERROR,
        &swedish::HSTRERROR,
        &swedish::GAI_STRERROR,
        swedish::NOEXPR,
        swedish::YESEXPR
      ),
      | "uk" => self.set_messages(
        &ukrainian::MISC_MESSAGES,
        &ukrainian::STRERROR,
        &ukrainian::STRSIGNAL,
        &ukrainian::REGERROR,
        &ukrainian::HSTRERROR,
        &ukrainian::GAI_STRERROR,
        ukrainian::NOEXPR,
        ukrainian::YESEXPR
      ),
      | "vi" => self.set_messages(
        &vietnamese::MISC_MESSAGES,
        &vietnamese::STRERROR,
        &vietnamese::STRSIGNAL,
        &vietnamese::REGERROR,
        &vietnamese::HSTRERROR,
        &vietnamese::GAI_STRERROR,
        vietnamese::NOEXPR,
        vietnamese::YESEXPR
      ),
      | "wa" => self.set_messages(
        &walloon::MISC_MESSAGES,
        &walloon::STRERROR,
        &walloon::STRSIGNAL,
        &walloon::REGERROR,
        &walloon::HSTRERROR,
        &walloon::GAI_STRERROR,
        walloon::NOEXPR,
        walloon::YESEXPR
      ),
      | "wae" => self.set_messages(
        &walser::MISC_MESSAGES,
        &walser::STRERROR,
        &walser::STRSIGNAL,
        &walser::REGERROR,
        &walser::HSTRERROR,
        &walser::GAI_STRERROR,
        walser::NOEXPR,
        walser::YESEXPR
      ),
      | "wuu" => self.set_messages(
        &wuu::MISC_MESSAGES,
        &wuu::STRERROR,
        &wuu::STRSIGNAL,
        &wuu::REGERROR,
        &wuu::HSTRERROR,
        &wuu::GAI_STRERROR,
        wuu::NOEXPR,
        wuu::YESEXPR
      ),
      | _ => return Err(errno::ENOENT)
    }

    self.name = Cow::Owned(locale.to_owned());
    Ok(self.name.as_ref())
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_MESSAGES;

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
