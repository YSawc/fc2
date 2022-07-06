pub mod configure;
pub mod renderer;
use crate::nes::Sprites;

use crate::cpu::*;
use crate::emulator::configure::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use configure::{PPU_CLOCK_RATE_FOR_CPU, PPU_DRAW_LINE_CYCLE, TOTAL_LINE, VERTICAL_PIXEL};

use crate::emulator::renderer::*;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::Sdl;

pub struct Emulator {
    pub cpu: CPU,
    pub cpu_cycle: u16,
    pub ppu_cycle: u16,
    pub ppu_clock_sync: u8,
    pub drawing_line: u16,
    pub sprites: Sprites,
    pub sdl: Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let cpu = CPU::default();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "fc2: render nes sprites",
                SQUARE_SIZE * PLAYGROUND_WIDTH,
                SQUARE_SIZE * PLAYGROUND_HEIGHT,
            )
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

        Self {
            cpu,
            cpu_cycle: 0,
            ppu_cycle: 0,
            ppu_clock_sync: 0,
            drawing_line: 0,
            sprites: vec![],
            sdl: sdl_context,
            canvas,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) -> Result<(), String> {
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();

        let (square_texture1, square_texture2, square_texture3, square_texture4) =
            dummy_texture(&mut self.canvas, &texture_creator)?;

        let cycle = match self.ppu_clock_sync {
            PPU_CLOCK_RATE_FOR_CPU => {
                self.ppu_clock_sync = 0;
                self.cpu.ex_ope()
            }
            _ => 0,
        };
        self.cpu_cycle += cycle as u16;
        if self.cpu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.cpu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VERTICAL_PIXEL {
                for n in 0..PLAYGROUND_WIDTH {
                    let i1 = self.drawing_line / 8;
                    let i2 = self.drawing_line % 8;
                    let sprite_idx = self.cpu.bus.ppu.map.name_table_00
                        [((i1 as usize) * PLAYGROUND_WIDTH as usize) + n as usize]
                        as usize;
                    for j in 0..8 {
                        let square_texture = match self.sprites[sprite_idx][i2 as usize][j as usize]
                        {
                            0 => &square_texture1,
                            1 => &square_texture2,
                            2 => &square_texture3,
                            3 => &square_texture4,
                            _ => unreachable!(),
                        };

                        self.canvas.copy(
                            square_texture,
                            None,
                            Rect::new(
                                (j + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                                (self.drawing_line) as i32,
                                SQUARE_SIZE,
                                SQUARE_SIZE,
                            ),
                        )?;
                    }
                }
                self.canvas.present();
            }

            if self.drawing_line == TOTAL_LINE {
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
        }
        self.ppu_clock_sync += 1;

        Ok(())
    }
}
