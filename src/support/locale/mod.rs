pub mod collate;
pub mod ctype;
pub mod messages;
pub mod monetary;
pub mod numeric;
pub mod time;

use {
  crate::{
    allocation::{
      format,
      string::{String, ToString}
    },
    c_char,
    c_int,
    intptr_t,
    locale_t,
    std::{errno, locale},
    support::{locale::locale::LC_GLOBAL_LOCALE, string::cbuf::CBufWriter}
  },
  atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut},
  core::{cell::UnsafeCell, ffi, fmt::Write, ptr}
};

pub trait LocaleObject: Clone + Default {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int>;
  fn set_to_posix(
    &mut self,
    locale: &ffi::CStr
  ) -> &ffi::CStr;
  fn get_name(&self) -> &ffi::CStr;
}

#[inline]
pub fn is_posix_locale(name: &str) -> bool {
  name == "C" ||
    name == "POSIX" ||
    name.starts_with("C.") ||
    name.starts_with("POSIX.")
}

#[inline]
pub fn canonicalize_locale(name: &str) -> (String, Option<String>) {
  let script_modifiers = [
    ("latin", "Latn"),
    ("cyrillic", "Cyrl"),
    ("devanagari", "Deva"),
    ("arabic", "Arab"),
    ("georgian", "Geor"),
    ("hant", "Hant"),
    ("hans", "Hans")
  ];

  let (without_codeset, codeset) = match name.split_once('.') {
    | Some((b, rest)) => {
      let (codeset, modifier_passthrough) = match rest.split_once('@') {
        | Some((c, m)) => (c, Some(m)),
        | None => (rest, None)
      };
      let reconstructed = match modifier_passthrough {
        | Some(m) => format!("{}@{}", b, m),
        | None => b.to_string()
      };
      (reconstructed, Some(codeset.to_string()))
    },
    | None => (name.to_string(), None)
  };

  let (base, modifier) = match without_codeset.split_once('@') {
    | Some((b, m)) => (b.to_string(), Some(m.to_string())),
    | None => (without_codeset, None)
  };

  let (lang, territory) = match base.split_once('_') {
    | Some((l, t)) => (l.to_string(), Some(t.to_string())),
    | None => (base, None)
  };

  let explicit_script = modifier.as_deref().and_then(|m| {
    script_modifiers
      .iter()
      .find(|(k, _)| k.eq_ignore_ascii_case(m))
      .map(|(_, v)| *v)
  });

  let default_script: Option<&str> = match lang.as_str() {
    | "wuu" => Some("Hans"),
    | "nan" => match territory.as_deref() {
      | Some("CN") => Some("Hans"),
      | _ => Some("Hant")
    },
    | "hak" => match territory.as_deref() {
      | Some("CN") => Some("Hans"),
      | _ => Some("Hant")
    },
    | "yue" => match territory.as_deref() {
      | Some("CN") => Some("Hans"),
      | _ => Some("Hant")
    },
    | "cmn" => match territory.as_deref() {
      | Some("CN") | Some("SG") => Some("Hans"),
      | _ => Some("Hant")
    },
    | _ => None
  };

  let script = explicit_script.or(default_script);

  let bcp_lang = match lang.as_str() {
    | "wuu" | "nan" | "hak" | "cmn" => "zh",
    | other => other
  };

  let mut out = bcp_lang.to_string();
  if let Some(s) = script {
    out.push('-');
    out.push_str(s);
  }
  if let Some(ref t) = territory {
    out.push('-');
    out.push_str(t);
  }

  if lang == "ar" && out.find("-u-nu-latn").is_none() {
    out.push_str("-u-nu-latn");
  }

  (out, codeset)
}

#[inline]
pub fn get_slot<'a, T: LocaleObject>(
  slot: &'a AtomicRefCell<Option<T>>
) -> Option<T> {
  let opt = slot.borrow();
  let guard = AtomicRef::filter_map(opt, |o| o.as_ref());
  if let Some(g) = guard { Some(g.clone()) } else { None }
}

#[inline]
pub fn get_slot_mut<'a, T: LocaleObject>(
  slot: &'a AtomicRefCell<Option<T>>
) -> AtomicRefMut<'a, T> {
  let opt = slot.borrow_mut();
  AtomicRefMut::map(opt, |o| o.get_or_insert_with(T::default))
}

#[inline]
pub fn get_slot_name<'a, T: LocaleObject>(
  slot: &'a AtomicRefCell<Option<T>>
) -> *const c_char {
  let opt = slot.borrow();
  let guard = AtomicRef::filter_map(opt, |o| o.as_ref());
  if let Some(g) = guard { g.get_name().as_ptr() } else { c"C".as_ptr() }
}

#[inline]
pub fn set_slot<T: LocaleObject>(
  slot: &AtomicRefCell<Option<T>>,
  name: &ffi::CStr
) -> Result<(), c_int> {
  let mut guard = slot.borrow_mut();
  let obj = guard.get_or_insert_with(T::default);
  obj.setlocale(name).map(|_| ()).map_err(|_| errno::ENOENT)
}

pub struct Locale<'a> {
  lc_all: AtomicRefCell<[u8; 1024]>,
  langinfo: AtomicRefCell<[u8; 1024]>,
  pub localeconv: AtomicRefCell<locale::lconv>,
  pub collate: AtomicRefCell<Option<collate::CollateObject<'a>>>,
  pub ctype: AtomicRefCell<Option<ctype::CtypeObject<'a>>>,
  pub messages: AtomicRefCell<Option<messages::MessagesObject<'a>>>,
  pub monetary: AtomicRefCell<Option<monetary::MonetaryObject<'a>>>,
  pub numeric: AtomicRefCell<Option<numeric::NumericObject<'a>>>,
  pub time: AtomicRefCell<Option<time::TimeObject<'a>>>
}

impl<'a> Locale<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      lc_all: AtomicRefCell::new([0; 1024]),
      langinfo: AtomicRefCell::new([0; 1024]),
      localeconv: AtomicRefCell::new(unsafe { core::mem::zeroed() }),
      collate: AtomicRefCell::new(Some(collate::CollateObject::new())),
      ctype: AtomicRefCell::new(Some(ctype::CtypeObject::new())),
      messages: AtomicRefCell::new(Some(messages::MessagesObject::new())),
      monetary: AtomicRefCell::new(Some(monetary::MonetaryObject::new())),
      numeric: AtomicRefCell::new(Some(numeric::NumericObject::new())),
      time: AtomicRefCell::new(Some(time::TimeObject::new()))
    }
  }

  #[inline]
  pub fn setlocale(
    &self,
    category: c_int,
    name: &ffi::CStr
  ) -> Result<&Self, c_int> {
    match category {
      | locale::LC_ALL => {
        set_slot(&self.collate, name)?;
        set_slot(&self.ctype, name)?;
        set_slot(&self.messages, name)?;
        set_slot(&self.monetary, name)?;
        set_slot(&self.numeric, name)?;
        set_slot(&self.time, name)?;
        Ok(self)
      },
      | locale::LC_COLLATE => {
        set_slot(&self.collate, name)?;
        Ok(self)
      },
      | locale::LC_CTYPE => {
        set_slot(&self.ctype, name)?;
        Ok(self)
      },
      | locale::LC_MESSAGES => {
        set_slot(&self.messages, name)?;
        Ok(self)
      },
      | locale::LC_MONETARY => {
        set_slot(&self.monetary, name)?;
        Ok(self)
      },
      | locale::LC_NUMERIC => {
        set_slot(&self.numeric, name)?;
        Ok(self)
      },
      | locale::LC_TIME => {
        set_slot(&self.time, name)?;
        Ok(self)
      },
      | _ => Err(errno::ENOENT)
    }
  }

  #[inline]
  pub fn querylocale(
    &self,
    category: c_int
  ) -> *mut c_char {
    let collate = unsafe { ffi::CStr::from_ptr(get_slot_name(&self.collate)) };
    let ctype = unsafe { ffi::CStr::from_ptr(get_slot_name(&self.ctype)) };
    let messages =
      unsafe { ffi::CStr::from_ptr(get_slot_name(&self.messages)) };
    let monetary =
      unsafe { ffi::CStr::from_ptr(get_slot_name(&self.monetary)) };
    let numeric = unsafe { ffi::CStr::from_ptr(get_slot_name(&self.numeric)) };
    let time = unsafe { ffi::CStr::from_ptr(get_slot_name(&self.time)) };

    match category {
      | locale::LC_ALL => {
        let names = [collate, ctype, monetary, numeric, messages, time];
        if names.windows(2).all(|w| w[0] == w[1]) {
          return collate.as_ptr().cast_mut();
        }

        let mut output = self.lc_all.borrow_mut();
        output.fill(0);

        let mut writer = CBufWriter::new(output.as_mut_slice());
        let r = write!(
          &mut writer,
          "LC_COLLATE={};LC_CTYPE={};LC_MESSAGES={};LC_MONETARY={};LC_NUMERIC={};LC_TIME={}\0",
          collate.display(),
          ctype.display(),
          messages.display(),
          monetary.display(),
          numeric.display(),
          time.display()
        );

        if r.is_err() {
          return ptr::null_mut();
        }

        let written: usize =
          output.iter().take_while(|&&b| b != b'\0').count() + 1;
        let output = &mut output[..written];

        output.as_mut_ptr().cast()
      },
      | locale::LC_COLLATE => collate.as_ptr().cast_mut(),
      | locale::LC_CTYPE => ctype.as_ptr().cast_mut(),
      | locale::LC_MESSAGES => messages.as_ptr().cast_mut(),
      | locale::LC_MONETARY => monetary.as_ptr().cast_mut(),
      | locale::LC_NUMERIC => numeric.as_ptr().cast_mut(),
      | locale::LC_TIME => time.as_ptr().cast_mut(),
      | _ => ptr::null_mut()
    }
  }
}

pub struct SyncLocale {
  inner: UnsafeCell<Locale<'static>>
}
unsafe impl Sync for SyncLocale {}

pub static GLOBAL_LOCALE: SyncLocale = SyncLocale {
  inner: UnsafeCell::new(Locale {
    lc_all: AtomicRefCell::new([0; 1024]),
    langinfo: AtomicRefCell::new([0; 1024]),
    localeconv: AtomicRefCell::new(unsafe { core::mem::zeroed() }),
    collate: AtomicRefCell::new(None),
    ctype: AtomicRefCell::new(None),
    messages: AtomicRefCell::new(None),
    monetary: AtomicRefCell::new(None),
    numeric: AtomicRefCell::new(None),
    time: AtomicRefCell::new(None)
  })
};

pub static DEFAULT_LOCALE: SyncLocale =
  SyncLocale { inner: UnsafeCell::new(Locale::new()) };

#[inline(always)]
pub fn get_real_locale(locale: locale_t<'static>) -> &'static Locale<'static> {
  unsafe {
    match locale as intptr_t {
      | 0 => &*DEFAULT_LOCALE.inner.get(),
      | -1 => &*GLOBAL_LOCALE.inner.get(),
      | _ => &*locale
    }
  }
}

#[thread_local]
static mut THREAD_LOCALE: Option<locale_t<'static>> = None;

#[inline]
pub fn get_thread_locale() -> &'static Locale<'static> {
  get_real_locale(get_thread_locale_ptr())
}

#[inline]
pub fn get_thread_locale_ptr() -> locale_t<'static> {
  unsafe { THREAD_LOCALE.unwrap_or(LC_GLOBAL_LOCALE) }
}

#[inline]
pub fn set_thread_locale(locale: Locale<'static>) {
  let mut locale = locale;
  let locale: locale_t<'static> = &mut locale;
  set_thread_locale_ptr(locale);
}

#[inline]
pub fn set_thread_locale_ptr(p: locale_t<'static>) {
  unsafe {
    if p == LC_GLOBAL_LOCALE {
      THREAD_LOCALE = None;
    } else {
      THREAD_LOCALE = Some(p);
    }
  }
}
