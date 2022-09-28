use fixed::types::*;
use fixed_math::FixedSinCos;

fn main() {
    let v = vec![
        I32F32::from_num(90),
        I32F32::from_num(45),
        I32F32::from_num(22.5),
        -I32F32::from_num(90),
        -I32F32::from_num(45),
        -I32F32::from_num(22.5),
    ];
    println!("I32F32 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, n.sin_cos());
    }
}
