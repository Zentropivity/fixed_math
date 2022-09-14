//! Square root functions for fixed numbers.
//!
//! ### License
//!
//! Our sqrt function is a modified version of [cordic's sqrt](https://github.com/sebcrozet/cordic).
// The BSD 3-Clause license of cordic can [be found here](../third_party/LICENSE_CORDIC).

use fixed::traits::{Fixed, FixedSigned, FixedUnsigned};

use crate::util::*;

// pub trait FixedSqrt: Fixed {
//     fn sqrt(&self) -> Self;
// }

// macro_rules! impl_sqrt {
//     ($f:ident, $f0:ident) => {
//         impl<Frac> FixedSqrt for $f<Frac>
//         where
//             Frac: Unsigned + $f0,
//         {
//             #[inline]
//             fn sqrt(&self) -> Self {
//                 //TODO
//                 self.clone()
//             }
//         }
//     };
// }
// impl_sqrt!(FixedI8, LeEqU8);

// pub fn sqrt_unsigned<Val>(num: Val) -> Val
// where
//     Val: FixedSigned,
// {
//     //TODO use next_power_of_two somehow
//     num
// }
/// Take square root of fixed number.
///
/// This implementation only handles numbers which can represent the value 1.
///
pub fn sqrt_u<Val>(num: Val) -> Val
where
    Val: FixedUnsigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "can not take square root of negative number!"
    );
    debug_assert!(
        0u32 < Val::INT_NBITS,
        "cannot take square root of numbers without integer bits!"
    );
    debug_assert!(1u32 < Val::INT_NBITS, "use `sqrt_u1` instead!");

    if num == Val::ZERO || num == fixed_one::<Val>() {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    if num < fixed_one::<Val>() {
        pow = fixed_one::<Val>() >> 1u32; //TODO? maybe this could be a constant

        while num <= pow.unwrapped_mul(pow) {
            // Note: unwrapped_mul will not panic because pow > 0
            pow >>= 1u32;
        }

        res = pow;
    } else {
        // 1 < num
        pow = fixed_one::<Val>() << 1u32; //TODO maybe const?
        while if let Some(p) = pow.checked_mul(pow) {
            p <= num
        } else {
            false
        } {
            pow <<= 1u32;
        }

        res = pow >> 1u32;
    }

    for i in 0..fixed_bits::<Val>() {
        pow >>= 1u32;
        let next_res = res + pow;
        println!("# {i} {next_res}");
        debug_assert!(Val::ZERO <= next_res, "shit");
        if if let Some(nr_sq) = next_res.checked_mul(next_res) {
            nr_sq <= num
        } else {
            false
        } {
            res = next_res;
        }
    }
    res
}

pub fn sqrt_i<Val>(num: Val) -> Val
where
    Val: FixedSigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        0 < Val::INT_NBITS,
        "cannot take square root of numbers without integer bits!"
    );
    debug_assert!(1u32 < Val::INT_NBITS, "use `sqrt_i1` instead!");

    if num == Val::ZERO || num == fixed_one::<Val>() {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    if num < fixed_one::<Val>() {
        pow = fixed_one::<Val>() >> 1u32; //TODO? maybe this could be a constant

        while num <= pow.unwrapped_mul(pow) {
            // Note: unwrapped_mul will not panic because pow > 0
            pow >>= 1u32;
        }

        res = pow;
    } else {
        // 1 < num
        let mut n: u32 = Val::INT_NBITS - 2;
        pow = fixed_one::<Val>(); //TODO maybe const?
        while if let Some(p) = pow.checked_mul(pow) {
            p <= num && n != 0u32
        } else {
            false
        } {
            //TODO log
            //TODO bench
            pow <<= 1u32;
            n -= 1u32;
        }
        if n == 0 {
            res = pow;
        } else {
            res = pow >> 1u32;
        }
    }

    for i in 0..fixed_bits::<Val>() {
        pow >>= 1u32;
        let next_res = res + pow;
        println!("# {i} {next_res}");
        debug_assert!(Val::ZERO <= next_res, "shit");
        if if let Some(nr_sq) = next_res.checked_mul(next_res) {
            nr_sq <= num
        } else {
            false
        } {
            res = next_res;
        }
    }
    res
}

/// Take square root of fixed number that cannot represent 1.
pub fn sqrt_i1<Val>(num: Val) -> Val
where
    Val: FixedSigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        0 < Val::INT_NBITS,
        "cannot take square root of numbers without integer bits!"
    );
    // Note: if they can represent 1, then they can represent > 1
    // we do not handle that in this function
    debug_assert!(
        1 == Val::INT_NBITS,
        "use `sqrt` for numbers with more integer bits!"
    );

    if num == Val::ZERO {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    pow = Val::DELTA << Val::FRAC_NBITS - 1; //TODO? maybe this could be a constant
    while num <= pow.unwrapped_mul(pow) {
        pow >>= 1u32;
    }

    res = pow;

    for _ in 0..fixed_bits::<Val>() {
        pow >>= 1u32;
        let next_res = res + pow;
        if if let Some(nr_sq) = next_res.checked_mul(next_res) {
            nr_sq <= num
        } else {
            false
        } {
            res = next_res;
        }
    }
    res
}

/// Take square root of unsigned fixed number that has 1 integer bits.
pub fn sqrt_u1<Val>(num: Val) -> Val
where
    Val: FixedUnsigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        0 < Val::INT_NBITS,
        "cannot take square root of numbers without integer bits!"
    );
    debug_assert!(
        1 == Val::INT_NBITS,
        "use `sqrt` for numbers with more integer bits!"
    );

    if num == Val::ZERO {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    pow = Val::DELTA << Val::FRAC_NBITS; //TODO? maybe this could be a constant
    while num <= pow.unwrapped_mul(pow) {
        pow >>= 1u32;
    }

    res = pow;

    for _ in 0..fixed_bits::<Val>() {
        pow >>= 1u32;
        let next_res = res + pow;
        if if let Some(nr_sq) = next_res.checked_mul(next_res) {
            nr_sq <= num
        } else {
            false
        } {
            res = next_res;
        }
    }
    res
}
