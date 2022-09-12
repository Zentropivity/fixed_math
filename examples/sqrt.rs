use fixed::types::*;
use fixed_math::{sqrt, sqrt_i1, sqrt_u1};

fn main() {
    // let v = vec![
    //     I32F32::from_num(2),
    //     I32F32::PI,
    //     I32F32::MAX,
    //     I32F32::MAX >> 1u32,
    // ];
    // println!("I32F32");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![I1F31::from_num(0.2), I1F31::MAX, I1F31::MAX >> 1u32];
    // println!("I1F31");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_i1(n));
    // }

    // let v = vec![
    //     U32F32::from_num(2),
    //     U32F32::PI,
    //     U32F32::MAX,
    //     U32F32::MAX >> 1u32,
    // ];
    // println!("U32F32");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![
    //     U64F64::from_num(2),
    //     U64F64::PI,
    //     U64F64::MAX,
    //     U64F64::MAX >> 1u32,
    // ];
    // println!("U64F64");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![I1F127::from_num(0.5), I1F127::MAX, I1F127::MAX >> 1u32];
    // println!("I1F127");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_i1(n));
    // }

    // let v = vec![U1F127::from_num(0.5), U1F127::MAX, U1F127::MAX >> 1u32];
    // println!("U1F127");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_u1(n));
    // }

    // let v = vec![U1F31::from_num(0.5), U1F31::MAX, U1F31::MAX >> 1u32];
    // println!("U1F31");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_u1(n));
    // }

    // let v = vec![U3F29::from_num(0.5), U3F29::MAX, U3F29::MAX >> 1u32];
    // println!("U3F29");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![U1F7::from_num(0.5), U1F7::MAX, U1F7::MAX >> 1u32];
    // println!("U1F7");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_u1(n));
    // }

    // let v = vec![I1F7::from_num(0.5), I1F7::MAX, I1F7::MAX >> 1u32];
    // println!("I1F7");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt_i1(n));
    // }

    // let v = vec![U3F5::from_num(0.5), U3F5::MAX, U3F5::MAX >> 1u32];
    // println!("U3F5");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![U2F6::from_num(0.5), U2F6::MAX, U2F6::MAX >> 1u32];
    // println!("U2F6");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    let v = vec![I2F6::from_num(0.5), I2F6::MAX, I2F6::MAX >> 1u32];
    println!("I2F6");
    for n in v {
        println!("sqrt {} = {}", n, sqrt(n));
    }

    // let v = vec![I4F4::from_num(0.5), I4F4::MAX, I4F4::MAX >> 1u32];
    // println!("I4F4");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![I6F2::from_num(0.5), I6F2::MAX, I6F2::MAX >> 1u32];
    // println!("I6F2");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    let v = vec![I2F14::from_num(0.5), I2F14::MAX, I2F14::MAX >> 1u32];
    println!("I2F14");
    for n in v {
        println!("sqrt {} = {}", n, sqrt(n));
    }

    // let v = vec![I8F8::from_num(0.5), I8F8::MAX, I8F8::MAX >> 1u32];
    // println!("I8F8");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }

    // let v = vec![I14F2::from_num(0.5), I14F2::MAX, I14F2::MAX >> 1u32];
    // println!("I14F2");
    // for n in v {
    //     println!("sqrt {} = {}", n, sqrt(n));
    // }
}
