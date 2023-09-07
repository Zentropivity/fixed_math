use fixed::{
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};

use seq_macro::seq;
use typenum::*;

use crate::sqrt::{sqrt_i, sqrt_i1, sqrt_u, sqrt_u0, sqrt_u1};

/// Take square root of a number.
///
/// It is not implemented for fixed number types with 0 integer bits.
/// Implementation exists for `f32` and `f64` too.
pub trait Sqrt {
    /// Calculate the square root of `self`.
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_i {
    ($f:ident, $f0:ty) => {
        impl Sqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_i(self)
            }
        }
    };
}
macro_rules! impl_sqrt_i1 {
    ($f:ident, $f0:ty) => {
        impl Sqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_i1(self)
            }
        }
    };
}

macro_rules! impl_sqrt_u {
    ($f:ident, $f0:ty) => {
        impl Sqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u(self)
            }
        }
    };
}
macro_rules! impl_sqrt_u1 {
    ($f:ident, $f0:ty) => {
        impl Sqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u1(self)
            }
        }
    };
}
macro_rules! impl_sqrt_u0 {
    ($f:ident, $f0:ty) => {
        impl Sqrt for $f<$f0> {
            #[inline(always)]
            fn sqrt(self) -> Self {
                sqrt_u0(self)
            }
        }
    };
}

seq!(N in 0..7 {
    impl_sqrt_i!(FixedI8, U~N);
});
impl_sqrt_i1!(FixedI8, U7);

seq!(N in 0..15 {
    impl_sqrt_i!(FixedI16, U~N);
});
impl_sqrt_i1!(FixedI16, U15);

seq!(N in 0..31 {
    impl_sqrt_i!(FixedI32, U~N);
});
impl_sqrt_i1!(FixedI32, U31);

seq!(N in 0..63 {
    impl_sqrt_i!(FixedI64, U~N);
});
impl_sqrt_i1!(FixedI64, U63);

seq!(N in 0..127 {
    impl_sqrt_i!(FixedI128, U~N);
});
impl_sqrt_i1!(FixedI128, U127);

seq!(N in 0..7 {
    impl_sqrt_u!(FixedU8, U~N);
});
impl_sqrt_u1!(FixedU8, U7);
impl_sqrt_u0!(FixedU8, U8);

seq!(N in 0..15 {
    impl_sqrt_u!(FixedU16, U~N);
});
impl_sqrt_u1!(FixedU16, U15);
impl_sqrt_u0!(FixedU16, U16);

seq!(N in 0..31 {
    impl_sqrt_u!(FixedU32, U~N);
});
impl_sqrt_u1!(FixedU32, U31);
impl_sqrt_u0!(FixedU32, U32);

seq!(N in 0..63 {
    impl_sqrt_u!(FixedU64, U~N);
});
impl_sqrt_u1!(FixedU64, U63);
impl_sqrt_u0!(FixedU64, U64);

seq!(N in 0..127 {
    impl_sqrt_u!(FixedU128, U~N);
});
impl_sqrt_u1!(FixedU128, U127);
impl_sqrt_u0!(FixedU128, U128);

impl Sqrt for f32 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}
impl Sqrt for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}
