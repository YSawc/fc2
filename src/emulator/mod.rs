pub mod configure;
pub mod renderer;
use crate::nes::Sprites;

use crate::cpu::*;
use crate::emulator::configure::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use configure::{PPU_DRAW_LINE_CYCLE, TOTAL_LINE, VERTICAL_PIXEL};

use sdl2::rect::Rect;
use sdl2::render::Texture;
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

    pub fn run(&mut self, textures: &[Texture]) -> Result<(), String> {
        use crate::bus::Mapper;

        let cycle = self.cpu.ex_ope();
        self.ppu_cycle += (cycle * 3) as u16;
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VERTICAL_PIXEL {
                for n in 0..PLAYGROUND_WIDTH {
                    let i1 = self.drawing_line / 8;
                    let i2 = self.drawing_line % 8;
                    let sprite_idx = self.cpu.bus.ppu.map.name_table_00
                        [((i1 as usize) * PLAYGROUND_WIDTH as usize) + n as usize]
                        as usize;
                    let attr_idx = (0x23C0 + (n / 4) + (self.drawing_line as u32 / 32) * 8) as u16;
                    let attr_arr_idx = ((n / 2) % 2) | (((self.drawing_line as u32 / 16) % 2) << 1);
                    let background_pallets = self.cpu.bus.ppu.map.addr(attr_idx);
                    let background_pallete_idx = self.cpu.bus.ppu.map.background_table
                        [(background_pallets & (0x1 << attr_arr_idx)) as usize];

                    let background_texture = &textures[background_pallete_idx as usize];

                    self.canvas.copy(
                        background_texture,
                        None,
                        Rect::new(
                            (n * SQUARE_SIZE) as i32,
                            (self.drawing_line) as i32,
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ),
                    )?;

                    let sprite = &self.sprites[sprite_idx][i2 as usize];
                    if *sprite == [0; 8] {
                        continue;
                    }
                    for j in 0..8 {
                        let background_idx = sprite[j as usize];
                        let sprite_color_idx =
                            self.cpu.bus.ppu.map.background_table[background_idx as usize];
                        let square_texture = &textures[sprite_color_idx as usize];

                        self.canvas.copy(
                            square_texture,
                            None,
                            Rect::new(
                                (j + n * SQUARE_SIZE) as i32,
                                (self.drawing_line) as i32,
                                SQUARE_SIZE,
                                SQUARE_SIZE,
                            ),
                        )?;
                    }
                }
            }

            if self.drawing_line == TOTAL_LINE {
                self.canvas.present();
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
        }
        Ok(())
    }
}
