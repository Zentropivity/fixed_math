/// Conversion for values from degrees to radians.
pub trait DegreesToRadians {
    const FRAC_PI_180: Self;
    fn deg_to_rad(angle_degs: Self) -> Self;
}
/// Conversion for values from radians to degrees.
/// This may overflow and panic... Anyone needs a safe alternative?
pub trait RadiansToDegrees {
    const FRAC_180_PI: Self;
    fn rad_to_deg(angle_degs: Self) -> Self;
}

/// Marks a value type that can be converted between degrees and radians.
pub trait TrigConvertible: DegreesToRadians + RadiansToDegrees {}

impl<Val: DegreesToRadians + RadiansToDegrees> TrigConvertible for Val {}

impl DegreesToRadians for f32 {
    const FRAC_PI_180: Self = std::f32::consts::PI / 180f32;

    #[inline]
    fn deg_to_rad(angle_degs: Self) -> Self {
        angle_degs * Self::FRAC_PI_180
    }
}
impl RadiansToDegrees for f32 {
    const FRAC_180_PI: Self = 180f32 / std::f32::consts::PI;

    #[inline]
    fn rad_to_deg(angle_degs: Self) -> Self {
        angle_degs * Self::FRAC_PI_180
    }
}
impl DegreesToRadians for f64 {
    const FRAC_PI_180: Self = std::f64::consts::PI / 180f64;

    #[inline]
    fn deg_to_rad(angle_degs: Self) -> Self {
        angle_degs * Self::FRAC_PI_180
    }
}
impl RadiansToDegrees for f64 {
    const FRAC_180_PI: Self = 180f64 / std::f64::consts::PI;

    #[inline]
    fn rad_to_deg(angle_degs: Self) -> Self {
        angle_degs * Self::FRAC_PI_180
    }
}

/// Efficient conversion for fixed point numbers between degrees and radians
mod fixed_impl {
    use super::*;
    use fixed::types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8};
    use fixed::{
        FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
        FixedU8,
    };

    macro_rules! impl_fixed_conv {
        ($f:ident, $leq:ident) => {
            impl<N> DegreesToRadians for $f<N>
            where
                N: $leq,
            {
                const FRAC_PI_180: Self =
                    Self::lit("0.0174532925199432957692369076848861271344287188854172545609719144");

                #[inline]
                fn deg_to_rad(angle_degs: Self) -> Self {
                    angle_degs * Self::FRAC_PI_180
                }
            }
            impl<N> RadiansToDegrees for $f<N>
            where
                N: $leq,
            {
                const FRAC_180_PI: Self =
                    Self::lit("57.295779513082320876798154814105170332405472466564321549160243861");

                #[inline]
                fn rad_to_deg(angle_degs: Self) -> Self {
                    angle_degs * Self::FRAC_PI_180
                }
            }
        };
    }

    impl_fixed_conv!(FixedI8, LeEqU8);
    impl_fixed_conv!(FixedI16, LeEqU16);
    impl_fixed_conv!(FixedI32, LeEqU32);
    impl_fixed_conv!(FixedI64, LeEqU64);
    impl_fixed_conv!(FixedI128, LeEqU128);

    impl_fixed_conv!(FixedU8, LeEqU8);
    impl_fixed_conv!(FixedU16, LeEqU16);
    impl_fixed_conv!(FixedU32, LeEqU32);
    impl_fixed_conv!(FixedU64, LeEqU64);
    impl_fixed_conv!(FixedU128, LeEqU128);
}
