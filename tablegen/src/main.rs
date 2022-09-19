use f128::f128;
use fixed::types::U0F128;
use num_traits::Float;
use std::fs::File;
use std::io::Write;
use std::{env, process};

fn build_cordic_atan_table(num_entries: u8) -> Vec<u128> {
    let mut table = Vec::with_capacity(num_entries as usize);
    let mut angle: f128 = f128::new(1.0);
    let step: f128 = f128::new(0.5);

    for _ in 0..num_entries {
        let atan = angle.atan();
        //TODO test: is this correct byte alignment from 'atan.inner()'
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
