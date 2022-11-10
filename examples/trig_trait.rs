use fixed::types::*;
use fixed_math::SinCos;

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

    // let v = vec![
    //     I9F7::from_num(90),
    //     I9F7::from_num(45),
    //     I9F7::from_num(22.5),
    //     -I9F7::from_num(90),
    //     -I9F7::from_num(45),
    //     -I9F7::from_num(22.5),
    // ];
    // println!("I9F7 deg");
    // for n in v {
    //     println!("sin_cos {} = {:?}", n, n.sin_cos());
    // }

    // let v = vec![
    //     I8F8::from_num(90),
    //     I8F8::from_num(45),
    //     I8F8::from_num(22.5),
    //     -I8F8::from_num(90),
    //     -I8F8::from_num(45),
    //     -I8F8::from_num(22.5),
    // ];
    // println!("I8F8 deg");
    // for n in v {
    //     println!("sin_cos {} = {:?}", n, n.sin_cos());
    // }

    // let v = vec![
    //     I7F9::from_num(45),
    //     I7F9::from_num(22.5),
    //     -I7F9::from_num(45),
    //     -I7F9::from_num(22.5),
    // ];
    // println!("I7F9 deg");
    // for n in v {
    //     println!("sin_cos {} = {:?}", n, n.sin_cos());
    // }
}
