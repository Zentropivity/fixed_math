//! # Trigonometric functions for fixed numbers.
//!
//! The following functions are implemented in some capacity:
//!
//! - sqrt
//! - sin_cos, sin, cos, tan
//!
//! They have different constraints and may panic or return wrong values if called with the wrong number representation.
//!
//! The trait implementations ([SinCos], [Sqrt]) could not be implemented for all valid number representations, use the functions for those.
//!
//! ## Example
//!
//! ```
//! use fixed::types::I32F32;
//! use fixed_math::Sqrt;
//! use fixed_math::sqrt::sqrt_i;
//!
//! let f = I32F32::from_num(4.0);
//!
//! assert_eq!(f.sqrt(), I32F32::from_num(2.0));
//! assert_eq!(sqrt_i(f), I32F32::from_num(2.0));
//! ```

pub mod util;

pub mod traits;
pub use traits::*;

pub mod sqrt;

pub mod trig;

pub(crate) mod tables;
