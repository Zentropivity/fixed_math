use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fixed::types::*;
use fixed_math::Sqrt;

pub fn sqrt_fixed_8(c: &mut Criterion) {
    let arr = vec![
        I4F4::lit("0"),
        I4F4::lit("0.123456789"),
        I4F4::lit("4"),
        I4F4::lit("13"),
        I4F4::MAX,
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sqrt_fixed_8", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sqrt());
            });
        });
    }
}

pub fn sqrt_fixed_16(c: &mut Criterion) {
    let arr = vec![
        I8F8::lit("0"),
        I8F8::lit("0.123456789"),
        I8F8::lit("4"),
        I8F8::lit("90"),
        I8F8::MAX,
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sqrt_fixed_16", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sqrt());
            });
        });
    }
}

pub fn sqrt_fixed_32(c: &mut Criterion) {
    let arr = vec![
        I16F16::lit("0"),
        I16F16::lit("0.123456789"),
        I16F16::lit("4"),
        I16F16::lit("1024"),
        I16F16::lit("90"),
        I16F16::MAX,
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sqrt_fixed_32", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sqrt());
            });
        });
    }
}

pub fn sqrt_fixed_64(c: &mut Criterion) {
    let arr = vec![
        I32F32::lit("0"),
        I32F32::lit("0.123456789"),
        I32F32::lit("4"),
        I32F32::lit("1024"),
        I32F32::lit("90"),
        I32F32::MAX,
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sqrt_fixed_64", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sqrt());
            });
        });
    }
}

pub fn sqrt_fixed_128(c: &mut Criterion) {
    let arr = vec![
        I64F64::lit("0"),
        I64F64::lit("0.123456789"),
        I64F64::lit("4"),
        I64F64::lit("1024"),
        I64F64::lit("90"),
        I64F64::MAX,
    ];
    for i in 0..arr.len() {
        c.bench_with_input(BenchmarkId::new("sqrt_fixed_128", i), &arr[i], |b, &s| {
            b.iter(|| {
                let _r = black_box(s.sqrt());
            });
        });
    }
}

criterion_group!(
    benches,
    sqrt_fixed_16,
    sqrt_fixed_32,
    sqrt_fixed_64,
    sqrt_fixed_128
);
criterion_main!(benches);
