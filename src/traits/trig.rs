use crate::trig::*;
use fixed::{
    traits::FixedSigned,
    types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8},
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8,
};

use typenum::{IsLessOrEqual, Sum, True, U1, U118, U12, U124, U2, U22, U28, U4, U54, U6, U60};

/// Calculation of sinus, cosinus and tangent for number in **degrees**.
pub trait SinCos
where
    Self: Sized,
{
    /// Simultaneously calculate sinus and cosinus, returns `(sin(self), cos(self))`.
    fn sin_cos(self) -> (Self, Self);
    /// Calculate sinus.
    /// `sin_cos` should be used instead when both sinus and cosinus are needed.
    fn sin(self) -> Self;
    /// Calculate cosinus.
    /// `sin_cos` should be used instead when both sinus and cosinus are needed.
    fn cos(self) -> Self;
    /// Calculate tangent if it exists
    fn tan(self) -> Option<Self>;
}

macro_rules! impl_sincos_deg {
    ($f:ident, $leq:ident, $f0:ty) => {
        impl<N> SinCos for $f<N>
        where
            N: $leq + IsLessOrEqual<$f0, Output = True>,
        {
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
    };
}

// macro_rules! impl_sincos_deg_small {
//     ($f:ident, $f0:ty, $f1:ty, $f2:ty) => {
//         impl SinCos for $f<$f0> {
//             fn sin_cos(self) -> (Self, Self) {
//                 sin_cos_i7(self)
//             }
//             fn sin(self) -> Self {
//                 sin_cos_i7(self).0
//             }
//             fn cos(self) -> Self {
//                 sin_cos_i7(self).1
//             }
//             fn tan(self) -> Option<Self> {
//                 let (sin, cos) = sin_cos_i7(self);
//                 sin.checked_div(cos)
//             }
//         }
//         impl SinCos for $f<$f1> {
//             fn sin_cos(self) -> (Self, Self) {
//                 sin_cos_i8(self)
//             }
//             fn sin(self) -> Self {
//                 sin_cos_i8(self).0
//             }
//             fn cos(self) -> Self {
//                 sin_cos_i8(self).1
//             }
//             fn tan(self) -> Option<Self> {
//                 let (sin, cos) = sin_cos_i8(self);
//                 sin.checked_div(cos)
//             }
//         }
//         impl SinCos for $f<$f2> {
//             fn sin_cos(self) -> (Self, Self) {
//                 sin_cos_i9(self)
//             }
//             fn sin(self) -> Self {
//                 sin_cos_i9(self).0
//             }
//             fn cos(self) -> Self {
//                 sin_cos_i9(self).1
//             }
//             fn tan(self) -> Option<Self> {
//                 let (sin, cos) = sin_cos_i9(self);
//                 sin.checked_div(cos)
//             }
//         }
//     };
// }

impl_sincos_deg!(FixedI16, LeEqU16, U6);
impl_sincos_deg!(FixedI32, LeEqU32, U22);
impl_sincos_deg!(FixedI64, LeEqU64, U54);
impl_sincos_deg!(FixedI128, LeEqU128, U118);

// impl_sincos_deg_small!(FixedI16, U9, U8, U7);
// impl_sincos_deg_small!(FixedI32, U25, U24, U23);
// impl_sincos_deg_small!(FixedI64, U53, U54, U55);
// impl_sincos_deg_small!(FixedI128, U117, U118, U119);

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
    ($f:ident, $leq:ident, $f0:ty) => {
        impl<N> FixedRadians for $f<N>
        where
            N: $leq + IsLessOrEqual<$f0, Output = True>,
            N: $leq + IsLessOrEqual<Sum<$f0, U1>, Output = True>,
            N: $leq + IsLessOrEqual<Sum<$f0, U2>, Output = True>,
        {
            const FRAC_PI_2: Self = $f::FRAC_PI_2;
            const PI: Self = $f::PI;
            const TAU: Self = $f::TAU;
        }
    };
}

impl_rad_consts!(FixedI8, LeEqU8, U4);
impl_rad_consts!(FixedI16, LeEqU16, U12);
impl_rad_consts!(FixedI32, LeEqU32, U28);
impl_rad_consts!(FixedI64, LeEqU64, U60);
impl_rad_consts!(FixedI128, LeEqU128, U124);

impl SinCos for f32 {
    fn sin_cos(self) -> (Self, Self) {
        f32::sin_cos(self.to_radians())
    }

    fn sin(self) -> Self {
        f32::sin(self.to_radians())
    }

    fn cos(self) -> Self {
        f32::cos(self.to_radians())
    }

    fn tan(self) -> Option<Self> {
        Some(f32::tan(self.to_radians()))
    }
}

impl SinCos for f64 {
    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self.to_radians())
    }

    fn sin(self) -> Self {
        f64::sin(self.to_radians())
    }

    fn cos(self) -> Self {
        f64::cos(self.to_radians())
    }

    fn tan(self) -> Option<Self> {
        Some(f64::tan(self.to_radians()))
    }
}
