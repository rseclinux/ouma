mod types;
pub use types::{Locale, Tailoring};

mod ascii;
mod cea;
mod cea_utils;
mod consts;
mod first_weight;
mod normalize;
mod prefix;
mod sort_key;
mod tailor;
mod weights;

pub mod collate;
pub mod xfrm;
