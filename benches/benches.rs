use criterion::{black_box, criterion_group, criterion_main, Criterion};

use correlation::spearmanr;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("spearmanr_small", |b| b.iter(|| {
        spearmanr(black_box(&(0..100).map(|i| -i as f64).collect()), black_box(&(0..100).map(|i| -i as f64).collect()));
    }));

    c.bench_function("spearmanr_large", |b| b.iter(|| {
        spearmanr(black_box(&(0..100000).map(|i| -i as f64).collect()), black_box(&(0..100000).map(|i| -i as f64).collect()));
    }));

    c.bench_function("spearmanr_huge", |b| b.iter(|| {
        spearmanr(black_box(&(0..10000000).map(|i| -i as f64).collect()), black_box(&(0..10000000).map(|i| -i as f64).collect()));
    }));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);