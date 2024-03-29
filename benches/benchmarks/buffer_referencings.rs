use core::time::Duration;
use criterion::{criterion_group, Criterion};

fn sized_referenceing() {
    let new_buffer: &mut [u8] = &mut [0; 1000000];
    let origin_buffer: [u8; 1000000] = [1; 1000000];

    for n in 0..origin_buffer.len() {
        new_buffer[n] = origin_buffer[n];
    }
}

fn iteration_referencing() {
    let new_buffer: &mut [u8] = &mut [0; 1000000];
    let origin_buffer: [u8; 1000000] = [1; 1000000];

    for (key, value) in origin_buffer.iter().enumerate() {
        new_buffer[key] = *value;
    }
}

fn buffer_references(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_references");

    group.bench_function("sized_referenceing", |b| b.iter(|| sized_referenceing()));
    group.bench_function("iteration_referencing", |b| {
        b.iter(|| iteration_referencing())
    });
}

fn short_warmup() -> Criterion {
    Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
        name = benches;
        config = short_warmup();
        targets =  buffer_references
}
