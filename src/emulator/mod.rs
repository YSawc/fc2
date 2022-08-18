pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::configure::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use crate::nes::Sprites;
use configure::{PPU_DRAW_LINE_CYCLE, TOTAL_LINE, VBLANK_LINE, VERTICAL_PIXEL};

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

    pub fn inc_ppu_cycle(&mut self) {
        self.ppu_cycle += (self.cpu.cycle * 3) as u16;
        self.cpu.clear_cycle();
    }

    pub fn run(&mut self, textures: &[Texture]) -> Result<(), String> {
        self.cpu.ex_ope();
        self.inc_ppu_cycle();
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VERTICAL_PIXEL {
                self.draw_line(textures)?;
            }
            if self.drawing_line == TOTAL_LINE {
                self.canvas.present();
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
        } else if self.cpu.bus.ppu.register.ppu_ctrl.gen_nmi {
            if self.is_just_in_vblank_line() {
                self.cpu.interrupt(Interrupt::Nmi);
            } else if self.drawing_line == 0 {
                self.cpu.register.p.interrupt = false;
            }
        }
        Ok(())
    }

    pub fn draw_line(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH {
            let attr_idx = (0x23C0 + (n / 4) + (self.drawing_line as u32 / 32) * 8) as u16;
            let attr_arr_idx = ((n / 2) % 2) | (((self.drawing_line as u32 / 16) % 2) << 1);
            let background_pallets = self.cpu.bus.ppu.map.addr(attr_idx);
            // println!("self.cpu.bus.ppu.map.background_table: {:?}", self.cpu.bus.ppu.map.background_table);
            let background_pallete_idx = self.cpu.bus.ppu.map.background_table
                [(background_pallets & (0x1 << attr_arr_idx)) as usize];
            // if background_pallete_idx != 0 {
            //     println!("background_pallete_idx: {:?}", background_pallete_idx);
            // }
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

            let i1 = self.drawing_line / 8;
            let i2 = self.drawing_line % 8;
            let sprite_idx = self.cpu.bus.ppu.map.name_table_00
                [((i1 as usize) * PLAYGROUND_WIDTH as usize) + n as usize]
                as usize;
            let sprite = &self.sprites[sprite_idx][i2 as usize];
            // println!("sprite_idx: {:#?}, n: {:?}, i1: {:?}, i2: {:?}", sprite_idx, n, i1, i2);
            // println!("((i1 as usize) * PLAYGROUND_WIDTH as usize) + n as usize: {:#?}", ((i1 as usize) * PLAYGROUND_WIDTH as usize) + n as usize);
            // println!("{:?}", self.cpu.bus.ppu.map.name_table_00);
            if *sprite == [0; 8] {
                continue;
            }
            // println!("sprite_idx: {:#?}, i1: {:?}, i2: {:?}", sprite_idx, i1, i2);
            // println!("{:#?}", sprite);
            for j in 0..8 {
                let background_idx = sprite[j as usize];
                let sprite_color_idx =
                    self.cpu.bus.ppu.map.background_table[background_idx as usize];
                let square_texture = &textures[sprite_color_idx as usize];
                // println!("color_idx: {:?}", sprite_color_idx);
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
        Ok(())
    }

    pub fn is_just_in_vblank_line(&self) -> bool {
        self.drawing_line == VBLANK_LINE
    }
}
