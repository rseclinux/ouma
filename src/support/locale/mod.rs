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
    string::{String, ToString},
    sync::Arc
  },
  atomic_refcell::AtomicRefCell,
  core::{cell::SyncUnsafeCell, ffi, fmt::Write, ptr}
};

const LOCALE_NAME_MAX: usize = 513;

pub trait LocaleObject: Default {
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
pub fn set_slot<T: LocaleObject>(
  slot: &mut Option<Arc<T>>,
  name: &ffi::CStr
) -> Result<(), c_int> {
  let mut obj = T::default();
  obj.setlocale(name).map_err(|_| errno::ENOENT)?;
  *slot = Some(Arc::new(obj));
  Ok(())
}

const C_LOCALE_NAME: [u8; LOCALE_NAME_MAX] = {
  let mut arr = [0; LOCALE_NAME_MAX];
  arr[0] = b'C';
  arr
};

pub struct Locale<'a> {
  lc_all: AtomicRefCell<[u8; LOCALE_NAME_MAX]>,
  pub crncystr: AtomicRefCell<[u8; 32]>,
  pub localeconv: AtomicRefCell<locale::lconv>,
  pub collate: Option<Arc<collate::CollateObject<'a>>>,
  pub ctype: Option<Arc<ctype::CtypeObject<'a>>>,
  pub messages: Option<Arc<messages::MessagesObject<'a>>>,
  pub monetary: Option<Arc<monetary::MonetaryObject<'a>>>,
  pub numeric: Option<Arc<numeric::NumericObject<'a>>>,
  pub time: Option<Arc<time::TimeObject<'a>>>
}

impl<'a> Locale<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      lc_all: AtomicRefCell::new(C_LOCALE_NAME),
      crncystr: AtomicRefCell::new([0; 32]),
      localeconv: AtomicRefCell::new(lconv::new()),
      collate: None,
      ctype: None,
      messages: None,
      monetary: None,
      numeric: None,
      time: None
    }
  }

  #[inline]
  pub const fn global() -> Self {
    Self {
      lc_all: AtomicRefCell::new(C_LOCALE_NAME),
      crncystr: AtomicRefCell::new([0; 32]),
      localeconv: AtomicRefCell::new(lconv::new()),
      collate: None,
      ctype: None,
      messages: None,
      monetary: None,
      numeric: None,
      time: None
    }
  }

  #[inline]
  pub fn as_ptr(&mut self) -> locale_t<'a> {
    self as locale_t
  }

  #[inline]
  pub fn setlocale(
    &mut self,
    category: c_int,
    name: &ffi::CStr
  ) -> Result<&Self, c_int> {
    match category {
      | locale::LC_ALL => {
        set_slot(&mut self.collate, name)?;
        set_slot(&mut self.ctype, name)?;
        set_slot(&mut self.messages, name)?;
        set_slot(&mut self.monetary, name)?;
        set_slot(&mut self.numeric, name)?;
        set_slot(&mut self.time, name)?;
        Ok(self)
      },
      | locale::LC_COLLATE => {
        set_slot(&mut self.collate, name)?;
        Ok(self)
      },
      | locale::LC_CTYPE => {
        set_slot(&mut self.ctype, name)?;
        Ok(self)
      },
      | locale::LC_MESSAGES => {
        set_slot(&mut self.messages, name)?;
        Ok(self)
      },
      | locale::LC_MONETARY => {
        set_slot(&mut self.monetary, name)?;
        Ok(self)
      },
      | locale::LC_NUMERIC => {
        set_slot(&mut self.numeric, name)?;
        Ok(self)
      },
      | locale::LC_TIME => {
        set_slot(&mut self.time, name)?;
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
    get_slot!(collate, self.collate, collate::CollateObject);
    get_slot!(ctype, self.ctype, ctype::CtypeObject);
    get_slot!(messages, self.messages, messages::MessagesObject);
    get_slot!(monetary, self.monetary, monetary::MonetaryObject);
    get_slot!(numeric, self.numeric, numeric::NumericObject);
    get_slot!(time, self.time, time::TimeObject);

    let collate = collate.get_name();
    let ctype = ctype.get_name();
    let messages = messages.get_name();
    let monetary = monetary.get_name();
    let numeric = numeric.get_name();
    let time = time.get_name();

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

  #[inline]
  pub fn update_strings(&self) {
    self.update_lc_all();
    self.update_localeconv();
    self.update_langinfo();
  }

  #[inline]
  fn update_lc_all(&self) {
    get_slot!(collate, self.collate, collate::CollateObject);
    get_slot!(ctype, self.ctype, ctype::CtypeObject);
    get_slot!(messages, self.messages, messages::MessagesObject);
    get_slot!(monetary, self.monetary, monetary::MonetaryObject);
    get_slot!(numeric, self.numeric, numeric::NumericObject);
    get_slot!(time, self.time, time::TimeObject);

    let collate = collate.get_name();
    let ctype = ctype.get_name();
    let messages = messages.get_name();
    let monetary = monetary.get_name();
    let numeric = numeric.get_name();
    let time = time.get_name();

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
    get_slot!(monetary, self.monetary, monetary::MonetaryObject);
    get_slot!(numeric, self.numeric, numeric::NumericObject);

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

  #[inline]
  fn update_langinfo(&self) {
    const EMPTY: &[u8] = c"".to_bytes_with_nul();

    get_slot!(monetary, self.monetary, monetary::MonetaryObject);

    let mut output = self.crncystr.borrow_mut();
    output.fill(0);

    let currency_symbol = &monetary.currency_symbol;

    let Ok(currency_symbol) = currency_symbol.to_str() else {
      output[..EMPTY.len()].copy_from_slice(EMPTY);
      return;
    };

    if monetary.p_cs_precedes != monetary.n_cs_precedes ||
      monetary.p_cs_precedes > 1
    {
      output[..EMPTY.len()].copy_from_slice(EMPTY);
      return;
    }

    let precedes = monetary.p_cs_precedes;
    let position = if precedes == 1 { '-' } else { '+' };

    let mut writer = CBufWriter::new(output.as_mut_slice());
    let r = write!(&mut writer, "{position}{currency_symbol}\0");

    if r.is_err() {
      output[..EMPTY.len()].copy_from_slice(EMPTY);
      return;
    }
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
