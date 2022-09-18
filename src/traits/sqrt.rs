use fixed::traits::Fixed;
use fixed::{
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};
use seq_macro::seq;

use crate::sqrt::{sqrt_i, sqrt_i1, sqrt_u, sqrt_u0, sqrt_u1};

/// Fixed number that has square root.
///
/// It is not implemented for number types with 0 integer bits.
pub trait FixedSqrt: Fixed {
    /// Calculate square root.
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_i {
    ($f:ident, $f0:literal) => {
        seq!(N in 0..$f0 {
            impl FixedSqrt for $f<N> {
                #[inline(always)]
                fn sqrt(self) -> Self {
                    sqrt_i(self)
                }
            }
        });
        impl FixedSqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_i1(self)
            }
        }
    };
}

macro_rules! impl_sqrt_u {
    ($f:ident, $f0:literal, $f1:literal) => {
        seq!(N in 0..$f0 {
            impl FixedSqrt for $f<N> {
                #[inline(always)]
                fn sqrt(self) -> Self {
                    sqrt_u(self)
                }
            }
        });
        impl FixedSqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u1(self)
            }
        }
        impl FixedSqrt for $f<$f1> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u0(self)
            }
        }
    };
}

impl_sqrt_i!(FixedI8, 7);
impl_sqrt_i!(FixedI16, 15);
impl_sqrt_i!(FixedI32, 31);
impl_sqrt_i!(FixedI64, 63);
impl_sqrt_i!(FixedI128, 127);

impl_sqrt_u!(FixedU8, 7, 8);
impl_sqrt_u!(FixedU16, 15, 16);
impl_sqrt_u!(FixedU32, 31, 32);
impl_sqrt_u!(FixedU64, 63, 64);
impl_sqrt_u!(FixedU128, 127, 128);
