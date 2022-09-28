use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fixed::types::*;
use fixed_math::FixedSinCos;

pub fn sin_cos_in_range(c: &mut Criterion) {
    let arr = vec![
        I32F32::from_num(90),
        I32F32::from_num(45),
        I32F32::from_num(22.5),
        -I32F32::from_num(90),
        -I32F32::from_num(45),
        -I32F32::from_num(22.5),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sin_cos_in_range", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sin_cos());
            });
        });
    }
}

pub fn sin_cos_out_range_pos(c: &mut Criterion) {
    let mut arr: Vec<I32F32> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F32::from_num(360)
                + I32F32::from_num(i) * I32F32::from_num(180)
                + I32F32::from_num(22.5),
        );
    }

    // Note: this will not be the same range as in radians
    // (the about) half of I32F32 in radians is a lot bigger angle
    // but algorithmically its interesting to bench
    let mut half = I32F32::from_num(360);
    while half < (I32F32::MAX >> 1) {
        half += I32F32::from_num(360);
    }
    for i in 0..2 {
        arr.push(half + I32F32::from_num(i) * I32F32::from_num(180) + I32F32::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_out_range_pos", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_out_range_neg(c: &mut Criterion) {
    let mut arr: Vec<I32F32> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F32::from_num(-360)
                - I32F32::from_num(i) * I32F32::from_num(180)
                - I32F32::from_num(22.5),
        );
    }

    let mut half = -I32F32::from_num(360);
    while half < (I32F32::MIN << 1) {
        half -= I32F32::from_num(360);
    }
    for i in 0..2 {
        arr.push(half - I32F32::from_num(i) * I32F32::from_num(180) - I32F32::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_out_range_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_big_in_range(c: &mut Criterion) {
    let arr = vec![
        I32F96::from_num(90),
        I32F96::from_num(45),
        I32F96::from_num(22.5),
        -I32F96::from_num(90),
        -I32F96::from_num(45),
        -I32F96::from_num(22.5),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_big_in_range", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_big_out_range_pos(c: &mut Criterion) {
    let mut arr: Vec<I32F96> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F96::from_num(360)
                + I32F96::from_num(i) * I32F96::from_num(180)
                + I32F96::from_num(22.5),
        );
    }

    let mut half = I32F96::from_num(360);
    while half < (I32F96::MAX >> 1) {
        half += I32F96::from_num(360);
    }
    for i in 0..2 {
        arr.push(half + I32F96::from_num(i) * I32F96::from_num(180) + I32F96::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_big_out_range_pos", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_big_out_range_neg(c: &mut Criterion) {
    let mut arr: Vec<I32F96> = Vec::new();
    for i in 0..2 {
        arr.push(
            I32F96::from_num(-360)
                - I32F96::from_num(i) * I32F96::from_num(180)
                - I32F96::from_num(22.5),
        );
    }

    let mut half = -I32F96::from_num(360);
    while half < (I32F96::MIN << 1) {
        half -= I32F96::from_num(360);
    }
    for i in 0..2 {
        arr.push(half - I32F96::from_num(i) * I32F96::from_num(180) - I32F96::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_big_out_range_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_small_in_range(c: &mut Criterion) {
    let arr = vec![
        I16F16::from_num(90),
        I16F16::from_num(45),
        I16F16::from_num(22.5),
        -I16F16::from_num(90),
        -I16F16::from_num(45),
        -I16F16::from_num(22.5),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_small_in_range", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_small_out_range_pos(c: &mut Criterion) {
    let mut arr: Vec<I16F16> = Vec::new();
    for i in 0..2 {
        arr.push(
            I16F16::from_num(360)
                + I16F16::from_num(i) * I16F16::from_num(180)
                + I16F16::from_num(22.5),
        );
    }

    let mut half = I16F16::from_num(360);
    while half < (I16F16::MAX >> 1) {
        half += I16F16::from_num(360);
    }
    for i in 0..2 {
        arr.push(half + I16F16::from_num(i) * I16F16::from_num(180) + I16F16::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_small_out_range_pos", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_small_out_range_neg(c: &mut Criterion) {
    let mut arr: Vec<I16F16> = Vec::new();
    for i in 0..2 {
        arr.push(
            I16F16::from_num(-360)
                - I16F16::from_num(i) * I16F16::from_num(180)
                - I16F16::from_num(22.5),
        );
    }

    let mut half = -I16F16::from_num(360);
    while half < (I16F16::MIN << 1) {
        half -= I16F16::from_num(360);
    }
    for i in 0..2 {
        arr.push(half - I16F16::from_num(i) * I16F16::from_num(180) - I16F16::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_small_out_range_neg", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_xsmall_in_range(c: &mut Criterion) {
    let arr = vec![
        I10F6::from_num(90),
        I10F6::from_num(45),
        I10F6::from_num(22.5),
        -I10F6::from_num(90),
        -I10F6::from_num(45),
        -I10F6::from_num(22.5),
    ];
    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_xsmall_in_range", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_xsmall_out_range_pos(c: &mut Criterion) {
    let mut arr: Vec<I10F6> = Vec::new();
    for i in 0..2 {
        arr.push(
            I10F6::from_num(180)
                + I10F6::from_num(i) * I10F6::from_num(180)
                + I10F6::from_num(22.5),
        );
    }

    let mut half = I10F6::from_num(180);
    while half < (I10F6::MAX >> 1) {
        half += I10F6::from_num(180);
    }
    for i in 0..2 {
        arr.push(half + I10F6::from_num(i) * I10F6::from_num(180) + I10F6::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_xsmall_out_range_pos", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

pub fn sin_cos_xsmall_out_range_neg(c: &mut Criterion) {
    let mut arr: Vec<I10F6> = Vec::new();
    for i in 0..2 {
        arr.push(
            I10F6::from_num(-180)
                - I10F6::from_num(i) * I10F6::from_num(180)
                - I10F6::from_num(22.5),
        );
    }

    let mut half = -I10F6::from_num(180);
    while half < (I10F6::MIN << 1) {
        half -= I10F6::from_num(180);
    }
    for i in 0..2 {
        arr.push(half - I10F6::from_num(i) * I10F6::from_num(180) - I10F6::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sin_cos_xsmall_out_range_neg", i),
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
    sin_cos_in_range,
    sin_cos_out_range_pos,
    sin_cos_out_range_neg,
    sin_cos_big_in_range,
    sin_cos_big_out_range_pos,
    sin_cos_big_out_range_neg,
    sin_cos_small_in_range,
    sin_cos_small_out_range_pos,
    sin_cos_small_out_range_neg,
    sin_cos_xsmall_in_range,
    sin_cos_xsmall_out_range_pos,
    sin_cos_xsmall_out_range_neg,
);
criterion_main!(benches);
