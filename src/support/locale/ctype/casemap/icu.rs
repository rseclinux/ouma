use {
  super::CaseMapObject,
  icu_casemap::CaseMapper,
  icu_properties::{CodePointMapData, CodePointSetData, props::*}
};

fn isalnum(c: u32) -> bool {
  CodePointSetData::new::<Alnum>().contains32(c)
}

fn isalpha(c: u32) -> bool {
  CodePointSetData::new::<Alphabetic>().contains32(c)
}

fn isblank(c: u32) -> bool {
  CodePointSetData::new::<Blank>().contains32(c)
}

fn iscntrl(c: u32) -> bool {
  CodePointMapData::<GeneralCategory>::new().get32(c) ==
    GeneralCategory::Control
}

fn isdigit(c: u32) -> bool {
  CodePointMapData::<GeneralCategory>::new().get32(c) ==
    GeneralCategory::DecimalNumber
}

fn isgraph(c: u32) -> bool {
  CodePointSetData::new::<Graph>().contains32(c)
}

fn islower(c: u32) -> bool {
  CodePointSetData::new::<Lowercase>().contains32(c)
}

fn isprint(c: u32) -> bool {
  CodePointSetData::new::<Print>().contains32(c)
}

fn ispunct(c: u32) -> bool {
  CodePointMapData::<GeneralCategory>::new().get32(c) >=
    GeneralCategory::DashPunctuation &&
    CodePointMapData::<GeneralCategory>::new().get32(c) <=
      GeneralCategory::OtherPunctuation
}

fn isspace(c: u32) -> bool {
  CodePointSetData::new::<WhiteSpace>().contains32(c)
}

fn isupper(c: u32) -> bool {
  CodePointSetData::new::<Uppercase>().contains32(c)
}

fn isxdigit(c: u32) -> bool {
  CodePointSetData::new::<Xdigit>().contains32(c)
}

pub fn tolower(c: u32) -> u32 {
  let Ok(c) = char::try_from(c) else {
    return c as u32;
  };
  let cm = CaseMapper::new();

  cm.simple_lowercase(c) as u32
}

pub fn toupper(c: u32) -> u32 {
  let Ok(c) = char::try_from(c) else {
    return c as u32;
  };
  let cm = CaseMapper::new();

  cm.simple_uppercase(c) as u32
}

pub const CASEMAP_ICU: CaseMapObject = CaseMapObject {
  isalnum: isalnum,
  isalpha: isalpha,
  isblank: isblank,
  isdigit: isdigit,
  iscntrl: iscntrl,
  isgraph: isgraph,
  islower: islower,
  isprint: isprint,
  ispunct: ispunct,
  isspace: isspace,
  isupper: isupper,
  isxdigit: isxdigit,
  tolower: tolower,
  toupper: toupper
};
