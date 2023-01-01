use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::buffer_creations::benches,
    benchmarks::buffer_referencings::benches,
    benchmarks::texture_buffer_referencings::benches,
}
