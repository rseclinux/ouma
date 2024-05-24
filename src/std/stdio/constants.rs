use crate::{c_int, c_uint};

pub const _IOFBF: c_int = 0;
pub const _IOLBF: c_int = 1;
pub const _IONBF: c_int = 2;

pub const EOF: c_int = -1;

pub const BUFSIZ: c_uint = 8192;
pub const FILENAME_MAX: c_uint = 4096;
pub const FOPEN_MAX: c_uint = 16;
pub const TMP_MAX: c_uint = 238328;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const L_tmpnam: c_uint = 20;
pub const L_ctermid: c_uint = 9;
pub const L_cuserid: c_uint = 9;
pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
