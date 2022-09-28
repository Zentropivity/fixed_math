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
        c.bench_with_input(BenchmarkId::new("sincos_in_range", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sin_cos());
            });
        });
    }
}

pub fn sin_cos_out_range_pos(c: &mut Criterion) {
    let mut arr: Vec<I32F32> = Vec::new();
    for i in 0..4 {
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
    for i in 0..4 {
        arr.push(half + I32F32::from_num(i) * I32F32::from_num(180) + I32F32::from_num(22.5));
    }

    for i in 0..arr.len() {
        c.bench_with_input(
            BenchmarkId::new("sincos_out_range_pos", i),
            &arr[i],
            |b, &s| {
                b.iter(|| {
                    let _r = black_box(s.sin_cos());
                });
            },
        );
    }
}

criterion_group!(benches, sin_cos_in_range, sin_cos_out_range_pos);
criterion_main!(benches);
