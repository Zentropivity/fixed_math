use fixed::types::*;
use fixed_math::sin_cos;

fn main() {
    let v = vec![
        // I32F32::from_num(2),
        // I32F32::PI,
        // I32F32::MAX,
        I32F32::from_num(30),
    ];
    println!("I32F32");
    for n in v {
        println!("sin_cos {} = {:?}", n, sin_cos(n));
    }

    // let v = vec![
    //     I32F96::from_num(2),
    //     I32F96::PI,
    //     I32F96::MAX,
    //     I32F96::from_num(30),
    // ];
    // println!("I32F96");
    // for n in v {
    //     println!("sin_cos {} = {:?}", n, sin_cos(n));
    // }
}
