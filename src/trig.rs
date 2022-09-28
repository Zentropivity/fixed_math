//! Trigonometric functions for fixed numbers.
//!
//! ## Implementation
//!
//! We use the CORDIC algorithm, following this paper:
//! Efficient Fixed-Point Trigonometry Using CORDIC Functions For PIC16F - Jose Benavides Microchip Technology Inc.
//! (2007)
//! https://ww1.microchip.com/downloads/en/AppNotes/01061A.pdf

use fixed::{
    traits::{Fixed, FixedSigned, FixedStrict},
    types::{U0F128, U1F127, U6F122},
};

use crate::{
    tables::{ATAN_2P_DEG, ATAN_2P_RAD},
    traits::FixedRadians,
};

// WolframAlpha is nice
/// This is a constant used by the algorithm.
/// product[i in 0..=128] 1/sqrt(1 + 2^{-2i})
const KN: U1F127 = U1F127::from_le_bytes([
    117, 160, 254, 235, 149, 132, 252, 250, 51, 45, 175, 33, 212, 118, 186, 77,
]);

/// 180 / π
/// used to convert from radians to degrees
pub const FRAC_180_PI: U6F122 = U6F122::from_le_bytes([
    215, 87, 210, 64, 127, 83, 151, 10, 195, 189, 15, 30, 211, 224, 46, 229,
]);

/// π / 180
/// used to convert from degrees to radians
pub const FRAC_PI_180: U0F128 = U0F128::from_le_bytes([
    39, 46, 164, 116, 179, 47, 118, 112, 69, 78, 167, 148, 168, 209, 119, 4,
]);

/// Simply convert a fixed number from radians to degrees by multiplication.
pub fn rad_to_deg<Val: Fixed>(angle_rads: Val) -> Val {
    angle_rads * Val::from_fixed(FRAC_180_PI)
}
/// Simply convert a fixed number from degrees to radians by multiplication.
pub fn deg_to_rad<Val: Fixed>(angle_degs: Val) -> Val {
    angle_degs * Val::from_fixed(FRAC_PI_180)
}

/// Works as a setup for sincos.
/// Maps an angle to the interval of -90 to 90 degrees.
/// Outputs a correct starting value and a bool meaning if the result will have to be negated.
#[inline(always)]
fn normalize_cordic<Val: FixedSigned>(mut angle_degs: Val) -> (Val, bool) {
    let mut neg = false;
    // FIXME these should be consts
    // TODO bench against Val::from_fixed
    let f_90 = Val::from_num(90);
    let f_180 = Val::from_num(180);
    let f_270 = Val::from_num(270);
    let f_360 = Val::from_num(360);

    angle_degs %= f_360;

    if angle_degs < -f_90 {
        if -f_270 <= angle_degs {
            angle_degs += f_180;
            neg = true;
        } else {
            angle_degs += f_360;
        }
    } else if f_90 < angle_degs {
        if angle_degs <= f_270 {
            angle_degs -= f_180;
            neg = true;
        } else {
            angle_degs -= f_360;
        }
    }
    (angle_degs, neg)
}

/// Compute sin and cos simultaneously of an angle in degrees.
///
/// Note: number representation needs at least 10 integer bits to fit 360.
#[inline]
pub fn sin_cos<Val: FixedSigned + FixedStrict>(angle_degs: Val) -> (Val, Val) {
    let (angle_normalized, neg) = normalize_cordic::<Val>(angle_degs);
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
pub fn sin<Val: FixedSigned + FixedStrict>(angle_degs: Val) -> Val {
    sin_cos(angle_degs).0
}

/// Compute cos of an angle in degrees.
///
/// Note: use sin_cos when you need both for performance.
#[inline]
pub fn cos<Val: FixedSigned + FixedStrict>(angle_degs: Val) -> Val {
    sin_cos(angle_degs).1
}

/// Calculate tangent of an angle in degrees.
/// Returns None when the cos is 0 or on overflow.
#[inline]
pub fn tan<Val: FixedSigned + FixedStrict>(angle_degs: Val) -> Option<Val> {
    let (sin, cos) = sin_cos(angle_degs);
    sin.checked_div(cos)
}

// /// Calculate arctangent of an angle in degrees.
// /// This is only correct for angles between -90 and 90 (degrees).
// #[inline]
// pub fn atan_deg_unchecked<Val: FixedSigned + FixedStrict>(mut angle_degs: Val) -> Val {
//     atan_div_deg_unchecked(angle_degs, fixed_one::<Val>())
// }

//TODO fix this. it should work... but it doesn't seem to and I do not need it right now
// /// Calculate arctangent of y/x.
// /// This is only correct for angles between -90 and 90 (degrees).
// #[inline]
// pub fn atan_div_deg_unchecked<Val: FixedSigned + FixedStrict>(mut y: Val, mut x: Val) -> Val {
//     let mut z: Val = Val::ZERO;
//     let mut at: Val = atan_table_deg::<Val>(0);
//     let mut i = 0u32;
//     while at != Val::ZERO {
//         // println!(
//         //     "i={i} x={}, y={}, z={}, atan(2^-i)={}",
//         //     x,
//         //     y,
//         //     angle_degs,
//         //     atan_table_deg::<Val>(i)
//         // );
//         let xx = x;
//         if y < 0 {
//             x -= y >> i;
//             y += xx >> i;
//             z -= at;
//         } else {
//             x -= y >> i;
//             y += xx >> i;
//             z -= at;
//         }

//         i += 1;
//         at = atan_table_deg::<Val>(i);
//     }
//     z
// }

/// Get atan(2^{-index}) in degrees.
#[inline]
fn atan_table_deg<Val>(index: u32) -> Val
where
    Val: Fixed,
{
    Val::from_fixed(ATAN_2P_DEG[index as usize])
}

/// Only works with values in -90 to 90 degrees.
#[inline]
pub fn sin_cos_deg_unchecked<Val: FixedSigned + FixedStrict>(mut angle_degs: Val) -> (Val, Val) {
    debug_assert!(
        angle_degs <= Val::from_num(90) && Val::from_num(-90) <= angle_degs,
        "Can only take sin_cos of values in -90 to 90! Got: {}",
        angle_degs
    );
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

/// Currently it is very imprecise.
/// You probably want to convert to degrees and use sin_cos instead.
#[inline]
pub fn sin_cos_rad<Val: FixedSigned + FixedStrict + FixedRadians>(
    mut angle_rads: Val,
) -> (Val, Val) {
    let mut neg = false;
    // FIXME % is very imprecise here...
    angle_rads %= Val::TAU;

    // FIXME FRAC_3_PI_2 should be a const; maybe even NEG_FRAC_3_PI_2
    let frac_3_pi_2 = Val::PI + Val::FRAC_PI_2;

    if angle_rads < -Val::FRAC_PI_2 {
        if -frac_3_pi_2 <= angle_rads {
            angle_rads += Val::PI;
            neg = true;
        } else {
            angle_rads += Val::TAU;
        }
    } else if Val::FRAC_PI_2 < angle_rads {
        if angle_rads <= frac_3_pi_2 {
            angle_rads -= Val::PI;
            neg = true;
        } else {
            angle_rads -= Val::TAU;
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
pub fn sin_cos_rad_unchecked<Val: FixedSigned + FixedStrict + FixedRadians>(
    mut angle_rads: Val,
) -> (Val, Val) {
    debug_assert!(
        angle_rads <= Val::FRAC_PI_2 && -Val::FRAC_PI_2 <= angle_rads,
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
