//! Trigonometric functions for fixed numbers.
//!
//! ## Implementation
//!
//! We use the CORDIC algorithm,
//! following this paper: https://ww1.microchip.com/downloads/en/AppNotes/01061A.pdf

use fixed::{
    traits::{Fixed, FixedSigned, FixedStrict},
    types::U1F127,
};

use crate::{
    tables::{ATAN_2P_DEG, ATAN_2P_RAD},
    util::fixed_one,
    FixedRadians,
};

// WolframAlpha is nice
// this is constant used by the algorithm
// product[i in 0..=128] 1/(1 + sqrt(2^{-2i}))
const KN: U1F127 = U1F127::from_le_bytes([
    117, 160, 254, 235, 149, 132, 252, 250, 51, 45, 175, 33, 212, 118, 186, 77,
]);

/// Maps an angle to the interval of -90 to 90 degrees.
/// The bool means if it will have to be negated to get the correct value.
#[inline]
pub fn normalize_cordic<Val: FixedSigned>(mut angle_degs: Val) -> (Val, bool) {
    let mut neg = false;
    // FIXME these should be consts
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
pub fn sin_cos<Val: FixedSigned + FixedStrict>(mut angle_degs: Val) -> (Val, Val) {
    let (angle_normalized, neg) = normalize_cordic::<Val>(angle_degs);
    let result = sin_cos_deg_unchecked(angle_normalized);
    if neg {
        (-result.0, -result.1)
    } else {
        result
    }
}

/// Calculate tangent of an angle in degrees.
/// Returns None when the cos is 0 or on overflow.
#[inline]
pub fn tan<Val: FixedSigned + FixedStrict>(angle_degs: Val) -> Option<Val> {
    let (sin, cos) = sin_cos(angle_degs);
    sin.checked_div(cos)
}

/// Calculate arctangent of an angle in degrees.
/// This is only correct for angles between -90 and 90 (degrees).
#[inline]
pub fn atan_deg_unchecked<Val: FixedSigned + FixedStrict>(mut angle_degs: Val) -> Val {
    atan_div_deg_unchecked(angle_degs, fixed_one::<Val>())
}

/// Calculate arctangent of y/x.
/// This is only correct for angles between -90 and 90 (degrees).
#[inline]
pub fn atan_div_deg_unchecked<Val: FixedSigned + FixedStrict>(mut y: Val, mut x: Val) -> Val {
    let mut z: Val = Val::ZERO;
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
        if y < 0 {
            x -= y >> i;
            y += xx >> i;
            z -= at;
        } else {
            x -= y >> i;
            y += xx >> i;
            z -= at;
        }

        i += 1;
        at = atan_table_deg::<Val>(i);
    }
    z
}

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
