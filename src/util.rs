use fixed::{
    traits::{Fixed, FixedSigned},
    types::{U0F128, U6F122},
};

pub const fn fixed_one<Val>() -> Val
where
    Val: Fixed,
{
    match Val::TRY_ONE {
        Some(one) => one,
        None => panic!("fixed number can not represent 1"),
    }
}

pub const fn fixed_neg_one<Val>() -> Val
where
    Val: FixedSigned,
{
    match Val::TRY_NEG_ONE {
        Some(one) => one,
        None => panic!("fixed number can not represent 1"),
    }
}

pub(crate) const fn fixed_bits<Val>() -> u32
where
    Val: Fixed,
{
    (Val::INT_NBITS + Val::FRAC_NBITS) as u32
}

// WolframAlpha: 180/PI
// 57.295779513082320876798154814105170332405472466564321549160243861...

/// `180 / PI`
/// used to convert radians to degrees
const FRAC_180_PI: U6F122 =
    U6F122::lit("57.295779513082320876798154814105170332405472466564321549160243861");

/// `PI / 180`
/// 0.0174532925199432957692369076848861271344287188854172545609719144...
/// used to convert degrees to radians
const FRAC_PI_180: U0F128 =
    U0F128::lit("0.0174532925199432957692369076848861271344287188854172545609719144");

/// Convert radians to degrees.
///
/// Note: this will overflow with Val=U6F122 and r > 1.*
pub fn rad_to_deg<Val>(r: Val) -> Val
where
    Val: Fixed,
{
    // 3.14.. rad = pi rad = 180
    // 1 rad = 180 / pi
    r * Val::from_fixed(FRAC_180_PI)
}

/// Convert degrees to radians
///
/// Note: this should never overflow
pub fn deg_to_rad<Val>(d: Val) -> Val
where
    Val: Fixed,
{
    // 1 deg = pi / 180
    d * Val::from_fixed(FRAC_PI_180)
}
