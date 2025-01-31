use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rfib::naive;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Naive fib(20):", |b| b.iter(|| naive::fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);