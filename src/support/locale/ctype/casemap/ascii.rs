use {super::CaseMapObject, crate::c_int};

fn isalnum(c: u32) -> bool {
  (c >= '0' as u32 && c <= '9' as u32) ||
    (c >= 'A' as u32 && c <= 'Z' as u32) ||
    (c >= 'a' as u32 && c <= 'z' as u32)
}

fn isalpha(c: u32) -> bool {
  (c >= 'A' as u32 && c <= 'Z' as u32) || (c >= 'a' as u32 && c <= 'z' as u32)
}

fn isblank(c: u32) -> bool {
  c == '\t' as u32 || c == ' ' as u32
}

fn iscntrl(c: u32) -> bool {
  (c >= '\0' as u32 && c < ' ' as u32) || c == 0x7f
}

fn isdigit(c: u32) -> bool {
  c >= '0' as u32 && c <= '9' as u32
}

fn isgraph(c: u32) -> bool {
  c >= '!' as u32 && c <= '~' as u32
}

fn islower(c: u32) -> bool {
  c >= 'a' as u32 && c <= 'z' as u32
}

fn isprint(c: u32) -> bool {
  c >= ' ' as u32 && c <= '~' as u32
}

fn ispunct(c: u32) -> bool {
  (c >= '!' as u32 && c <= '/' as u32) ||
    (c >= ':' as u32 && c <= '@' as u32) ||
    (c >= '[' as u32 && c <= '`' as u32) ||
    (c >= '{' as u32 && c <= '~' as u32)
}

fn isspace(c: u32) -> bool {
  (c >= '\t' as u32 && c <= '\r' as u32) || c == ' ' as u32
}

fn isupper(c: u32) -> bool {
  c >= 'A' as u32 && c <= 'Z' as u32
}

fn isxdigit(c: u32) -> bool {
  (c >= '0' as u32 && c <= '9' as u32) ||
    (c >= 'A' as u32 && c <= 'F' as u32) ||
    (c >= 'a' as u32 && c <= 'f' as u32)
}

fn wcwidth(c: u32) -> c_int {
  match c {
    | 0 => 0,
    | 0x20..=0x7e => 1,
    | 0x01..=0x1f | 0x7f => -1,
    | _ => -1
  }
}

pub const CASEMAP_ASCII: CaseMapObject = CaseMapObject {
  isalnum: isalnum,
  isalpha: isalpha,
  isblank: isblank,
  iscntrl: iscntrl,
  isdigit: isdigit,
  isgraph: isgraph,
  islower: islower,
  isprint: isprint,
  ispunct: ispunct,
  isspace: isspace,
  isupper: isupper,
  isxdigit: isxdigit,
  wcwidth: wcwidth
};
