use core::time::Duration;
use criterion::{criterion_group, Criterion};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;

fn clear_texture(texture: &mut Texture) {
    let origin_buffer: [u8; 100000] = [0; 100000];
    texture
        .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (key, value) in origin_buffer.iter().enumerate() {
                buffer[key] = *value;
            }
        })
        .unwrap();
}

fn sized_texture_referenceing(texture: &mut Texture) {
    let origin_buffer: [u8; 100000] = [1; 100000];
    texture
        .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for n in 0..origin_buffer.len() {
                buffer[n] = origin_buffer[n]
            }
        })
        .unwrap();
}

fn iteration_texture_referencing(texture: &mut Texture) {
    let origin_buffer: [u8; 100000] = [1; 100000];
    texture
        .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (key, value) in origin_buffer.iter().enumerate() {
                buffer[key] = *value;
            }
        })
        .unwrap();
}

fn texture_buffer_references(c: &mut Criterion) {
    let mut group = c.benchmark_group("texture_buffer_references");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fc2: render nes sprites", 0, 0)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())
        .unwrap();

    group.bench_function("sized_texture_referenceing", |b| {
        b.iter(|| sized_texture_referenceing(&mut texture))
    });

    clear_texture(&mut texture);
    group.bench_function("iteration_texture_referencing", |b| {
        b.iter(|| iteration_texture_referencing(&mut texture))
    });
}

fn short_warmup() -> Criterion {
    Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
        name = benches;
        config = short_warmup();
        targets =  texture_buffer_references
}
