extern crate sdl2;

use fc2::emulator::configure::{PLAYGROUND_WIDTH, SQUARE_SIZE};
use fc2::emulator::texture::*;
use fc2::emulator::*;
use fc2::nes::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::env;

pub fn main() -> Result<(), String> {
    let nes = Nes::new();
    let mut emulator = Emulator::default();
    emulator.cpu.init(&nes);
    // println!("{:#?}", nes);
    // println!("{:#?}", emulator.cpu);
    emulator.reset();

    emulator.sprites = nes.read_sprites();
    emulator.canvas.set_draw_color(Color::RGB(255, 0, 0));
    emulator.canvas.clear();
    emulator.canvas.present();

    let mut event_pump = emulator.sdl.event_pump()?;
    let texture_creator: TextureCreator<_> = emulator.canvas.texture_creator();

    let args: Vec<String> = env::args().collect();
    if &args[1] == "show_sprites" {
        let (square_texture1, square_texture2, square_texture3, square_texture4) =
            dummy_texture(&mut emulator.canvas, &texture_creator)?;

        for n in 0..nes.header.info.sprites_num {
            for i in 0..8 {
                for j in 0..8 {
                    let square_texture = match emulator.sprites[n as usize][i as usize][j as usize]
                    {
                        0 => &square_texture1,
                        1 => &square_texture2,
                        2 => &square_texture3,
                        3 => &square_texture4,
                        _ => unreachable!(),
                    };

                    emulator.canvas.copy(
                        square_texture,
                        None,
                        Rect::new(
                            (j + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            (i + (n / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ),
                    )?;
                }
            }
        }
        emulator.canvas.present();

        'show_sprites: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'show_sprites,
                    _ => {}
                }
            }
        }
        return Ok(());
    }

    let textures = texture_combine_builtin_colors(&mut emulator.canvas, &texture_creator)?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        emulator.run(&textures)?;
    }

    Ok(())
}
