//! Trigonometric functions for fixed numbers.
//!
//! ### License
//!
//! Our trigonometric functions are modified versions of [cordic's](https://github.com/sebcrozet/cordic).
// The BSD 3-Clause license of cordic can be found at: third_party/LICENSE_CORDIC.

use fixed::{
    traits::{Fixed, FixedSigned, FixedStrict},
    types::U0F128,
};

use crate::{
    traits::FixedTrigConsts,
    util::{fixed_neg_one, fixed_one},
};

const ATAN_TABLE: &[u8] = include_bytes!("tables/atan.table");

fn lookup_table(table: &[u8], index: i32) -> U0F128 {
    let i = index as usize * 16;
    U0F128::from_bits(u128::from_le_bytes(table[i..(i + 16)].try_into().unwrap()))
}

fn cordic_circular<Val: Fixed>(
    mut x: Val,
    mut y: Val,
    mut z: Val,
    vecmode: Val,
) -> (Val, Val, Val) {
    let _0 = Val::ZERO;
    let _2 = fixed_one::<Val>() + fixed_one::<Val>();

    for i in 0..Val::FRAC_BITS {
        let iu = i as u32;
        if vecmode >= _0 && y < vecmode || vecmode < _0 && z >= _0 {
            let x1 = x - (y >> iu);
            y = y + (x >> iu);
            x = x1;
            z = z - Val::from_num(lookup_table(ATAN_TABLE, i));
        } else {
            let x1 = x + (y >> iu);
            y = y - (x >> iu);
            x = x1;
            z = z + Val::from_num(lookup_table(ATAN_TABLE, i));
        }
    }

    (x, y, z)
}

fn gain_cordic<Val: FixedSigned>() -> Val {
    cordic_circular(
        fixed_one::<Val>(),
        Val::ZERO,
        Val::ZERO,
        fixed_neg_one::<Val>(),
    )
    .0
}

//TODO docs

pub fn sin_cos<Val>(mut angle: Val) -> (Val, Val)
where
    Val: FixedTrigConsts + FixedSigned + FixedStrict,
{
    let mut negative = false;

    while angle > <Val as FixedTrigConsts>::FRAC_PI_2 {
        angle -= <Val as FixedTrigConsts>::PI;
        negative = !negative;
    }

    while angle < -<Val as FixedTrigConsts>::FRAC_PI_2 {
        angle += <Val as FixedTrigConsts>::PI;
        negative = !negative;
    }

    let inv_gain = fixed_one::<Val>() / gain_cordic::<Val>(); // FIXME: precompute this.
    let res = cordic_circular(inv_gain, Val::ZERO, angle, fixed_neg_one::<Val>());

    if negative {
        (-res.1, -res.0)
    } else {
        (res.1, res.0)
    }
}

pub fn sin<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}
pub fn cos<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}

pub fn tan<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}

pub fn asin<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}
pub fn acos<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}
pub fn atan<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}
pub fn atan2<Val>(mut angle: Val) -> Val
where
    Val: FixedTrigConsts,
{
    todo!("impl")
}
