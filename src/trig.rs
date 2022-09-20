//! Trigonometric functions for fixed numbers.
//!
//! ### License
//!
//! Our trigonometric functions are modified versions of [cordic's](https://github.com/sebcrozet/cordic).
// The BSD 3-Clause license of cordic can be found at: third_party/LICENSE_CORDIC.

use fixed::{
    traits::{Fixed, FixedSigned, FixedStrict},
    types::{I1F127, I2F126, U0F128},
};

use crate::{
    traits::FixedTrigConsts,
    util::{fixed_bits, fixed_neg_one, fixed_one},
};

const ATAN_TABLE: &[u8] = include_bytes!("tables/atan.table");

fn lookup_table(table: &[u8], index: u32) -> U0F128 {
    let i = index as usize * 16;
    U0F128::from_bits(u128::from_le_bytes(table[i..(i + 16)].try_into().unwrap()))
}

// fn atan_table(index: u32) -> U0F128 {
//     lookup_table(ATAN_TABLE, index)
// }

/// atan(2^{-i})
const ATAN_INV2: [I2F126; 4] = [
    // atan(1)
    I2F126::from_le_bytes([
        52, 7, 55, 224, 162, 152, 49, 49, 141, 48, 90, 136, 168, 246, 67, 50,
    ]),
    // atan(0.5)
    I2F126::from_le_bytes([
        160, 81, 135, 151, 189, 136, 252, 173, 104, 79, 187, 97, 5, 103, 172, 29,
    ]),
    // atan(0.25)
    I2F126::from_le_bytes([
        126, 33, 122, 95, 239, 121, 220, 86, 177, 110, 64, 150, 252, 186, 173, 15,
    ]),
    I2F126::from_le_bytes([
        120, 68, 244, 249, 196, 188, 68, 150, 113, 219, 11, 171, 166, 110, 245, 7,
    ]),
];

fn atan_table<Val>(index: u32) -> Val
where
    Val: FixedTrigConsts,
{
    Val::from_fixed(ATAN_INV2[index as usize])
}

// fn cordic_circular<Val: Fixed>(
//     mut x: Val,
//     mut y: Val,
//     mut z: Val,
//     vecmode: Val,
// ) -> (Val, Val, Val) {
//     let _0 = Val::ZERO;
//     let _2 = fixed_one::<Val>() + fixed_one::<Val>();

//     for i in 0..Val::FRAC_BITS as u32 {
//         let iu = i as u32;
//         if vecmode >= _0 && y < vecmode || vecmode < _0 && z >= _0 {
//             let x1 = x - (y >> iu);
//             y = y + (x >> iu);
//             x = x1;
//             z = z - Val::from_num(atan_table(i));
//         } else {
//             let x1 = x + (y >> iu);
//             y = y - (x >> iu);
//             x = x1;
//             z = z + Val::from_num(atan_table(i));
//         }
//     }

//     (x, y, z)
// }

// fn gain_cordic<Val: FixedSigned>() -> Val {
//     cordic_circular(
//         fixed_one::<Val>(),
//         Val::ZERO,
//         Val::ZERO,
//         fixed_neg_one::<Val>(),
//     )
//     .0
// }

//TODO docs

// pub fn sin_cos<Val>(mut angle: Val) -> (Val, Val)
// where
//     Val: FixedTrigConsts + FixedSigned + FixedStrict,
// {
//     let mut negative = false;

//     while angle > <Val as FixedTrigConsts>::FRAC_PI_2 {
//         angle -= <Val as FixedTrigConsts>::PI;
//         negative = !negative;
//     }

//     while angle < -<Val as FixedTrigConsts>::FRAC_PI_2 {
//         angle += <Val as FixedTrigConsts>::PI;
//         negative = !negative;
//     }

//     let inv_gain = fixed_one::<Val>() / gain_cordic::<Val>(); // FIXME: precompute this.
//     let res = cordic_circular(inv_gain, Val::ZERO, angle, fixed_neg_one::<Val>());

//     if negative {
//         (-res.1, -res.0)
//     } else {
//         (res.1, res.0)
//     }
// }

// WolframAlpha is amazing
// N[Product[1/Sqrt[1 + 2^(-2 i)], {i, 0, 128}], 160]
// 0.6072529350088812561694467525049282631123908521500897724569760131101478812084257809626778113264927754670382234713960540121013325132969687064052864022846666377675
const K_N: I1F127 = I1F127::from_le_bytes([
    117, 160, 254, 235, 149, 132, 252, 250, 51, 45, 175, 33, 212, 118, 186, 77,
]);
// 1/K_N
const A_N: I2F126 = I2F126::from_le_bytes([
    128, 135, 196, 206, 119, 155, 77, 240, 64, 52, 225, 62, 35, 133, 100, 105,
]);

fn rad_to_deg<Val>(r: Val) -> Val
where
    Val: FixedTrigConsts + FixedStrict,
{
    // 3.14.. rad = pi rad = 180
    // 1 rad = 180 / pi
    r * Val::from_num(180) / Val::PI
}

pub fn sin_cos<Val>(mut z: Val) -> (Val, Val)
where
    Val: FixedTrigConsts + FixedSigned + FixedStrict,
{
    let mut y = Val::ZERO;
    let mut x = Val::from_fixed(K_N);
    for i in 0..4 {
        //TODO fixed_bits::<Val>() {
        let xx = x;
        let yy = y;
        // let atan_p2 = rad_to_deg::<Val>(Val::from_fixed(atan_table::<Val>(i)));
        let atan_p2 = Val::from_fixed(atan_table::<Val>(i));
        println!("SC i={} x={}, y={}, z={}, atan_p2={}", i, x, y, z, atan_p2);
        if z < 0 {
            x = xx + (yy >> i);
            y = yy - (xx >> i);
            z = z + atan_p2;
        } else {
            x = xx - (yy >> i);
            y = yy + (xx >> i);
            z = z - atan_p2;
        }
    }
    (y, x)
}

// pub fn sin<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }
// pub fn cos<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }

// pub fn tan<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }

// pub fn asin<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }
// pub fn acos<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }
// pub fn atan<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }
// pub fn atan2<Val>(mut angle: Val) -> Val
// where
//     Val: FixedTrigConsts,
// {
//     todo!("impl")
// }
