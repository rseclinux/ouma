use {
  super::CaseMapObject,
  crate::c_int,
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

fn wcwidth(c: u32) -> c_int {
  if (' ' as u32..='~' as u32).contains(&c) {
    return 1;
  }

  if (c < ' ' as u32) || c == 0x7F || c == 0 {
    return 0;
  }

  let c = match char::from_u32(c) {
    | Some(c) => c,
    | None => return -1
  };

  if CodePointSetData::new::<DefaultIgnorableCodePoint>().contains(c) {
    return 0;
  }

  if CodePointSetData::new::<JoinControl>().contains(c) {
    return 0;
  }
  if CodePointSetData::new::<VariationSelector>().contains(c) {
    return 0;
  }

  let gc = CodePointMapData::<GeneralCategory>::new().get(c);
  if matches!(
    gc,
    GeneralCategory::NonspacingMark | GeneralCategory::EnclosingMark
  ) || CodePointSetData::new::<GraphemeExtend>().contains(c)
  {
    return 0;
  }

  if CodePointSetData::new::<Emoji>().contains(c) {
    return 2;
  }

  match CodePointMapData::<HangulSyllableType>::new().get(c) {
    | HangulSyllableType::VowelJamo | HangulSyllableType::TrailingJamo => {
      return 0;
    },
    | HangulSyllableType::LeadingJamo |
    HangulSyllableType::LeadingVowelSyllable |
    HangulSyllableType::LeadingVowelTrailingSyllable => return 2,
    | _ => ()
  }

  if c as u32 >= 0x3248 && c as u32 <= 0x4dff {
    if c as u32 <= 0x324f {
      return 2;
    };
    if c as u32 >= 0x4dc0 {
      return 2;
    };
  }

  match CodePointMapData::<EastAsianWidth>::new().get(c) {
    | EastAsianWidth::Fullwidth | EastAsianWidth::Wide => return 2,
    | EastAsianWidth::Halfwidth | EastAsianWidth::Narrow => return 1,
    | EastAsianWidth::Ambiguous | EastAsianWidth::Neutral => return 1,
    | _ => ()
  };

  -1
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
  wcwidth: wcwidth,
  tolower: tolower,
  toupper: toupper
};
