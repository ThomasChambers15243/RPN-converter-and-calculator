use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rpn::{
    get_rpn_yard,
    get_rpn_tree,
    solve_numerical,
};

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.bench_function("Yard", |b| b.iter(|| get_rpn_yard(
        black_box("10+3*(5^2)-32.5"))));
    group.bench_function("Tree", |b| b.iter(|| get_rpn_tree(
        black_box("10+3*(5^2)-32.5"))));
    group.finish();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rpn", 
        |b| b.iter(|| get_rpn_yard(
            black_box("10+3*(5^2)-32.5"))));
        }

criterion_group!(benches, benchmarks);
criterion_main!(benches);