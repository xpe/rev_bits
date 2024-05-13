mod range;
mod ref_impl;
mod reverse;

pub use range::*;
pub use ref_impl::*;
pub use reverse::*;

const BITS: u32 = 64;
const ALL_ONES: u64 = 0xFFFFFFFFFFFFFFFF;
