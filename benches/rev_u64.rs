use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rev_bits::u64::reverse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse u64 4..12", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0a0b0c0d0), 4..12))
    });
    c.bench_function("reverse u64 5..13", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0a0b0c0d0), 5..13))
    });
    c.bench_function("reverse u64 8..48", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0a0b0c0d0), 8..48))
    });
    c.bench_function("reverse u64 9..49", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0a0b0c0d0), 9..49))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
