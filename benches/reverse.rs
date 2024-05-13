use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rev_bits::u32::reverse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse 4..12", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0), 4..12))
    });
    c.bench_function("reverse 5..13", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0), 4..12))
    });
    c.bench_function("reverse 4..24", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0), 4..12))
    });
    c.bench_function("reverse 5..25", |b| {
        b.iter(|| reverse(black_box(0xa0b0c0d0), 4..12))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
