use fixed::traits::Fixed;

pub const fn fixed_one<Val>() -> Val
where
    Val: Fixed,
{
    match Val::TRY_ONE {
        Some(one) => one,
        None => panic!("fixed number can not represent 1"),
    }
}

pub const fn fixed_bits<Val>() -> u32
where
    Val: Fixed,
{
    Val::INT_NBITS + Val::FRAC_NBITS
}
