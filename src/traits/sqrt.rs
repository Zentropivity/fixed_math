use fixed::traits::Fixed;
use fixed::types::extra::*;

pub trait FixedSqrt: Fixed {
    fn sqrt(&self) -> Self;
}

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
