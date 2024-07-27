use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(unused)]
use rpn::{
    get_rpn_yard,
    get_rpn_tree,
    // Unused
    benchmark_setup::test,
    solve_numerical,
};


// Bench in to post conversions
pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.bench_function("Yard", |b| b.iter(|| get_rpn_yard(
        black_box("(x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23)"))));
    group.bench_function("Tree", |b| b.iter(|| get_rpn_tree(
        black_box("(x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23)"))));
    group.finish();
}

// Bench individual functions
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rpn", 
        |b| b.iter(|| get_rpn_yard(
            black_box("(x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23) * (x + 87.31)*(x-31.23)"))));
        }

criterion_group!(benches, benchmarks);
criterion_main!(benches);
