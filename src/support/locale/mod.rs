pub mod collate;
pub mod ctype;
pub mod messages;
pub mod monetary;
pub mod numeric;
pub mod time;

use {
  crate::{
    c_char,
    c_int,
    intptr_t,
    locale_t,
    std::{
      errno,
      locale::{self, lconv}
    },
    support::{locale::locale::LC_GLOBAL_LOCALE, string::cbuf::CBufWriter}
  },
  allocation::{
    format,
    string::{String, ToString}
  },
  atomic_refcell::{AtomicRef, AtomicRefCell},
  core::{cell::SyncUnsafeCell, ffi, fmt::Write, ptr}
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

  let (lang, territory) = match base.split_once(&['_', '-']) {
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

  let mut out = lang.to_string();
  if let Some(s) = script {
    out.push('-');
    out.push_str(s);
  }
  if let Some(ref t) = territory {
    out.push('-');
    out.push_str(t);
  }

  if (lang == "ar" || lang == "fa") && out.find("-u-nu-latn").is_none() {
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

const C_LOCALE_NAME: [u8; 512] = {
  let mut arr = [0; 512];
  arr[0] = b'C';
  arr
};

#[derive(Debug)]
pub struct Locale<'a> {
  lc_all: AtomicRefCell<[u8; 512]>,
  langinfo: AtomicRefCell<[u8; 512]>,
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
      lc_all: AtomicRefCell::new(C_LOCALE_NAME),
      langinfo: AtomicRefCell::new([0; 512]),
      localeconv: AtomicRefCell::new(lconv::new()),
      collate: AtomicRefCell::new(Some(collate::CollateObject::new())),
      ctype: AtomicRefCell::new(Some(ctype::CtypeObject::new())),
      messages: AtomicRefCell::new(Some(messages::MessagesObject::new())),
      monetary: AtomicRefCell::new(Some(monetary::MonetaryObject::new())),
      numeric: AtomicRefCell::new(Some(numeric::NumericObject::new())),
      time: AtomicRefCell::new(Some(time::TimeObject::new()))
    }
  }

  #[inline]
  pub const fn global() -> Self {
    Self {
      lc_all: AtomicRefCell::new(C_LOCALE_NAME),
      langinfo: AtomicRefCell::new([0; 512]),
      localeconv: AtomicRefCell::new(lconv::new()),
      collate: AtomicRefCell::new(None),
      ctype: AtomicRefCell::new(None),
      messages: AtomicRefCell::new(None),
      monetary: AtomicRefCell::new(None),
      numeric: AtomicRefCell::new(None),
      time: AtomicRefCell::new(None)
    }
  }

  #[inline]
  pub fn as_ptr(&mut self) -> locale_t<'a> {
    self as locale_t
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
      | locale::LC_ALL => self.lc_all.as_ptr().cast(),
      | locale::LC_COLLATE => collate.as_ptr().cast_mut(),
      | locale::LC_CTYPE => ctype.as_ptr().cast_mut(),
      | locale::LC_MESSAGES => messages.as_ptr().cast_mut(),
      | locale::LC_MONETARY => monetary.as_ptr().cast_mut(),
      | locale::LC_NUMERIC => numeric.as_ptr().cast_mut(),
      | locale::LC_TIME => time.as_ptr().cast_mut(),
      | _ => ptr::null_mut()
    }
  }

  pub fn update_strings(&self) {
    self.update_lc_all();
    self.update_localeconv();
  }

  #[inline]
  fn update_lc_all(&self) {
    let collate_guard = self.collate.borrow();
    let ctype_guard = self.ctype.borrow();
    let messages_guard = self.messages.borrow();
    let monetary_guard = self.monetary.borrow();
    let numeric_guard = self.numeric.borrow();
    let time_guard = self.time.borrow();

    let default_collate = collate::CollateObject::new();
    let default_ctype = ctype::CtypeObject::new();
    let default_messages = messages::MessagesObject::new();
    let default_monetary = monetary::MonetaryObject::new();
    let default_numeric = numeric::NumericObject::new();
    let default_time = time::TimeObject::new();

    let collate = collate_guard.as_ref().unwrap_or(&default_collate).get_name();
    let ctype = ctype_guard.as_ref().unwrap_or(&default_ctype).get_name();
    let messages =
      messages_guard.as_ref().unwrap_or(&default_messages).get_name();
    let monetary =
      monetary_guard.as_ref().unwrap_or(&default_monetary).get_name();
    let numeric = numeric_guard.as_ref().unwrap_or(&default_numeric).get_name();
    let time = time_guard.as_ref().unwrap_or(&default_time).get_name();

    let mut output = self.lc_all.borrow_mut();
    output.fill(0);

    let names = [collate, ctype, monetary, numeric, messages, time];
    if names.windows(2).all(|w| w[0] == w[1]) {
      let bytes = collate.to_bytes_with_nul();
      output[..bytes.len()].copy_from_slice(bytes);
      return;
    }

    let mut writer = CBufWriter::new(output.as_mut_slice());
    let r = write!(
      &mut writer,
      "LC_COLLATE={};LC_CTYPE={};LC_MESSAGES={};LC_MONETARY={};LC_NUMERIC={};LC_TIME={}\0",
      collate.to_bytes().escape_ascii(),
      ctype.to_bytes().escape_ascii(),
      messages.to_bytes().escape_ascii(),
      monetary.to_bytes().escape_ascii(),
      numeric.to_bytes().escape_ascii(),
      time.to_bytes().escape_ascii()
    );

    if r.is_err() {
      output.fill(0);
    }
  }

  #[inline]
  fn update_localeconv(&self) {
    let numeric_guard = self.numeric.borrow();
    let monetary_guard = self.monetary.borrow();

    let default_numeric = numeric::NumericObject::new();
    let default_monetary = monetary::MonetaryObject::new();

    let numeric = numeric_guard.as_ref().unwrap_or(&default_numeric);
    let monetary = monetary_guard.as_ref().unwrap_or(&default_monetary);

    let grouping: *mut c_char = if numeric.grouping.is_empty() {
      let slice: &[u8] = &[b'\0'];
      slice.as_ptr() as *mut u8 as *mut c_char
    } else {
      numeric.grouping.as_slice().as_ptr() as *mut u8 as *mut c_char
    };
    let mon_grouping: *mut c_char = if monetary.mon_grouping.is_empty() {
      let slice: &[u8] = &[b'\0'];
      slice.as_ptr() as *mut u8 as *mut c_char
    } else {
      monetary.mon_grouping.as_slice().as_ptr() as *mut u8 as *mut c_char
    };
    let int_curr_symbol: *mut c_char = if monetary.int_curr_symbol.is_empty() {
      let slice: &[u8] = &[b'\0'];
      slice.as_ptr() as *mut u8 as *mut c_char
    } else {
      monetary.int_curr_symbol.as_slice().as_ptr() as *mut u8 as *mut c_char
    };

    let result = lconv {
      decimal_point: numeric.decimal_point.as_ptr().cast_mut(),
      thousands_sep: numeric.thousands_sep.as_ptr().cast_mut(),
      grouping: grouping,
      int_curr_symbol: int_curr_symbol,
      currency_symbol: monetary.currency_symbol.as_ptr().cast_mut(),
      mon_decimal_point: monetary.mon_decimal_point.as_ptr().cast_mut(),
      mon_thousands_sep: monetary.mon_thousands_sep.as_ptr().cast_mut(),
      mon_grouping: mon_grouping,
      positive_sign: monetary.positive_sign.as_ptr().cast_mut(),
      negative_sign: monetary.negative_sign.as_ptr().cast_mut(),
      int_frac_digits: monetary.int_frac_digits,
      frac_digits: monetary.frac_digits,
      p_cs_precedes: monetary.p_cs_precedes,
      p_sep_by_space: monetary.p_sep_by_space,
      n_cs_precedes: monetary.n_cs_precedes,
      n_sep_by_space: monetary.n_sep_by_space,
      p_sign_posn: monetary.p_sign_posn,
      n_sign_posn: monetary.n_sign_posn,
      int_p_cs_precedes: monetary.int_p_cs_precedes,
      int_p_sep_by_space: monetary.int_p_sep_by_space,
      int_n_cs_precedes: monetary.int_n_cs_precedes,
      int_n_sep_by_space: monetary.int_n_sep_by_space,
      int_p_sign_posn: monetary.int_p_sign_posn,
      int_n_sign_posn: monetary.int_n_sign_posn
    };

    unsafe { core::ptr::write(self.localeconv.as_ptr(), result) };
  }
}

#[thread_local]
static THREAD_LOCALE: SyncUnsafeCell<Option<locale_t<'static>>> =
  SyncUnsafeCell::new(None);

pub static GLOBAL_LOCALE: SyncUnsafeCell<Locale<'static>> =
  SyncUnsafeCell::new(Locale::global());

pub static DEFAULT_LOCALE: SyncUnsafeCell<Locale<'static>> =
  SyncUnsafeCell::new(Locale::new());

#[inline]
pub fn get_global_locale() -> &'static Locale<'static> {
  unsafe { &*GLOBAL_LOCALE.get() }
}

#[inline]
pub fn set_global_locale(loc: Locale<'static>) {
  unsafe {
    *GLOBAL_LOCALE.get() = loc;
  };
}

#[inline]
pub fn get_real_locale(locale: locale_t<'static>) -> &'static Locale<'static> {
  unsafe {
    match locale as intptr_t {
      | 0 => &*DEFAULT_LOCALE.get(),
      | -1 => &*GLOBAL_LOCALE.get(),
      | _ => &*locale
    }
  }
}

#[inline]
pub fn get_thread_locale() -> &'static Locale<'static> {
  get_real_locale(get_thread_locale_ptr())
}

#[inline]
pub fn get_thread_locale_ptr() -> locale_t<'static> {
  let loc = unsafe { *THREAD_LOCALE.get() };
  loc.unwrap_or(LC_GLOBAL_LOCALE)
}

#[inline]
pub fn set_thread_locale_ptr(p: locale_t<'static>) {
  unsafe {
    if p == LC_GLOBAL_LOCALE {
      *THREAD_LOCALE.get() = None;
    } else {
      *THREAD_LOCALE.get() = Some(p);
    }
  }
}
