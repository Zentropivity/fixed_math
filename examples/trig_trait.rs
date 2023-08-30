use fixed::types::*;
use fixed_math::SinCos;

fn main() {
    let v = vec![
        I32F32::lit("360"),
        I32F32::lit("270"),
        I32F32::lit("180"),
        I32F32::lit("90"),
        I32F32::lit("45"),
        I32F32::lit("22.5"),
        I32F32::lit("0"),
        -I32F32::lit("22.5"),
        -I32F32::lit("45"),
        -I32F32::lit("90"),
        -I32F32::lit("180"),
        -I32F32::lit("270"),
        -I32F32::lit("360"),
    ];
    println!("I32F32 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, n.sin_cos());
    }

    let v = vec![
        I9F7::lit("90"),
        I9F7::lit("45"),
        I9F7::lit("22.5"),
        -I9F7::lit("90"),
        -I9F7::lit("45"),
        -I9F7::lit("22.5"),
    ];
    println!("I9F7 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, n.sin_cos());
    }

    let v = vec![
        I8F8::lit("90"),
        I8F8::lit("45"),
        I8F8::lit("22.5"),
        -I8F8::lit("90"),
        -I8F8::lit("45"),
        -I8F8::lit("22.5"),
    ];
    println!("I8F8 deg");
    for n in v {
        println!("sin_cos {} = {:?}", n, n.sin_cos());
    }

    //TODO fix sin_cos right_angles overflow at 90
    // let v = vec![
    //     I7F9::lit("45"),
    //     I7F9::lit("22.5"),
    //     -I7F9::lit("45"),
    //     -I7F9::lit("22.5"),
    // ];
    // println!("I7F9 deg");
    // for n in v {
    //     println!("sin_cos {} = {:?}", n, n.sin_cos());
    // }
}
