extern crate sdl2;

use crate::emurator::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
mod ppu {
    use crate::emurator::NES_FILE;
    use std::fs::File;
    use std::io::Read;

    #[derive(Clone)]
    pub struct PPU {
        // nes_header_size: u32,
        // chr_rom_size: u32,
        // prm_rom_size: u32,
        // charactor_rom_size: u32,
        // default_canvas_width: u32,
        pub sprites_num: u32,
        pub charactor_rom_start: u32,
    }

    impl PPU {
        pub fn new() -> Self {
            let mut f = File::open(NES_FILE).unwrap();
            let mut buffer = Vec::new();

            f.read_to_end(&mut buffer).unwrap();
            use std::str;
            if str::from_utf8(&buffer[0..3]) != Ok("NES") {
                panic!("File format is not nes!");
            } else {
                println!("NES file read!");
            }

            println!("PRG ROM size: {:?}", buffer[4]);
            println!("CHR ROM size: {:?}", buffer[5]);
            let prm_rom_size = buffer[4] as u32;
            let chr_rom_size = buffer[5] as u32;

            if buffer[5] == 0 {
                println!("Info: The board uses chr RAM!");
            }

            let nes_header_size = 0x0010;
            let program_rom_size = 0x4000;
            let charactor_rom_size = 0x2000;
            // let default_canvas_width = 800;
            let sprites_num = charactor_rom_size * chr_rom_size / 16;
            let charactor_rom_start = nes_header_size + prm_rom_size * program_rom_size;

            Self {
                // nes_header_size,
                // chr_rom_size,
                // prm_rom_size,
                // charactor_rom_size,
                // default_canvas_width,
                sprites_num,
                charactor_rom_start,
            }
        }

        pub fn read_sprites(self) -> Vec<Vec<Vec<u32>>> {
            let mut f = File::open(NES_FILE).unwrap();
            let mut buffer = Vec::new();
            let mut sprites = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            for n in 0..self.sprites_num {
                let mut sprite = vec![vec![0; 8]; 8];
                for i in 0..16 {
                    let val_str = buffer[((self.charactor_rom_start as u32) + n * 16 + i) as usize];
                    sprite[(i % 8) as usize] = sprite[(i % 8) as usize]
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, x)| {
                            x + format!("{:08b}", val_str)
                                .chars()
                                .nth(idx)
                                .unwrap()
                                .to_digit(2)
                                .unwrap()
                        })
                        .collect::<Vec<_>>();
                }
                sprites.push(sprite);
            }
            sprites
        }
    }
}

mod emurator {
    pub const SQUARE_SIZE: u32 = 8;
    pub const PLAYGROUND_WIDTH: u32 = 240;
    pub const PLAYGROUND_HEIGHT: u32 = 256;
    pub const NES_FILE: &str = "hello-world.nes";
}

pub mod monochrome_sheme {
    use sdl2::pixels::Color;
    pub fn turn_color(n: u32) -> Color {
        match n {
            3 => Color::RGB(255, 255, 255),
            2 => Color::RGB(170, 170, 170),
            1 => Color::RGB(85, 85, 85),
            0 => Color::RGB(0, 0, 0),
            _ => unimplemented!(),
        }
    }
}

pub fn main() -> Result<(), String> {
    let ppu = ppu::PPU::new();
    let sprites = ppu.to_owned().read_sprites();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "fc2: render nes sprites",
            SQUARE_SIZE * PLAYGROUND_WIDTH,
            SQUARE_SIZE * PLAYGROUND_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let (square_texture1, square_texture2, square_texture3, square_texture4) =
        dummy_texture(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;
    // let mut frame: u32 = 0;

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

        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.clear();

        for n in 0..ppu.sprites_num {
            for i in 0..8 {
                for j in 0..8 {
                    let square_texture = match sprites[n as usize][i as usize][j as usize] {
                        0 => &square_texture1,
                        1 => &square_texture2,
                        2 => &square_texture3,
                        3 => &square_texture4,
                        _ => unreachable!(),
                    };

                    canvas.copy(
                        square_texture,
                        None,
                        Rect::new(
                            (j + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            (i + (n / PLAYGROUND_HEIGHT) * SQUARE_SIZE) as i32,
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ),
                    )?;
                }
            }
        }

        canvas.present();
        // frame += 1;
    }

    Ok(())
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>), String> {
    enum TextureColor {
        Zero,
        One,
        Two,
        Three,
    }
    let mut square_texture1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture3 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture4 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;

    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Zero),
            (&mut square_texture2, TextureColor::One),
            (&mut square_texture3, TextureColor::Two),
            (&mut square_texture4, TextureColor::Three),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Zero => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::One => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(85, 85, 85));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Two => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(170, 170, 170));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Three => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(250, 250, 250));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                };
            })
            .map_err(|e| e.to_string())?;
    }
    Ok((
        square_texture1,
        square_texture2,
        square_texture3,
        square_texture4,
    ))
}
