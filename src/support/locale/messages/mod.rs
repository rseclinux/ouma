use {
  super::{LocaleObject, is_posix_locale},
  crate::{allocation::borrow::ToOwned, c_int, std::errno},
  allocation::borrow::Cow,
  core::ffi
};

mod afar;
mod albanian;
mod amish;
mod arabic;
mod belarusian;
mod bokmal;
mod cantonese;
mod chinese_hans;
mod chinese_hant;
mod danish;
mod dutch;
mod english;
mod flemish;
mod german;
mod japanese;
mod korean;
mod limburgish;
mod maltese;
mod polish;
mod romansh;
mod russian;
mod serbian;
mod shanghainese;
mod swedish;
mod ukrainian;

pub struct Messages<'a> {
  pub misc_messages: [&'a str; 3],
  pub strerror: [&'a str; 134],
  pub strsignal: [&'a str; 32],
  pub gai_strerror: [&'a str; 15],
  pub yesexpr: &'a ffi::CStr,
  pub noexpr: &'a ffi::CStr
}

pub struct AvailableMessages<'a> {
  pub name: &'a str,
  pub messages: Messages<'a>
}

const AVAILABLE_MESSAGES: [AvailableMessages; 23] = [
  AvailableMessages { name: "aa", messages: afar::MESSAGES_AFAR },
  AvailableMessages { name: "ar", messages: arabic::MESSAGES_ARABIC },
  AvailableMessages { name: "be", messages: belarusian::MESSAGES_BELARUSIAN },
  AvailableMessages { name: "en", messages: english::MESSAGES_ENGLISH },
  AvailableMessages { name: "da", messages: danish::MESSAGES_DANISH },
  AvailableMessages { name: "de", messages: german::MESSAGES_GERMAN },
  AvailableMessages { name: "li", messages: limburgish::MESSAGES_LIMBURGISH },
  AvailableMessages { name: "ja", messages: japanese::MESSAGES_JAPANESE },
  AvailableMessages { name: "ko", messages: korean::MESSAGES_KOREAN },
  AvailableMessages { name: "mt", messages: maltese::MESSAGES_MALTESE },
  AvailableMessages { name: "nb", messages: bokmal::MESSAGES_BOKMAL },
  AvailableMessages { name: "nl", messages: dutch::MESSAGES_DUTCH },
  AvailableMessages { name: "nn", messages: danish::MESSAGES_DANISH },
  AvailableMessages { name: "pdc", messages: amish::MESSAGES_AMISH },
  AvailableMessages { name: "pl", messages: polish::MESSAGES_POLISH },
  AvailableMessages { name: "rm", messages: romansh::MESSAGES_ROMANSH },
  AvailableMessages { name: "ru", messages: russian::MESSAGES_RUSSIAN },
  AvailableMessages { name: "sq", messages: albanian::MESSAGES_ALBANIAN },
  AvailableMessages { name: "sr", messages: serbian::MESSAGES_SERBIAN },
  AvailableMessages { name: "sv", messages: swedish::MESSAGES_SWEDISH },
  AvailableMessages { name: "uk", messages: ukrainian::MESSAGES_UKRAINIAN },
  AvailableMessages {
    name: "wuu",
    messages: shanghainese::MESSAGES_SHANGHAINESE
  },
  AvailableMessages { name: "yue", messages: cantonese::MESSAGES_CANTONESE }
];

pub struct MessagesObject<'a> {
  name: Cow<'a, ffi::CStr>,
  pub misc_messages: [&'a str; 3],
  pub strerror: [&'a str; 134],
  pub strsignal: [&'a str; 32],
  pub gai_strerror: [&'a str; 15],
  pub yesexpr: &'a ffi::CStr,
  pub noexpr: &'a ffi::CStr
}

impl<'a> LocaleObject for MessagesObject<'a> {
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

    // Special case 1: Chinese
    if name.starts_with("zh") {
      if name.contains("CN") {
        self.misc_messages = chinese_hans::MISC_MESSAGES;
        self.strerror = chinese_hans::STRERROR;
        self.strsignal = chinese_hans::STRSIGNAL;
        self.gai_strerror = chinese_hans::GAI_STRERROR;
        self.noexpr = chinese_hans::NOEXPR;
        self.yesexpr = chinese_hans::YESEXPR;
      } else {
        self.misc_messages = chinese_hant::MISC_MESSAGES;
        self.strerror = chinese_hant::STRERROR;
        self.strsignal = chinese_hant::STRSIGNAL;
        self.gai_strerror = chinese_hant::GAI_STRERROR;
        self.noexpr = chinese_hant::NOEXPR;
        self.yesexpr = chinese_hant::YESEXPR;
      }

      self.name = Cow::Owned(locale.to_owned());

      return Ok(self.name.as_ref());
    }

    // Special case 2: Flemish
    if name.starts_with("nl") && name.contains("BE") {
      self.name = Cow::Owned(locale.to_owned());
      self.misc_messages = flemish::MISC_MESSAGES;
      self.strerror = flemish::STRERROR;
      self.strsignal = flemish::STRSIGNAL;
      self.gai_strerror = flemish::GAI_STRERROR;
      self.noexpr = flemish::NOEXPR;
      self.yesexpr = flemish::YESEXPR;

      return Ok(self.name.as_ref());
    }

    let mut parts = name.split(['_', '-']);
    let lang = parts.next().unwrap_or("");
    if lang.is_empty() {
      return Err(errno::EINVAL);
    }

    for m in AVAILABLE_MESSAGES {
      if lang == m.name {
        self.name = Cow::Owned(locale.to_owned());
        self.misc_messages = m.messages.misc_messages;
        self.strerror = m.messages.strerror;
        self.strsignal = m.messages.strsignal;
        self.gai_strerror = m.messages.gai_strerror;
        self.noexpr = m.messages.noexpr;
        self.yesexpr = m.messages.yesexpr;

        return Ok(self.name.as_ref());
      }
    }

    Err(errno::EINVAL)
  }

  fn set_to_posix(&mut self) -> &ffi::CStr {
    *self = DEFAULT_MESSAGES;

    self.name.as_ref()
  }

  fn get_name(&self) -> &ffi::CStr {
    self.name.as_ref()
  }
}

impl<'a> Default for MessagesObject<'a> {
  fn default() -> Self {
    DEFAULT_MESSAGES
  }
}

pub const DEFAULT_MESSAGES: MessagesObject = MessagesObject {
  name: Cow::Borrowed(c"C"),
  misc_messages: english::MISC_MESSAGES,
  strerror: english::STRERROR,
  strsignal: english::STRSIGNAL,
  gai_strerror: english::GAI_STRERROR,
  noexpr: english::NOEXPR,
  yesexpr: english::YESEXPR
};
