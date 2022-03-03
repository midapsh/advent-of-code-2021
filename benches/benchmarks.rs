use advent_of_code_2021::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1-1", |b| b.iter(p1_1));
    c.bench_function("1-2", |b| b.iter(p1_2));
    c.bench_function("2-1", |b| b.iter(p2_1));
    c.bench_function("2-2", |b| b.iter(p2_2));
    c.bench_function("3-1", |b| b.iter(p3_1));
    c.bench_function("3-2", |b| b.iter(p3_2));
    c.bench_function("4-1", |b| b.iter(p4_1));
    c.bench_function("4-2", |b| b.iter(p4_2));
    c.bench_function("5-1", |b| b.iter(p5_1));
    c.bench_function("5-2", |b| b.iter(p5_2));
    c.bench_function("6-1", |b| b.iter(p6_1));
    c.bench_function("6-2", |b| b.iter(p6_2));
    c.bench_function("7-1", |b| b.iter(p7_1));
    c.bench_function("7-2", |b| b.iter(p7_2));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
