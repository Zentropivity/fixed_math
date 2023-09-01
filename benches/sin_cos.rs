use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fixed::types::*;
use fixed_math::SinCos;

pub fn sin_cos_fixed_16_in(c: &mut Criterion) {
    let arr = vec![
        I10F6::lit("90"),
        I10F6::lit("45"),
        I10F6::lit("22.5"),
        -I10F6::lit("90"),
        -I10F6::lit("45"),
        -I10F6::lit("22.5"),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_16_in", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_16_out(c: &mut Criterion) {
    let mut arr: Vec<I10F6> = Vec::new();
    for i in 0..2 {
        arr.push(I10F6::lit("180") + I10F6::from_num(i) * I10F6::lit("180") + I10F6::lit("22.5"));
    }

    let mut half = I10F6::lit("180");
    while half < (I10F6::MAX >> 1) {
        half += I10F6::lit("180");
    }
    for i in 0..2 {
        arr.push(half + I10F6::from_num(i) * I10F6::lit("180") + I10F6::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_16_out", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_16_out_neg(c: &mut Criterion) {
    let mut arr: Vec<I10F6> = Vec::new();
    for i in 0..2 {
        arr.push(I10F6::lit("-180") - I10F6::from_num(i) * I10F6::lit("180") - I10F6::lit("22.5"));
    }

    let mut half = -I10F6::lit("180");
    while half < (I10F6::MIN << 1) {
        half -= I10F6::lit("180");
    }
    for i in 0..2 {
        arr.push(half - I10F6::from_num(i) * I10F6::lit("180") - I10F6::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_16_out_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_32_in(c: &mut Criterion) {
    let arr = vec![
        I16F16::lit("90"),
        I16F16::lit("45"),
        I16F16::lit("22.5"),
        -I16F16::lit("90"),
        -I16F16::lit("45"),
        -I16F16::lit("22.5"),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_32_in", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_32_out(c: &mut Criterion) {
    let mut arr: Vec<I16F16> = Vec::new();
    for i in 0..2 {
        arr.push(
            I16F16::lit("360") + I16F16::from_num(i) * I16F16::lit("180") + I16F16::lit("22.5"),
        );
    }

    let mut half = I16F16::lit("360");
    while half < (I16F16::MAX >> 1) {
        half += I16F16::lit("360");
    }
    for i in 0..2 {
        arr.push(half + I16F16::from_num(i) * I16F16::lit("180") + I16F16::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_32_out", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_32_out_neg(c: &mut Criterion) {
    let mut arr: Vec<I16F16> = Vec::new();
    for i in 0..2 {
        arr.push(
            I16F16::lit("-360") - I16F16::from_num(i) * I16F16::lit("180") - I16F16::lit("22.5"),
        );
    }

    let mut half = -I16F16::lit("360");
    while half < (I16F16::MIN << 1) {
        half -= I16F16::lit("360");
    }
    for i in 0..2 {
        arr.push(half - I16F16::from_num(i) * I16F16::lit("180") - I16F16::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_32_out_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_64_in(c: &mut Criterion) {
    let arr = vec![
        I32F32::lit("90"),
        I32F32::lit("45"),
        I32F32::lit("22.5"),
        -I32F32::lit("90"),
        -I32F32::lit("45"),
        -I32F32::lit("22.5"),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_64_in", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_64_out(c: &mut Criterion) {
    let mut arr: Vec<I32F32> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F32::lit("360") + I32F32::from_num(i) * I32F32::lit("180") + I32F32::lit("22.5"),
        );
    }

    // Note: this will not be the same range as in radians
    // (the about) half of I32F32 in radians is a lot bigger angle
    // but algorithmically its interesting to bench
    let mut half = I32F32::lit("360");
    while half < (I32F32::MAX >> 1) {
        half += I32F32::lit("360");
    }
    for i in 0..2 {
        arr.push(half + I32F32::from_num(i) * I32F32::lit("180") + I32F32::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_64_out", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_64_out_neg(c: &mut Criterion) {
    let mut arr: Vec<I32F32> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F32::lit("-360") - I32F32::from_num(i) * I32F32::lit("180") - I32F32::lit("22.5"),
        );
    }

    let mut half = -I32F32::lit("360");
    while half < (I32F32::MIN << 1) {
        half -= I32F32::lit("360");
    }
    for i in 0..2 {
        arr.push(half - I32F32::from_num(i) * I32F32::lit("180") - I32F32::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_64_out_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_128_in(c: &mut Criterion) {
    let arr = vec![
        I32F96::lit("90"),
        I32F96::lit("45"),
        I32F96::lit("22.5"),
        -I32F96::lit("90"),
        -I32F96::lit("45"),
        -I32F96::lit("22.5"),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_128_in", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_128_out(c: &mut Criterion) {
    let mut arr: Vec<I32F96> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F96::lit("360") + I32F96::from_num(i) * I32F96::lit("180") + I32F96::lit("22.5"),
        );
    }

    let mut half = I32F96::lit("360");
    while half < (I32F96::MAX >> 1) {
        half += I32F96::lit("360");
    }
    for i in 0..2 {
        arr.push(half + I32F96::from_num(i) * I32F96::lit("180") + I32F96::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_128_out", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_fixed_128_out_neg(c: &mut Criterion) {
    let mut arr: Vec<I32F96> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F96::lit("-360") - I32F96::from_num(i) * I32F96::lit("180") - I32F96::lit("22.5"),
        );
    }

    let mut half = -I32F96::lit("360");
    while half < (I32F96::MIN << 1) {
        half -= I32F96::lit("360");
    }
    for i in 0..2 {
        arr.push(half - I32F96::from_num(i) * I32F96::lit("180") - I32F96::lit("22.5"));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_fixed_128_out_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

criterion_group!(
    benches,
    sin_cos_fixed_16_in,
    sin_cos_fixed_16_out,
    sin_cos_fixed_16_out_neg,
    sin_cos_fixed_32_in,
    sin_cos_fixed_32_out,
    sin_cos_fixed_32_out_neg,
    sin_cos_fixed_64_in,
    sin_cos_fixed_64_out,
    sin_cos_fixed_64_out_neg,
    sin_cos_fixed_128_in,
    sin_cos_fixed_128_out,
    sin_cos_fixed_128_out_neg,
);
criterion_main!(benches);
