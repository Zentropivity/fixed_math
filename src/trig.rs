//! Trigonometric functions for fixed numbers.
//!
//! ## Implementation
//!
//! We use the CORDIC algorithm,
//! following the notes of this paper: https://ww1.microchip.com/downloads/en/AppNotes/01061A.pdf

use fixed::{
    traits::{Fixed, FixedSigned, FixedStrict},
    types::U1F127,
};

use crate::tables::ATAN_2P_DEG;

// fn atan_table_rad<Val>(index: usize) -> Val
// where
//     Val: Fixed,
// {
//     Val::from_fixed(ATAN_2P_RAD[index])
// }

fn atan_table_deg<Val>(index: u32) -> Val
where
    Val: Fixed,
{
    Val::from_fixed(ATAN_2P_DEG[index as usize])
}

// WolframAlpha is nice
const KN: U1F127 = U1F127::from_le_bytes([
    117, 160, 254, 235, 149, 132, 252, 250, 51, 45, 175, 33, 212, 118, 186, 77,
]);

/// Only works with values in -90 to 90 degrees.
pub fn sin_cos_unchecked<Val: FixedSigned + FixedStrict>(mut z: Val) -> (Val, Val) {
    let mut x: Val = Val::from_fixed(KN);
    let mut y: Val = Val::ZERO;
    let mut at: Val = atan_table_deg::<Val>(0);
    let mut i = 0u32;
    while at != Val::ZERO {
        println!(
            "i={i} x={}, y={}, z={}, atan(2^-i)={}",
            x,
            y,
            z,
            atan_table_deg::<Val>(i)
        );
        let xx = x;
        if z < 0 {
            x += y >> i;
            y -= xx >> i;
            z += at;
        } else {
            x -= y >> i;
            y += xx >> i;
            z -= at;
        }

        i += 1;
        at = atan_table_deg::<Val>(i);
    }
    // for i in 0..fixed_bits::<Val>() as u32 {
    //     let xx = x;
    //     if z < 0 {
    //         x += y >> i;
    //         y -= xx >> i;
    //         z += atan_table_deg::<Val>(i);
    //     } else {
    //         x -= y >> i;
    //         y += xx >> i;
    //         z -= atan_table_deg::<Val>(i);
    //     }
    // }
    (y, x)
}
