use fixed::{
    traits::Fixed, FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32,
    FixedU64, FixedU8,
};
use seq_macro::seq;

/// There are requirements for certain constants:
///
/// - FRAC_PI_2 needs 2 int bits
/// - PI needs 3 int bits
///
/// => for a fixed number with N bits,
///    we can have at most N - 3 fractional bits for constants to be representable
pub trait FixedTrigConsts: Fixed {
    const PI: Self;
    const FRAC_PI_2: Self;
}

macro_rules! impl_trig_consts {
    ($f:ident, $f0:literal) => {
        seq!(FRAC in 0..=$f0 {
            impl FixedTrigConsts for $f<FRAC> {
                const PI: Self = Self::PI;
                const FRAC_PI_2: Self = Self::FRAC_PI_2;
            }
        });
    };
}

impl_trig_consts!(FixedI8, 5);
impl_trig_consts!(FixedI16, 13);
impl_trig_consts!(FixedI32, 29);
impl_trig_consts!(FixedI64, 61);
impl_trig_consts!(FixedI128, 125);

impl_trig_consts!(FixedU8, 5);
impl_trig_consts!(FixedU16, 13);
impl_trig_consts!(FixedU32, 29);
impl_trig_consts!(FixedU64, 61);
impl_trig_consts!(FixedU128, 125);
