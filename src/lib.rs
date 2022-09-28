#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod util;

pub mod traits;
pub use traits::*;

pub mod sqrt;

pub mod trig;

pub(crate) mod tables;
