pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::configure::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use crate::ppu::mapper::Map;
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
            sdl: sdl_context,
            canvas,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self, textures: &[Texture]) -> Result<(), String> {
        let cycle = self.cpu.ex_ope();
        self.ppu_cycle += (cycle * 3) as u16;
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

    pub fn ppu_map(&mut self) -> &Map {
        &self.cpu.bus.ppu.map
    }

    pub fn draw_backgraund_line(
        &mut self,
        textures: &[Texture],
        x: u16,
        y: u16,
    ) -> Result<(), String> {
        let attr_idx = (0x23C0 + (x / 4) + (y / 32) * 8) as u16;
        let attr_arr_idx = ((x / 2) % 2) | (((y / 16) % 2) << 1);
        let pallets = self.ppu_map().addr(attr_idx);
        let pallete_idx = self
            .ppu_map()
            .addr(0x3F00 + (pallets & (0x1 << attr_arr_idx)) as u16);
        let texture = &textures[pallete_idx as usize];

        self.canvas.copy(
            texture,
            None,
            Rect::new(
                (x as u32 * SQUARE_SIZE) as i32,
                (self.drawing_line) as i32,
                SQUARE_SIZE,
                SQUARE_SIZE,
            ),
        )?;

        Ok(())
    }

    pub fn draw_sprite_line(&mut self, textures: &[Texture], x: u16, y: u16) -> Result<(), String> {
        let i1 = y / 8;
        let i2 = y % 8;
        let sprite_idx = self.ppu_map().name_table_00
            [((i1 as usize) * PLAYGROUND_WIDTH as usize) + x as usize]
            as usize;
        let sprite_row = self
            .ppu_map()
            .addr((sprite_idx * 0x10 as usize) as u16 + i2);
        let sprite_high = self
            .ppu_map()
            .addr((sprite_idx * 0x10 as usize + 0x8) as u16 + i2);
        for j in 0..8 {
            let row_idx = (sprite_row & (0b1 << (7 - j)) != 0) as u16;
            let high_idx = (sprite_high & (0b1 << (7 - j)) != 0) as u16;
            let idx = high_idx << 1 | row_idx;
            let sprite_color_idx = self.ppu_map().addr(0x3F00 + idx);
            let square_texture = &textures[sprite_color_idx as usize];

            self.canvas.copy(
                square_texture,
                None,
                Rect::new(
                    (j + x as u32 * SQUARE_SIZE) as i32,
                    y as i32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                ),
            )?;
        }

        Ok(())
    }

    pub fn draw_line(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH {
            self.draw_backgraund_line(textures, n as u16, self.drawing_line)?;
            self.draw_sprite_line(textures, n as u16, self.drawing_line)?;
        }
        Ok(())
    }

    pub fn is_just_in_vblank_line(&self) -> bool {
        self.drawing_line == VBLANK_LINE
    }

    pub fn set_sprites(&mut self, chars: &Vec<u8>) {
        if chars.len() > 0x2000 {
            unimplemented!();
        }
        for (i, chr) in chars.iter().enumerate() {
            self.cpu.bus.ppu.map.set(i as u16, *chr);
        }
    }
}
