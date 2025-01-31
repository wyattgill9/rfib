use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rfib::{naive, memo, iterative, matmult};

fn naive(c: &mut Criterion) {
    c.bench_function("Naive fib(20):", |b| b.iter(|| naive::fib(black_box(20))));
}

fn memo(c: &mut Criterion) {
    c.bench_function("Memo fib(20):", |b| b.iter(|| memo::fib(black_box(20))));
}

fn iterative(c: &mut Criterion) {
    c.bench_function("Iterative fib(20):", |b| b.iter(|| iterative::fib(black_box(20))));
}

fn matmult(c: &mut Criterion) {
    c.bench_function("Matmult fib(20):", |b| b.iter(|| matmult::fib(black_box(20))));
}

criterion_group!(benches, naive, memo, iterative);
criterion_main!(benches);