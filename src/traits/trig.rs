use crate::trig::*;
use fixed::{traits::FixedSigned, FixedI128, FixedI16, FixedI32, FixedI64, FixedI8};
use seq_macro::seq;

//TODO impl for lower than 10 int bits
/// Calculation of sinus and cosinus for fixed number in degrees.
pub trait FixedSinCos: FixedSigned {
    fn sin_cos(self) -> (Self, Self);
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Option<Self>;
}

macro_rules! impl_sincos_deg {
    ($f:ident, $f0:literal) => {
        seq!(FRAC in 0..=$f0 {
            impl FixedSinCos for $f<FRAC> {
                fn sin_cos(self) -> (Self, Self) {
                    sin_cos(self)
                }
                fn sin(self) -> Self {
                    sin(self)
                }
                fn cos(self) -> Self {
                    cos(self)
                }
                fn tan(self) -> Option<Self> {
                    tan(self)
                }
            }
        });
    };
}

macro_rules! impl_sincos_deg_small {
    ($f:ident, $f0:literal) => {
        impl FixedSinCos for $f<{ $f0 + 2 }> {
            fn sin_cos(self) -> (Self, Self) {
                sin_cos_i7(self)
            }
            fn sin(self) -> Self {
                sin_cos_i7(self).0
            }
            fn cos(self) -> Self {
                sin_cos_i7(self).1
            }
            fn tan(self) -> Option<Self> {
                let (sin, cos) = sin_cos_i7(self);
                sin.checked_div(cos)
            }
        }
        impl FixedSinCos for $f<{ $f0 + 1 }> {
            fn sin_cos(self) -> (Self, Self) {
                sin_cos_i8(self)
            }
            fn sin(self) -> Self {
                sin_cos_i8(self).0
            }
            fn cos(self) -> Self {
                sin_cos_i8(self).1
            }
            fn tan(self) -> Option<Self> {
                let (sin, cos) = sin_cos_i8(self);
                sin.checked_div(cos)
            }
        }
        impl FixedSinCos for $f<$f0> {
            fn sin_cos(self) -> (Self, Self) {
                sin_cos_i9(self)
            }
            fn sin(self) -> Self {
                sin_cos_i9(self).0
            }
            fn cos(self) -> Self {
                sin_cos_i9(self).1
            }
            fn tan(self) -> Option<Self> {
                let (sin, cos) = sin_cos_i9(self);
                sin.checked_div(cos)
            }
        }
    };
}

impl_sincos_deg!(FixedI16, 6);
impl_sincos_deg!(FixedI32, 22);
impl_sincos_deg!(FixedI64, 54);
impl_sincos_deg!(FixedI128, 118);

impl_sincos_deg_small!(FixedI16, 7);
impl_sincos_deg_small!(FixedI32, 23);
impl_sincos_deg_small!(FixedI64, 55);
impl_sincos_deg_small!(FixedI128, 119);

//TODO impl this somehow for perf
// pub trait FixedDegrees: FixedSigned {
//     const D_90: Self;
//     const D_180: Self;
//     const D_270: Self;
//     const D_360: Self;
// }

/// There are requirements for certain constants:
///
/// - FRAC_PI_2 needs 2 int bits
/// - PI needs 3 int bits
/// - TAU needs 4 int bits
///
/// => for a fixed number with N bits,
///    we can have at most N - 3 fractional bits for constants to be representable
pub trait FixedRadians: FixedSigned {
    const FRAC_PI_2: Self;
    const PI: Self;
    const TAU: Self;
}

macro_rules! impl_rad_consts {
    ($f:ident, $f0:literal) => {
        seq!(FRAC in 0..=$f0 {
            impl FixedRadians for $f<FRAC> {
                const FRAC_PI_2: Self = Self::FRAC_PI_2;
                const PI: Self = Self::PI;
                const TAU: Self = Self::TAU;
            }
        });
    };
}
impl_rad_consts!(FixedI8, 4);
impl_rad_consts!(FixedI16, 12);
impl_rad_consts!(FixedI32, 28);
impl_rad_consts!(FixedI64, 60);
impl_rad_consts!(FixedI128, 124);
