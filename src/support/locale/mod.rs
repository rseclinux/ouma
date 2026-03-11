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
    std::{errno, locale},
    support::locale::locale::LC_GLOBAL_LOCALE
  },
  atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut},
  core::{
    cell::UnsafeCell,
    ffi,
    fmt::{Error, Write},
    ptr
  }
};

pub trait LocaleObject: Clone + Default {
  fn setlocale(
    &mut self,
    locale: &ffi::CStr
  ) -> Result<&ffi::CStr, c_int>;
  fn set_to_posix(&mut self) -> &ffi::CStr;
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

fn writer_name_to_category<W: Write>(
  f: &mut W,
  category: &str,
  s: &ffi::CStr,
  is_not_final: bool
) -> Result<(), Error> {
  if is_not_final {
    f.write_fmt(format_args!("{}={};", category, s.display()))
  } else {
    f.write_fmt(format_args!("{}={}", category, s.display()))
  }
}

pub struct Locale<'a> {
  lc_all: AtomicRefCell<[c_char; 1024]>,
  pub localeconv: AtomicRefCell<locale::lconv>,
  pub collate: AtomicRefCell<Option<collate::CollateObject<'a>>>,
  pub ctype: AtomicRefCell<Option<ctype::CtypeObject<'a>>>,
  pub messages: AtomicRefCell<Option<messages::MessagesObject<'a>>>,
  pub monetary: AtomicRefCell<Option<monetary::MonetaryObject<'a>>>,
  pub numeric: AtomicRefCell<Option<numeric::NumericObject<'a>>>,
  pub time: AtomicRefCell<Option<time::TimeObject<'a>>>
}

impl<'a> Locale<'a> {
  pub fn new() -> Self {
    Self {
      lc_all: AtomicRefCell::new([0; 1024]),
      localeconv: AtomicRefCell::new(unsafe { core::mem::zeroed() }),
      collate: AtomicRefCell::new(Some(collate::DEFAULT_COLLATE)),
      ctype: AtomicRefCell::new(Some(ctype::DEFAULT_CTYPE)),
      messages: AtomicRefCell::new(Some(messages::DEFAULT_MESSAGES)),
      monetary: AtomicRefCell::new(Some(monetary::DEFAULT_MONETARY)),
      numeric: AtomicRefCell::new(Some(numeric::DEFAULT_NUMERIC)),
      time: AtomicRefCell::new(Some(time::DEFAULT_TIME))
    }
  }

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

        let mut buf = self.lc_all.borrow_mut();
        buf.fill(0);

        let mut ss = crate::support::string::StringStream::new(&mut buf[..]);

        let cats: [(&'static str, &ffi::CStr, bool); 6] = [
          ("LC_COLLATE", collate, true),
          ("LC_CTYPE", ctype, true),
          ("LC_MESSAGES", messages, true),
          ("LC_MONETARY", monetary, true),
          ("LC_NUMERIC", numeric, true),
          ("LC_TIME", time, false)
        ];

        for (label, val, with_sep) in cats {
          if writer_name_to_category(&mut ss, label, val, with_sep).is_err() {
            return ptr::null_mut();
          }
        }

        let trimmed_size: usize = buf.iter().filter(|&x| *x != 0).count() + 1;
        let output = &mut buf[..trimmed_size];

        if output[trimmed_size - 1] as u8 != b'\0' {
          return ptr::null_mut();
        }

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
    localeconv: AtomicRefCell::new(unsafe { core::mem::zeroed() }),
    collate: AtomicRefCell::new(None),
    ctype: AtomicRefCell::new(None),
    messages: AtomicRefCell::new(None),
    monetary: AtomicRefCell::new(None),
    numeric: AtomicRefCell::new(None),
    time: AtomicRefCell::new(None)
  })
};

pub static DEFAULT_LOCALE: SyncLocale = SyncLocale {
  inner: UnsafeCell::new(Locale {
    lc_all: AtomicRefCell::new([0; 1024]),
    localeconv: AtomicRefCell::new(unsafe { core::mem::zeroed() }),
    collate: AtomicRefCell::new(Some(collate::DEFAULT_COLLATE)),
    ctype: AtomicRefCell::new(Some(ctype::DEFAULT_CTYPE)),
    messages: AtomicRefCell::new(Some(messages::DEFAULT_MESSAGES)),
    monetary: AtomicRefCell::new(Some(monetary::DEFAULT_MONETARY)),
    numeric: AtomicRefCell::new(Some(numeric::DEFAULT_NUMERIC)),
    time: AtomicRefCell::new(Some(time::DEFAULT_TIME))
  })
};

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
