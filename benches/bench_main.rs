use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::buffer_creations::benches,
}
