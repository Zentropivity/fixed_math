use fixed::types::*;
use fixed_math::*;

fn main() {
    let v = vec![
        // I32F32::PI,
        I32F32::from_num(30),
        I32F32::from_num(60),
        I32F32::from_num(90),
        // I32F32::MAX,
    ];
    println!("I32F32 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, sin_cos_unchecked(n));
    }
}
