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

    let v = vec![
        I32F32::MIN >> 1,
        I32F32::MAX >> 1,
        I32F32::MIN,
        I32F32::MAX,
        I32F32::from_num(-15),
        I32F32::from_num(-25),
        I32F32::from_num(-45),
        I32F32::from_num(15),
        I32F32::from_num(25),
        I32F32::from_num(45),
        I32F32::from_num(90),
        I32F32::from_num(270),
        I32F32::from_num((I32F32::MIN >> 8) * I32F32::from_num(180) / I32F32::PI),
    ];
    for n in v {
        println!("tan {} = {:?}", n, tan(n));
    }

    let v = vec![
        //TODO fix
        I32F32::from_num(-0.4663076578),
        I32F32::from_num(0.466307657),
        I32F32::from_num(-0.9999999993),
        I32F32::from_num(0.9999999981),
    ];
    for n in v {
        println!("atan {} = {:?}", n, atan_deg_unchecked(n));
    }
}
