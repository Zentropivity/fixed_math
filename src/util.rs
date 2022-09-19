use fixed::traits::{Fixed, FixedSigned};

pub(crate) const fn fixed_one<Val>() -> Val
where
    Val: Fixed,
{
    match Val::TRY_ONE {
        Some(one) => one,
        None => panic!("fixed number can not represent 1"),
    }
}

pub(crate) const fn fixed_neg_one<Val>() -> Val
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
    (Val::INT_BITS + Val::FRAC_BITS) as u32
}
