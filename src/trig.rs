//! Trigonometric functions for fixed numbers.
//!
//! ## Implementation
//!
//! We use the CORDIC algorithm, following this paper:
//! Efficient Fixed-Point Trigonometry Using CORDIC Functions For PIC16F - Jose Benavides Microchip Technology Inc.
//! (2007)
//! <https://ww1.microchip.com/downloads/en/AppNotes/01061A.pdf>

use fixed::{
    traits::{Fixed, FixedSigned},
    types::U1F127,
};

use crate::{
    tables::{ATAN_2P_DEG, ATAN_2P_RAD},
    traits::FixedRadians,
};

/// WolframAlpha: Product[1/Sqrt[1 + 2^-2j],{j,0,128}]
/// This is a constant used by the algorithm.
const KN: U1F127 = U1F127::lit(
    "0.60725293500888125616944675250492826311239085215008977245697601311014788120842578",
);

/// Setup traits for the cordic sincos.
/// Maps an angle to the interval of -90 to 90 degrees.
/// Normalization outputs a correct starting value and a bool meaning if the result will have to be negated.
pub(crate) mod normalization {
    use fixed::types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64};
    use fixed::{FixedI128, FixedI16, FixedI32, FixedI64, FixedI8};
    use seq_macro::seq;
    use typenum::*;

    /// Constant number 90.
    pub trait Fixed90 {
        const F_90: Self;
    }
    /// Constant number 180.
    pub trait Fixed180 {
        const F_180: Self;
    }
    /// Constant number 270.
    pub trait Fixed270 {
        const F_270: Self;
    }
    /// Constant number 360.
    pub trait Fixed360 {
        const F_360: Self;
    }

    /// Get the angle in the range where cordic can run on it also returning
    /// if the final result will need to be negated based on where it is on an interval.
    pub trait NormalizeCordic: Sized {
        fn normalize_cordic(angle_degs: Self) -> (Self, bool);
    }

    macro_rules! impl_f90 {
        ($f:ident, $leq:ident, $f0:ty) => {
            impl<N> Fixed90 for $f<N>
            where
                N: $leq + IsLess<$f0, Output = True>,
            {
                const F_90: Self = Self::lit("90");
            }
        };
    }

    impl Fixed90 for FixedI8<U0> {
        const F_90: Self = Self::lit("90");
    }

    impl_f90!(FixedI16, LeEqU16, U9);
    impl_f90!(FixedI32, LeEqU32, U25);
    impl_f90!(FixedI64, LeEqU64, U57);
    impl_f90!(FixedI128, LeEqU128, U121);

    macro_rules! impl_f180 {
        ($f:ident, $leq:ident, $f0:ty) => {
            impl<N> Fixed180 for $f<N>
            where
                N: $leq + IsLess<$f0, Output = True>,
            {
                const F_180: Self = Self::lit("180");
            }
        };
    }

    impl_f180!(FixedI16, LeEqU16, U8);
    impl_f180!(FixedI32, LeEqU32, U24);
    impl_f180!(FixedI64, LeEqU64, U56);
    impl_f180!(FixedI128, LeEqU128, U120);

    macro_rules! impl_f270plus {
        ($f:ident, $leq:ident, $f0:ty) => {
            impl<N> Fixed270 for $f<N>
            where
                N: $leq + IsLess<$f0, Output = True>,
            {
                const F_270: Self = Self::lit("270");
            }
            impl<N> Fixed360 for $f<N>
            where
                N: $leq + IsLess<$f0, Output = True>,
            {
                const F_360: Self = Self::lit("360");
            }
        };
    }

    impl_f270plus!(FixedI16, LeEqU16, U7);
    impl_f270plus!(FixedI32, LeEqU32, U23);
    impl_f270plus!(FixedI64, LeEqU64, U55);
    impl_f270plus!(FixedI128, LeEqU128, U119);

    macro_rules! impl_norm_small {
        ($f:ident, $frac:ident) => {
            impl NormalizeCordic for $f<$frac> {
                #[inline(always)]
                fn normalize_cordic(angle_degs: Self) -> (Self, bool) {
                    (angle_degs, false)
                }
            }
        };
    }

    seq!(N in 1..=8 {
        impl_norm_small!(FixedI8, U~N);
    });
    seq!(N in 9..=16 {
        impl_norm_small!(FixedI16, U~N);
    });
    seq!(N in 25..=32 {
        impl_norm_small!(FixedI32, U~N);
    });
    seq!(N in 57..=64 {
        impl_norm_small!(FixedI64, U~N);
    });
    seq!(N in 121..=128 {
        impl_norm_small!(FixedI128, U~N);
    });

    // Note: it would overflow at 180 so 90 has to be added twice
    macro_rules! impl_norm_i8 {
        ($f:ident, $frac:ident) => {
            impl NormalizeCordic for $f<$frac> {
                #[inline(always)]
                fn normalize_cordic(mut angle_degs: Self) -> (Self, bool) {
                    let mut neg = false;

                    if angle_degs < -Self::F_90 {
                        angle_degs += Self::F_90;
                        angle_degs += Self::F_90;
                        neg = true;
                    } else if Self::F_90 < angle_degs {
                        angle_degs -= Self::F_90;
                        angle_degs -= Self::F_90;
                        neg = true;
                    }
                    (angle_degs, neg)
                }
            }
        };
    }

    impl_norm_i8!(FixedI8, U0);
    impl_norm_i8!(FixedI16, U8);
    impl_norm_i8!(FixedI32, U24);
    impl_norm_i8!(FixedI64, U56);
    impl_norm_i8!(FixedI128, U120);

    macro_rules! impl_norm_i9 {
        ($f:ident, $frac:ident) => {
            impl NormalizeCordic for $f<$frac> {
                #[inline(always)]
                fn normalize_cordic(mut angle_degs: Self) -> (Self, bool) {
                    let mut neg = false;

                    if angle_degs < -Self::F_90 {
                        angle_degs += Self::F_180;
                        neg = true;
                    } else if Self::F_90 < angle_degs {
                        angle_degs -= Self::F_180;
                        neg = true;
                    }
                    (angle_degs, neg)
                }
            }
        };
    }

    impl_norm_i9!(FixedI16, U7);
    impl_norm_i9!(FixedI32, U23);
    impl_norm_i9!(FixedI64, U55);
    impl_norm_i9!(FixedI128, U119);

    // Normalize any bigger number
    macro_rules! impl_norm {
        ($f:ident, $frac:ident) => {
            impl NormalizeCordic for $f<$frac> {
                #[inline(always)]
                fn normalize_cordic(mut angle_degs: Self) -> (Self, bool) {
                    let mut neg = false;
                    angle_degs %= Self::F_360;

                    if angle_degs < -Self::F_90 {
                        if -Self::F_270 <= angle_degs {
                            angle_degs += Self::F_180;
                            neg = true;
                        } else {
                            angle_degs += Self::F_360;
                        }
                    } else if Self::F_90 < angle_degs {
                        if angle_degs <= Self::F_270 {
                            angle_degs -= Self::F_180;
                            neg = true;
                        } else {
                            angle_degs -= Self::F_360;
                        }
                    }
                    (angle_degs, neg)
                }
            }
        };
    }

    seq!(N in 0..=6 {
        impl_norm!(FixedI16, U~N);
    });
    seq!(N in 0..=22 {
        impl_norm!(FixedI32, U~N);
    });
    seq!(N in 0..=54 {
        impl_norm!(FixedI64, U~N);
    });
    seq!(N in 0..=118 {
        impl_norm!(FixedI128, U~N);
    });
}
pub use normalization::*;

/// Only works with values in -90 to 90 degrees.
#[inline]
pub fn sin_cos_deg_unchecked<Val: FixedSigned + NormalizeCordic>(
    mut angle_degs: Val,
) -> (Val, Val) {
    // debug_assert!(
    //     angle_degs <= Val::from_num(90) && Val::from_num(-90) <= angle_degs,
    //     "Can only take sin_cos of values in -90 to 90! Got: {}",
    //     angle_degs
    // );
    let mut x: Val = Val::from_fixed(KN);
    let mut y: Val = Val::ZERO;
    let mut at: Val = atan_table_deg::<Val>(0);
    let mut i = 0u32;
    while at != Val::ZERO {
        // println!(
        //     "i={i} x={}, y={}, z={}, atan(2^-i)={}",
        //     x,
        //     y,
        //     angle_degs,
        //     atan_table_deg::<Val>(i)
        // );
        let xx = x;
        if angle_degs < 0 {
            x += y >> i;
            y -= xx >> i;
            angle_degs += at;
        } else {
            x -= y >> i;
            y += xx >> i;
            angle_degs -= at;
        }

        i += 1;
        at = atan_table_deg::<Val>(i);
    }
    (y, x)
}

//TODO fix for small int bits, 90 overflows
#[inline]
fn sin_cos_right_angles<Val: FixedSigned + NormalizeCordic>(
    angle_normalized: Val,
    neg: bool,
) -> Option<(Val, Val)> {
    return match angle_normalized {
        angle_normalized if angle_normalized == Val::from_num(0) => {
            if neg {
                return Some((Val::from_num(0), Val::from_num(-1)));
            } else {
                return Some((Val::from_num(0), Val::from_num(1)));
            }
        }
        angle_normalized if angle_normalized == Val::from_num(90) => {
            if neg {
                return Some((Val::from_num(-1), Val::from_num(0)));
            } else {
                return Some((Val::from_num(1), Val::from_num(0)));
            }
        }
        angle_normalized if angle_normalized == Val::from_num(-90) => {
            if neg {
                return Some((Val::from_num(1), Val::from_num(0)));
            } else {
                return Some((Val::from_num(-1), Val::from_num(0)));
            }
        }
        _ => None,
    };
}

/// Compute sin and cos simultaneously of an angle in degrees.
///
/// Note: number representation needs at least 10 integer bits to fit 360.
#[inline]
pub fn sin_cos<Val: FixedSigned + NormalizeCordic>(angle_degs: Val) -> (Val, Val) {
    let (angle_normalized, neg) = Val::normalize_cordic(angle_degs);
    //TODO use constants instead of from_num
    // get values for right angles
    match sin_cos_right_angles(angle_normalized, neg) {
        Some(res) => return res,
        None => {}
    }
    let result = sin_cos_deg_unchecked(angle_normalized);
    if neg {
        (-result.0, -result.1)
    } else {
        result
    }
}

/// Compute sin of an angle in degrees.
///
/// Note: use sin_cos when you need both for performance.
#[inline]
pub fn sin<Val: FixedSigned + NormalizeCordic>(angle_degs: Val) -> Val {
    sin_cos(angle_degs).0
}

/// Compute cos of an angle in degrees.
///
/// Note: use sin_cos when you need both for performance.
#[inline]
pub fn cos<Val: FixedSigned + NormalizeCordic>(angle_degs: Val) -> Val {
    sin_cos(angle_degs).1
}

/// Calculate tangent of an angle in degrees.
/// Returns None when the cos is 0 or on overflow.
#[inline]
pub fn tan<Val: FixedSigned + NormalizeCordic>(angle_degs: Val) -> Option<Val> {
    let (sin, cos) = sin_cos(angle_degs);
    sin.checked_div(cos)
}

//TODO? explode the atan tables with macro to avoid from_fixed conversion here
/// Get atan(2^{-index}) in degrees.
#[inline]
fn atan_table_deg<Val>(index: u32) -> Val
where
    Val: Fixed,
{
    Val::from_fixed(ATAN_2P_DEG[index as usize])
}

/// Currently it is very imprecise.
/// You probably want to convert to degrees and use sin_cos instead.
#[inline]
pub fn sin_cos_rad<Val: FixedSigned + FixedRadians>(mut angle_rads: Val) -> (Val, Val) {
    let mut neg = false;
    // FIXME % is very imprecise here...
    angle_rads %= <Val as FixedRadians>::TAU;

    // FIXME FRAC_3_PI_2 should be a const; maybe even NEG_FRAC_3_PI_2
    let frac_3_pi_2 = <Val as FixedRadians>::PI + <Val as FixedRadians>::FRAC_PI_2;

    if angle_rads < -<Val as FixedRadians>::FRAC_PI_2 {
        if -frac_3_pi_2 <= angle_rads {
            angle_rads += <Val as FixedRadians>::PI;
            neg = true;
        } else {
            angle_rads += <Val as FixedRadians>::TAU;
        }
    } else if <Val as FixedRadians>::FRAC_PI_2 < angle_rads {
        if angle_rads <= frac_3_pi_2 {
            angle_rads -= <Val as FixedRadians>::PI;
            neg = true;
        } else {
            angle_rads -= <Val as FixedRadians>::TAU;
        }
    }
    let c_res = sin_cos_rad_unchecked(angle_rads);
    if neg {
        (-c_res.0, -c_res.1)
    } else {
        c_res
    }
}

/// Get atan(2^{-index}) in radians.
#[inline]
fn atan_table_rad<Val>(index: u32) -> Val
where
    Val: Fixed,
{
    Val::from_fixed(ATAN_2P_RAD[index as usize])
}

/// Only works for values in -π/2 to π/2 radians
#[inline]
pub fn sin_cos_rad_unchecked<Val: FixedSigned + FixedRadians>(mut angle_rads: Val) -> (Val, Val) {
    debug_assert!(
        angle_rads <= <Val as FixedRadians>::FRAC_PI_2
            && -<Val as FixedRadians>::FRAC_PI_2 <= angle_rads,
        "Can only take sin_cos of values in -π/2 to π/2 (~ 1.57079)! Got: {}",
        angle_rads
    );
    let mut x: Val = Val::from_fixed(KN);
    let mut y: Val = Val::ZERO;
    let mut at: Val = atan_table_rad::<Val>(0);
    let mut i = 0u32;
    while at != Val::ZERO {
        // println!(
        //     "i={i} x={}, y={}, z={}, atan(2^-i)={}",
        //     x,
        //     y,
        //     angle_rads,
        //     atan_table_rad::<Val>(i)
        // );
        let xx = x;
        if angle_rads < 0 {
            x += y >> i;
            y -= xx >> i;
            angle_rads += at;
        } else {
            x -= y >> i;
            y += xx >> i;
            angle_rads -= at;
        }

        i += 1;
        at = atan_table_rad::<Val>(i);
    }
    (y, x)
}
