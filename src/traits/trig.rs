use crate::trig::*;
use fixed::{
    traits::FixedSigned,
    types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8},
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8,
};

use typenum::{IsLessOrEqual, Sum, True, U1, U12, U121, U124, U2, U25, U28, U4, U57, U60, U9};

/// Calculation of sine, cosine and tangent for number in **degrees**.
pub trait SinCos
where
    Self: Sized,
{
    /// Simultaneously calculate sine and cosine, returns `(sin(self), cos(self))`.
    fn sin_cos(self) -> (Self, Self);
    /// Calculate sine of an angle.
    /// `sin_cos` should be used instead when both sine and cosine are needed.
    fn sin(self) -> Self;
    /// Calculate cosinus on an angle.
    /// `sin_cos` should be used instead when both sine and cosine are needed.
    fn cos(self) -> Self;
    /// Calculate tangent of an angle if it exists.
    fn tan(self) -> Option<Self>;
}

macro_rules! impl_sincos_deg {
    ($f:ident, $leq:ident, $f0:ty) => {
        impl<N> SinCos for $f<N>
        where
            N: $leq + IsLessOrEqual<$f0, Output = True>,
            $f<N>: NormalizeCordic,
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

//TODO implement on sub-7 int bits too -> KN will need to be recalculated for each step down too...
// Note: right now the atan table has 45 as highest entry which would overflow on 6 int bits.
//       maybe skipping the highest entries in some way, the algorithm still works?
impl_sincos_deg!(FixedI8, LeEqU8, U1);
impl_sincos_deg!(FixedI16, LeEqU16, U9);
impl_sincos_deg!(FixedI32, LeEqU32, U25);
impl_sincos_deg!(FixedI64, LeEqU64, U57);
impl_sincos_deg!(FixedI128, LeEqU128, U121);

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
