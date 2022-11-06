pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::configure::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SPRITE_SIZE, SQUARE_SIZE};
use crate::emulator::texture::{dummy_texture, texture_combine_builtin_colors};
use crate::ppu::oam::SpriteInfo;
use configure::{PPU_DRAW_LINE_CYCLE, TOTAL_LINE, VBLANK_LINE, VERTICAL_PIXEL};
use rustc_hash::FxHashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Emulator {
    pub cpu: CPU,
    pub ppu_cycle: u16,
    pub drawing_line: u16,
    pub sdl: Sdl,
    canvas: Canvas<Window>,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    fn new() -> Self {
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
            ppu_cycle: 0,
            drawing_line: 0,
            sdl: sdl_context,
            canvas,
        }
    }

    pub fn reset(&mut self) {
        let pc = self.cpu.get_pc() as u8;
        self.cpu.push_stack(pc);
        let p = self.cpu.get_p();
        self.cpu.push_stack(p);
        self.cpu.reset();
    }

    fn inc_ppu_cycle(&mut self) {
        self.ppu_cycle += (self.cpu.cycle * 3) as u16;
        self.cpu.clear_cycle();
    }

    pub fn render_all_sprites(&mut self, sprites_num: u32) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let (square_texture1, square_texture2, square_texture3, square_texture4) =
            dummy_texture(&mut self.canvas, &texture_creator)?;

        for n in 0..sprites_num {
            for i in 0..8 {
                let sprite_row_line = self.cpu.bus.ppu.map.addr((n * 0x10) as u16 + i);
                let sprite_high_line = self.cpu.bus.ppu.map.addr((n * 0x10) as u16 + i + 0x8);
                for j in 0..8 {
                    let r = ((sprite_row_line & (0b1 << (7 - j))) != 0) as u16;
                    let h = ((sprite_high_line & (0b1 << (7 - j))) != 0) as u16;
                    let sprite_dot = h << 1 | r;
                    let square_texture = match sprite_dot {
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
                            (j as u32 + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            (i as u32 + (n / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            SPRITE_SIZE,
                            SPRITE_SIZE,
                        ),
                    )?;
                }
            }
        }
        self.canvas.present();

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
        Ok(())
    }

    fn handle_keyboard(&mut self, event_pump: &EventPump) -> Option<u32> {
        let pressed_scancodes: FxHashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for scancode in pressed_scancodes.iter() {
            match *scancode {
                Keycode::Escape => return None,
                Keycode::A => self.cpu.bus.controller_0_polling_data |= 0b00000001,
                Keycode::B => self.cpu.bus.controller_0_polling_data |= 0b00000010,
                Keycode::Space => self.cpu.bus.controller_0_polling_data |= 0b00000100,
                Keycode::Z => self.cpu.bus.controller_0_polling_data |= 0b00001000,
                Keycode::Up => self.cpu.bus.controller_0_polling_data |= 0b00010000,
                Keycode::Down => self.cpu.bus.controller_0_polling_data |= 0b00100000,
                Keycode::Left => self.cpu.bus.controller_0_polling_data |= 0b01000000,
                Keycode::Right => self.cpu.bus.controller_0_polling_data |= 0b10000000,
                _ => (),
            };
        }
        Some(1)
    }

    pub fn main_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let textures = texture_combine_builtin_colors(&mut self.canvas, &texture_creator)?;
        'running: loop {
            match self.handle_keyboard(&event_pump) {
                Some(_) => (),
                None => break 'running,
            }
            event_pump.poll_event();

            self.run(&textures)?;
        }

        Ok(())
    }

    fn run(&mut self, textures: &[Texture]) -> Result<(), String> {
        self.cpu.ex_ope();
        self.inc_ppu_cycle();
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VERTICAL_PIXEL {
                self.draw_line(textures)?;
                if self.cpu.bus.ppu.register.ppu_mask.show_sprites {
                    self.prepare_secondary_oam();
                    self.draw_sprites_refed_secondary_oams(textures)?;
                }
            }
            if self.drawing_line == TOTAL_LINE {
                self.canvas.present();
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
            if self.is_just_in_vblank_line() {
                self.cpu.bus.ppu.register.ppu_status.in_vlank = true;
                if self.cpu.bus.ppu.register.ppu_ctrl.gen_nmi {
                    self.cpu.interrupt(Interrupt::Nmi);
                }
            } else if self.drawing_line == 0 {
                self.cpu.bus.ppu.register.ppu_status.in_vlank = false;
                self.cpu.set_interrupt(false);
            }
        }
        Ok(())
    }

    fn clear_secondary_oam(&mut self) {
        self.cpu.bus.ppu.secondary_oam.clear_sprite_infos();
    }

    fn set_secondary_oam(&mut self) {
        self.cpu
            .bus
            .ppu
            .set_secondary_oam_in_line(self.drawing_line as u8);
    }

    fn prepare_secondary_oam(&mut self) {
        self.clear_secondary_oam();
        self.set_secondary_oam();
    }

    fn draw_backgraund_line(&mut self, textures: &[Texture], x: u16, y: u16) -> Result<(), String> {
        let attr_idx = (0x23C0 + (x / 4) + (y / 32) * 8) as u16;
        let attr_arr_idx = ((x / 2) % 2) | (((y / 16) % 2) << 1);
        let ppu_map = &mut self.cpu.bus.ppu.map;
        let pallets = ppu_map.addr(attr_idx);
        let pallete_idx = ppu_map.addr(0x3F00 + (pallets & (0x1 << attr_arr_idx)) as u16);
        let texture = if pallete_idx as usize <= textures.len() {
            &textures[pallete_idx as usize]
        } else {
            &textures[(pallete_idx as usize) - &textures.len()]
        };

        self.canvas.copy(
            texture,
            None,
            Rect::new(
                (x as u32 * SQUARE_SIZE) as i32,
                (self.drawing_line) as i32,
                SPRITE_SIZE,
                SPRITE_SIZE,
            ),
        )?;

        Ok(())
    }

    fn draw_sprite_line(&mut self, textures: &[Texture], x: u16, y: u16) -> Result<(), String> {
        let i1 = y / 8;
        let i2 = y % 8;
        let ppu_map = &mut self.cpu.bus.ppu.map;

        let sprite_idx = ppu_map.addr((0x2000 + i1 * PLAYGROUND_WIDTH as u16) + x);
        let sprite_row = ppu_map.addr((sprite_idx as u16 * 0x10) as u16 + i2);
        let sprite_high = ppu_map.addr((sprite_idx as u16 * 0x10 + 0x8) as u16 + i2);
        for j in 0..8 {
            let row_idx = (sprite_row & (0b1 << (7 - j)) != 0) as u16;
            let high_idx = (sprite_high & (0b1 << (7 - j)) != 0) as u16;
            let idx = high_idx << 1 | row_idx;
            let sprite_color_idx = ppu_map.addr(0x3F00 + idx);
            let square_texture = if (sprite_color_idx as usize) <= textures.len() {
                &textures[sprite_color_idx as usize]
            } else {
                &textures[(sprite_color_idx as usize) - textures.len()]
            };

            self.canvas.copy(
                square_texture,
                None,
                Rect::new(
                    (j + x as u32 * SQUARE_SIZE) as i32,
                    y as i32,
                    SPRITE_SIZE,
                    SPRITE_SIZE,
                ),
            )?;
        }

        Ok(())
    }

    fn draw_line(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH {
            self.draw_backgraund_line(textures, n as u16, self.drawing_line)?;
            self.draw_sprite_line(textures, n as u16, self.drawing_line)?;
        }
        Ok(())
    }

    fn is_just_in_vblank_line(&self) -> bool {
        self.drawing_line == VBLANK_LINE
    }

    pub fn set_sprites(&mut self, chars: &Vec<u8>) {
        if chars.len() > 0x2000 {
            unimplemented!()
        }
        for (i, chr) in chars.iter().enumerate() {
            self.cpu.bus.ppu.map.set(i as u16, *chr);
        }
    }

    fn draw_sprites_refed_secondary_oams(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH {
            match self
                .cpu
                .bus
                .ppu
                .secondary_oam
                .pick_sprite_info_with_x(n as u8 * 8)
            {
                Some(SpriteInfo {
                    pos_y,
                    tile_index,
                    pos_x,
                    ..
                }) => {
                    let relative_hight = (self.drawing_line - *pos_y as u16) % 8;
                    let base_addr = tile_index.bank_of_tile as u16 * 0x1000
                        + (tile_index.tile_number as u16) * 0x10
                        + relative_hight;
                    let ppu_map = &mut self.cpu.bus.ppu.map;
                    let sprite_row = ppu_map.addr(base_addr);
                    let sprite_high = ppu_map.addr(base_addr + 8);
                    for j in 0..8 {
                        let row_idx = (sprite_row & (0b1 << (7 - j)) != 0) as u16;
                        let high_idx = (sprite_high & (0b1 << (7 - j)) != 0) as u16;
                        let idx = high_idx << 1 | row_idx;
                        let sprite_color_idx = ppu_map.addr(0x3F00 + idx);
                        let square_texture = if sprite_color_idx as usize <= textures.len() {
                            &textures[sprite_color_idx as usize]
                        } else {
                            &textures[(sprite_color_idx as usize) - textures.len()]
                        };
                        self.canvas.copy(
                            square_texture,
                            None,
                            Rect::new(
                                (*pos_x + j) as i32,
                                self.drawing_line as i32,
                                SPRITE_SIZE,
                                SPRITE_SIZE,
                            ),
                        )?;
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }
}
