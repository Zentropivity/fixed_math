//! Square root functions for fixed numbers.
//!
//! - `sqrt_u` : for unsigned numbers with 1 < integer bits
//! - `sqrt_u0` : for unsigned numbers with 0 integer bits
//! - `sqrt_u1` : for unsigned numbers with 1 integer bit
//! - `sqrt_i` : for signed numbers with 1 < integer bits
//! - `sqrt_i1` : for signed numbers with 1 integer bit
//! - _`sqrt_i0`_ : Does not exist, because sqrt of number can not be represented.
//!
//! Square root is not implemented for fixed numbers with 0 integer bits as it would almost always overflow.
//!
//! ### License
//!
//! Our sqrt function is a modified version of [cordic's](https://github.com/sebcrozet/cordic).
// The BSD 3-Clause license of cordic can be found at: third_party/LICENSE_CORDIC.

use fixed::traits::{FixedSigned, FixedUnsigned};

use crate::util::*;

/// Calculate square root of unsigned fixed number.
///
/// This implementation only handles numbers which can represent 1.
/// Use `sqrt_u1` for number with 1 integer bit.
/// Use `sqrt_u0` for number without integer bits.
#[inline]
pub fn sqrt_u<Val>(num: Val) -> Val
where
    Val: FixedUnsigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "can not take square root of negative number!"
    );
    debug_assert!(0 != Val::INT_BITS, "use `sqrt_u0` instead!");
    debug_assert!(
        0 < Val::INT_BITS,
        "cannot take square root of numbers without integer bits!"
    );
    debug_assert!(1 < Val::INT_BITS, "use `sqrt_u1` instead!");

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

/// Calculate square root of unsigned fixed number that has 1 integer bits.
#[inline]
pub fn sqrt_u1<Val>(num: Val) -> Val
where
    Val: FixedUnsigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        -1 < Val::INT_BITS,
        "not implemented for negative number of bits!"
    );
    debug_assert!(0 != Val::INT_BITS, "use `sqrt_u0` instead!");
    debug_assert!(
        1 == Val::INT_BITS,
        "use `sqrt` for numbers with more integer bits!"
    );

    if num == Val::ZERO {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    pow = Val::DELTA << Val::FRAC_BITS as u32; //TODO? maybe this could be a constant
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

/// Calculate square root of unsigned fixed number without integer bits.
#[inline]
pub fn sqrt_u0<Val>(num: Val) -> Val
where
    Val: FixedUnsigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "can not take square root of negative number!"
    );
    debug_assert!(
        -1 < Val::INT_BITS,
        "not implemented for negative number of bits!"
    );
    debug_assert!(0 == Val::INT_BITS, "use `sqrt_u` or `sqrt_u1` instead!");

    if num == Val::ZERO {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    pow = Val::DELTA << (Val::FRAC_BITS - 1) as u32; //TODO? maybe this could be a constant
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

/// Take square root of signed fixed number.
///
/// This implementation only handles numbers which can represent 1.
/// Use `sqrt_i1` for numbers with 1 integer bit.
#[inline]
pub fn sqrt_i<Val>(num: Val) -> Val
where
    Val: FixedSigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        -1 < Val::INT_BITS,
        "not implemented for negative number of bits!"
    );
    debug_assert!(
        0 < Val::INT_BITS,
        "cannot take square root of numbers without integer bits!"
    );
    debug_assert!(1 < Val::INT_BITS, "use `sqrt_i1` instead!");

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
        let mut n: u32 = Val::INT_BITS as u32 - 2;
        pow = fixed_one::<Val>();
        while if let Some(p) = pow.checked_mul(pow) {
            p <= num && n != 0u32
        } else {
            false
        } {
            //TODO bench against unsigned
            pow <<= 1u32;
            n -= 1u32;
        }
        if n == 0 {
            res = pow;
        } else {
            res = pow >> 1u32;
        }
    }

    for _ in 0..fixed_bits::<Val>() {
        pow >>= 1u32;
        if pow == Val::ZERO {
            //TODO bench -> keep or remove this if
            break;
        }
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

/// Calculate square root of signed fixed number that has 1 integer bit.
#[inline]
pub fn sqrt_i1<Val>(num: Val) -> Val
where
    Val: FixedSigned,
{
    debug_assert!(
        Val::ZERO <= num,
        "cannot take square root of negative number!"
    );
    debug_assert!(
        -1 < Val::INT_BITS,
        "not implemented for negative number of bits!"
    );
    debug_assert!(
        0 < Val::INT_BITS,
        "cannot take square root of numbers without integer bits!"
    );
    // Note: if they can represent 1, then they can represent > 1
    // we do not handle that in this function
    debug_assert!(
        1 == Val::INT_BITS,
        "use `sqrt` for numbers with more integer bits!"
    );

    if num == Val::ZERO {
        return num;
    }

    let mut pow: Val;
    let mut res: Val;

    pow = Val::DELTA << Val::FRAC_BITS as u32 - 1; //TODO? maybe this could be a constant
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
