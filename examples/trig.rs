use fixed::types::*;
use fixed_math::*;

fn main() {
    //TODO more
    let v = vec![
        I32F32::MIN >> 1,
        I32F32::MAX >> 1,
        I32F32::MIN,
        I32F32::MAX,
        I32F32::from_num(270),
        I32F32::from_num((I32F32::MIN >> 8) * I32F32::from_num(180) / I32F32::PI),
    ];
    println!("I32F32 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, sin_cos(n));
    }
}
