use f128::f128;
use fixed::traits::{Fixed, FixedStrict};
use fixed::types::*;
use num_traits::Float;
use std::fs::File;
use std::io::Write;
use std::{env, process};

// Atan approximation algorithm from paper:
// https://mae.ufl.edu/~uhk/ARCTAN-APPROX-PAPER.pdf
fn f_a(a: f128) -> f128 {
    (f128::new(15159)
        + f128::new(147455) * a * a
        + f128::new(346345) * a * a * a * a
        + f128::new(225225) * a * a * a * a * a * a)
        / (f128::new(35)
            * (f128::new(35)
                + f128::new(1260) * a * a
                + f128::new(6930) * a * a * a * a
                + f128::new(12012) * a * a * a * a * a * a
                + f128::new(6435) * a * a * a * a * a * a * a * a))
}
fn atan(nsteps: i32, target: f128) -> f128 {
    let mut result;
    let mut best_result = f128::new(1);
    let mut a = f128::new(1);
    let mut lh_arctan = f128::PI / f128::new(4);
    let stepsize = f128::new(1) / f128::new(nsteps);
    let mut next_goal = a - stepsize / f128::new(2);

    if target != f128::new(0) {
        next_goal = target;
    }

    let mut gap = f128::new(1) / a - next_goal;
    let orig_da = stepsize * f128::new(0.1);
    let mut da = orig_da;

    for _ in 1..40000 {
        let aa = (a * (a + da) + f128::new(1)) / da;
        result = lh_arctan - aa * f_a(aa);
        a = a + da;

        if a.recip() - next_goal <= gap && gap > f128::new(1e-30) {
            let gap_0 = f128::new(1) / a;
            if gap_0 < next_goal {
                gap = next_goal - gap_0;
            } else {
                gap = gap_0 - next_goal;
            }
            da = gap.min(da);
            best_result = result;
        } else {
            if target == f128::new(0) {
                next_goal = next_goal - stepsize;
            }
            gap = f128::new(1);
            da = orig_da;
        }
        lh_arctan = result;
    }
    best_result
}
// fn f_a(a: f128) -> f128 {
//     (f128::new(15159)
//         + f128::new(147455) * a * a
//         + f128::new(346345) * a * a * a * a
//         + f128::new(225225) * a * a * a * a * a * a)
//         / (f128::new(35)
//             * (f128::new(35)
//                 + f128::new(1260) * a * a
//                 + f128::new(6930) * a * a * a * a
//                 + f128::new(12012) * a * a * a * a * a * a
//                 + f128::new(6435) * a * a * a * a * a * a * a * a))
// }
// fn atan(nsteps: i32, target: f128) -> f128 {
//     let mut result;
//     let mut best_result = f128::new(1);
//     let mut a = f128::new(1);
//     let mut lh_arctan = f128::PI / f128::new(4);
//     let stepsize = f128::new(1) / f128::new(nsteps);
//     let mut next_goal = a - stepsize / f128::new(2);

//     if target != f128::new(0) {
//         next_goal = target;
//     }

//     let mut gap = f128::new(1) / a - next_goal;
//     let orig_da = stepsize * f128::new(0.1);
//     let mut da = orig_da;

//     for _ in 1..40000 {
//         let aa = (a * (a + da) + f128::new(1)) / da;
//         result = lh_arctan - aa * f_a(aa);
//         a = a + da;

//         if a.recip() - next_goal <= gap && gap > f128::new(1e-100) {
//             let gap_0 = f128::new(1) / a;
//             if gap_0 < next_goal {
//                 gap = next_goal - gap_0;
//             } else {
//                 gap = gap_0 - next_goal;
//             }
//             da = gap.min(da);
//             best_result = result;
//         } else {
//             if target == f128::new(0) {
//                 next_goal = next_goal - stepsize;
//             }
//             gap = f128::new(1);
//             da = orig_da;
//         }
//         lh_arctan = result;
//     }
//     best_result
// }

// Note: in deg we only need 6 int bits -> U6F122::MAX ~ 63.99
//       but to be able to convert between deg/rad we need 8 int bits
//       but with 8 int bits, conversion can overflow if

// WolframAlpha: 180/PI
// 57.295779513082320876798154814105170332405472466564321549160243861...

const FRAC_180_PI: U6F122 = U6F122::from_le_bytes([
    215, 87, 210, 64, 127, 83, 151, 10, 195, 189, 15, 30, 211, 224, 46, 229,
]);

const FRAC_PI_180: U0F128 = U0F128::from_le_bytes([
    39, 46, 164, 116, 179, 47, 118, 112, 69, 78, 167, 148, 168, 209, 119, 4,
]);

// this will overflow with Val=U6F122 and r > 1.*
fn rad_to_deg<Val>(r: Val) -> Val
where
    Val: Fixed,
{
    // 3.14.. rad = pi rad = 180
    // 1 rad = 180 / pi
    r * Val::from_fixed(FRAC_180_PI)
}

// this should never overflow
fn deg_to_rad<Val>(d: Val) -> Val
where
    Val: Fixed,
{
    // 1 deg = pi / 180
    d * Val::from_fixed(FRAC_PI_180)
}

fn build_cordic_atan_table(num_entries: u8) -> Vec<u128> {
    let mut table = Vec::with_capacity(num_entries as usize);
    let mut angle: f128 = f128::new(1.0);
    let step: f128 = f128::new(0.5);

    for i in 0..num_entries {
        let atan = angle.atan();
        println!("A i={}, angle={}, atan={}", i, angle, atan);
        //atan(10000, angle);
        //f128.inner() is be aligned
        let ival = U0F128::from_bits(u128::from_be_bytes(atan.inner()));
        table.push(ival.to_bits());
        angle = angle * step;
    }

    table
}

fn vec_u128_to_le_bytes(data: Vec<u128>) -> Vec<u8> {
    let mut bytes = vec![];
    data.into_iter()
        .for_each(|d| bytes.extend(&d.to_le_bytes()));
    bytes
}

fn write_table(table_filepath: &str, num_bits: Option<u8>, data: Vec<u8>) {
    let mut table_file = File::create(table_filepath).expect("unable to open file");
    if let Some(num_bits) = num_bits {
        table_file
            .write_all(&[num_bits])
            .expect("unable to write to file");
    }
    table_file
        .write_all(&data)
        .expect("unable to write to file");
}

fn print_usage(error: Option<&str>) {
    if let Some(error) = error {
        eprintln!("Error: {}", error);
    }
    println!("usage: tablegen {{atan}} table_filepath");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage(None);
        process::exit(0);
    }
    match args[1].as_str() {
        "atan" => {
            let table = build_cordic_atan_table(128);
            let data = vec_u128_to_le_bytes(table);
            write_table(args[2].as_str(), None, data);
        }
        _ => {
            print_usage(Some("Invalid table type"));
            process::exit(1)
        }
    }
}
