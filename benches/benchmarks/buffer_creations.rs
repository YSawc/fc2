use core::time::Duration;
use criterion::{criterion_group, Criterion};

fn create_buffer_with_index_origin() {
    let new_buffer: &mut [u8] = &mut [0; 1000000];
    let origin_buffer: [u8; 1000000] = [1; 1000000];

    for n in 0..origin_buffer.len() {
        new_buffer[n] = origin_buffer[n];
    }
}

fn create_buffer_with_clone() {
    let new_buffer: &mut [u8] = &mut [0; 1000000];
    let origin_buffer: [u8; 1000000] = [1; 1000000];

    for n in 0..origin_buffer.len() {
        new_buffer[n].clone_from(&origin_buffer[n]);
    }
}

fn compare_buffer_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_creation");

    group.bench_function("create_buffer_with_index_origin", |b| {
        b.iter(|| create_buffer_with_index_origin())
    });
    group.bench_function("create_buffer_with_clone", |b| {
        b.iter(|| create_buffer_with_clone())
    });
}

fn short_warmup() -> Criterion {
    Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
        name = benches;
        config = short_warmup();
        targets =  compare_buffer_creation
}
