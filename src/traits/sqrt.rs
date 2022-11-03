use fixed::traits::Fixed;
use fixed::types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8};
use fixed::{
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};

use typenum::{IsLess, True, U127, U15, U31, U63, U7};

use crate::sqrt::{sqrt_i, sqrt_u};

/// Fixed number that has square root.
///
/// It is not implemented for number types with 0 integer bits.
pub trait FixedSqrt: Fixed {
    /// Calculate square root.
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_i {
    ($f:ident, $leq:ident, $f0:ty) => {
        impl<N> FixedSqrt for $f<N>
        where
            N: $leq + IsLess<$f0, Output = True>,
        {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_i(self)
            }
        }
        //FIXME this should be doable...
        // impl<N> FixedSqrt for $f<N>
        // where
        //     N: Unsigned + IsEqual<$f0, Output = True>,
        // {
        //     #[inline(always)]
        //     fn sqrt(self) -> Self {
        //         sqrt_i1(self)
        //     }
        // }
    };
}

macro_rules! impl_sqrt_u {
    ($f:ident, $leq:ident, $f0:ty, $f1:ty) => {
        impl<N> FixedSqrt for $f<N>
        where
            N: $leq + IsLess<$f0, Output = True>,
        {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u(self)
            }
        }
        //FIXME this should be doable...
        // impl FixedSqrt for $f<$f0> {
        //     #[inline(always)]
        //     fn sqrt(self) -> Self {
        //         sqrt_u1(self)
        //     }
        // }
        // impl FixedSqrt for $f<$f1> {
        //     #[inline(always)]
        //     fn sqrt(self) -> Self {
        //         sqrt_u0(self)
        //     }
        // }
    };
}

impl_sqrt_i!(FixedI8, LeEqU8, U7);
impl_sqrt_i!(FixedI16, LeEqU16, U15);
impl_sqrt_i!(FixedI32, LeEqU32, U31);
impl_sqrt_i!(FixedI64, LeEqU64, U63);
impl_sqrt_i!(FixedI128, LeEqU128, U127);

impl_sqrt_u!(FixedU8, LeEqU8, U7, U8);
impl_sqrt_u!(FixedU16, LeEqU16, U15, U16);
impl_sqrt_u!(FixedU32, LeEqU32, U31, U32);
impl_sqrt_u!(FixedU64, LeEqU64, U63, U64);
impl_sqrt_u!(FixedU128, LeEqU128, U127, U128);
