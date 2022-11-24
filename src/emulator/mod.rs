pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::texture::texture_combine_builtin_colors;
use crate::ppu::oam::SpriteInfo;
use rustc_hash::FxHashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Emulator<
    const PLAYGROUND_HEIGHT: u32,
    const PLAYGROUND_WIDTH: u32,
    const SQUARE_SIZE: u32,
    const SPRITE_SIZE: u32,
    const PPU_DRAW_LINE_CYCLE: u16,
    const VBLANK_LINE: u16,
    const TOTAL_LINE: u16,
    const VERTICAL_PIXEL: u16,
> {
    pub cpu: CPU,
    pub ppu_cycle: u16,
    pub drawing_line: u16,
    pub sdl: Sdl,
    canvas: Canvas<Window>,
}

impl<
        const PLAYGROUND_HEIGHT: u32,
        const PLAYGROUND_WIDTH: u32,
        const SQUARE_SIZE: u32,
        const SPRITE_SIZE: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VERTICAL_PIXEL: u16,
    > Default
    for Emulator<
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        SQUARE_SIZE,
        SPRITE_SIZE,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VERTICAL_PIXEL,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        const PLAYGROUND_HEIGHT: u32,
        const PLAYGROUND_WIDTH: u32,
        const SQUARE_SIZE: u32,
        const SPRITE_SIZE: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VERTICAL_PIXEL: u16,
    >
    Emulator<
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        SQUARE_SIZE,
        SPRITE_SIZE,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VERTICAL_PIXEL,
    >
{
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

    pub fn startup(&mut self) {
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
        let textures = texture_combine_builtin_colors(&mut self.canvas, &texture_creator)?;
        let ppu_map = &mut self.cpu.bus.ppu.map;

        for n in 0..sprites_num {
            for i in 0..8 {
                let sprite_row_line = ppu_map.addr((n * 0x10) as u16 + i);
                let sprite_high_line = ppu_map.addr((n * 0x10) as u16 + i + 0x8);
                for j in 0..8 {
                    let (idx, x, y) = {
                        let idx = {
                            let r = ((sprite_row_line & (0b1 << (7 - j))) != 0) as u16;
                            let h = ((sprite_high_line & (0b1 << (7 - j))) != 0) as u16;
                            h << 1 | r
                        };
                        let x = (j as u32 + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32;
                        let y = (i as u32 + (n / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32;
                        (idx, x, y)
                    };
                    let texture = &textures[idx as usize];

                    self.canvas
                        .copy(texture, None, Rect::new(x, y, SPRITE_SIZE, SPRITE_SIZE))?;
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
                Keycode::X => self.cpu.bus.controller_0_polling_data |= 0b00000001,
                Keycode::Z => self.cpu.bus.controller_0_polling_data |= 0b00000010,
                Keycode::Space => self.cpu.bus.controller_0_polling_data |= 0b00000100,
                Keycode::Return => self.cpu.bus.controller_0_polling_data |= 0b00001000,
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

    fn enable_render_bottom(&self) -> bool {
        self.drawing_line >= 8
    }

    fn draw_sprite_for_big_size(&mut self, textures: &[Texture]) -> Result<(), String> {
        self.draw_sprites_for_big_top(textures)?;
        if self.enable_render_bottom() {
            self.set_secondary_oam_for_bottom();
            self.draw_sprites_for_big_bottom(textures)?;
        };
        Ok(())
    }

    fn run(&mut self, textures: &[Texture]) -> Result<(), String> {
        self.cpu.ex_ope();
        self.inc_ppu_cycle();
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VERTICAL_PIXEL {
                self.draw_line(textures)?;
                if self.cpu.bus.cpu_bus.ppu_register.ppu_mask.show_sprites {
                    self.set_secondary_oam_for_nomal();
                    if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.for_big() {
                        self.draw_sprite_for_big_size(textures)?;
                    } else {
                        self.draw_sprites_for_normal_size(textures)?;
                    }
                }
            }
            if self.drawing_line == TOTAL_LINE {
                self.canvas.present();
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
            if self.is_just_in_vblank_line() {
                self.cpu.bus.cpu_bus.ppu_register.ppu_status.in_vlank = true;
                if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.gen_nmi {
                    self.cpu.interrupt(Interrupt::Nmi);
                }
            } else if self.drawing_line == 0 {
                self.cpu.bus.cpu_bus.ppu_register.ppu_status.in_vlank = false;
                self.cpu.set_interrupt(false);
            }
        }
        Ok(())
    }

    fn set_secondary_oam_for_nomal(&mut self) {
        self.cpu.bus.ppu.set_secondary_oam(self.drawing_line as u8);
    }

    fn set_secondary_oam_for_bottom(&mut self) {
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8 - 8);
    }

    fn draw_sprite_line(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH as u16 {
            let i1 = self.drawing_line / 8;
            let i2 = self.drawing_line % 8;
            let ppu_map = &mut self.cpu.bus.ppu.map;
            let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;

            let sprite_idx = ppu_map.addr((0x2000 + i1 * PLAYGROUND_WIDTH as u16) + n);
            let base_addr = (sprite_idx as u16 * 0x10) as u16 + i2;
            let sprite_row = ppu_map.addr(base_addr);
            let sprite_high = ppu_map.addr(base_addr + 0x8);
            for j in 0..8 {
                let (idx, x, y) = {
                    let idx = {
                        let row_idx = (sprite_row & (0b1 << (7 - j)) != 0) as u16;
                        let high_idx = (sprite_high & (0b1 << (7 - j)) != 0) as u16;
                        high_idx << 1 | row_idx
                    };
                    let x = (j + n as u32 * SQUARE_SIZE) as i32;
                    let y = self.drawing_line as i32;
                    (idx, x, y)
                };
                let mut sprite_color_idx = ppu_map.background_table[idx as usize];
                if ppu_mask.gray_scale {
                    sprite_color_idx &= 0b11110000;
                }
                let square_texture = if (sprite_color_idx as usize) < textures.len() {
                    &textures[sprite_color_idx as usize]
                } else {
                    &textures[(sprite_color_idx as usize) - textures.len()]
                };

                self.canvas.copy(
                    square_texture,
                    None,
                    Rect::new(x, y, SPRITE_SIZE, SPRITE_SIZE),
                )?;
            }
        }

        Ok(())
    }

    fn draw_line(&mut self, textures: &[Texture]) -> Result<(), String> {
        Ok(self.draw_sprite_line(textures)?)
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

    fn draw_sprites_for_big_top(&mut self, textures: &[Texture]) -> Result<(), String> {
        Ok(self.draw_sprites_for_big_size(textures, false)?)
    }

    fn draw_sprites_for_big_bottom(&mut self, textures: &[Texture]) -> Result<(), String> {
        Ok(self.draw_sprites_for_big_size(textures, true)?)
    }

    fn draw_sprites_for_big_size(
        &mut self,
        textures: &[Texture],
        is_bottom: bool,
    ) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH * 8 {
            match self
                .cpu
                .bus
                .ppu
                .secondary_oam
                .pick_sprite_info_with_x(n as u8)
            {
                Some(SpriteInfo {
                    pos_y,
                    tile_index,
                    pos_x,
                    attr,
                }) => {
                    let relative_hight = (self.drawing_line - *pos_y as u16) % 8;
                    let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;
                    let base_addr = (((tile_index.tile_number + 1) % 2) == 0) as u16 * 0x1000
                        + ((tile_index.tile_number as u16) / 2) * 0x20
                        + relative_hight;
                    let ppu_map = &mut self.cpu.bus.ppu.map;
                    let (sprite_row, sprite_high) = {
                        if is_bottom {
                            (
                                ppu_map.addr(base_addr + 0x10),
                                ppu_map.addr(base_addr + 0x18),
                            )
                        } else {
                            (ppu_map.addr(base_addr), ppu_map.addr(base_addr + 8))
                        }
                    };
                    let pallet_base_idx = (attr.palette * 4) as usize;
                    let for_count = if is_bottom { 16 } else { 8 };
                    for i in for_count - 8..for_count {
                        let (idx, x, y) = {
                            if is_bottom {
                                let idx = {
                                    let r = (sprite_row & (0b1 << (15 - i)) != 0) as u16;
                                    let h = (sprite_high & (0b1 << (15 - i)) != 0) as u16;
                                    h << 1 | r
                                };
                                let x = pos_x.wrapping_add(i - 8) as i32;
                                let y = (self.drawing_line) as i32;
                                (idx, x, y)
                            } else {
                                let idx = {
                                    let r = (sprite_row & (0b1 << (7 - i)) != 0) as u16;
                                    let h = (sprite_high & (0b1 << (7 - i)) != 0) as u16;
                                    h << 1 | r
                                };
                                let x = pos_x.wrapping_add(i) as i32;
                                let y = (self.drawing_line) as i32;
                                (idx, x, y)
                            }
                        };

                        let mut color_idx = ppu_map.sprite_pallet[pallet_base_idx + idx as usize];
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }

                        let square_texture = if (color_idx as usize) < textures.len() {
                            &textures[color_idx as usize]
                        } else {
                            &textures[(color_idx as usize) - textures.len()]
                        };
                        self.canvas.copy(
                            square_texture,
                            None,
                            Rect::new(x, y, SPRITE_SIZE, SPRITE_SIZE),
                        )?;
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }

    fn draw_sprites_for_normal_size(&mut self, textures: &[Texture]) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH * 8 {
            match self
                .cpu
                .bus
                .ppu
                .secondary_oam
                .pick_sprite_info_with_x(n as u8)
            {
                Some(SpriteInfo {
                    pos_y,
                    tile_index,
                    pos_x,
                    attr,
                }) => {
                    let ppu_ctrl = &self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl;
                    let relative_hight = (self.drawing_line - *pos_y as u16) % 8;
                    let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;
                    let base_addr = tile_index.bank_of_tile as u16 * 0x1000
                        + ppu_ctrl.sprite_ptn_table_addr as u16 * 0x1000
                        + (tile_index.tile_number as u16) * 0x10
                        + if attr.flip_sprite_vertically {
                            7 - relative_hight
                        } else {
                            relative_hight
                        };
                    let ppu_map = &mut self.cpu.bus.ppu.map;
                    let sprite_row = ppu_map.addr(base_addr);
                    let sprite_high = ppu_map.addr(base_addr + 8);
                    let pallet_base_idx = (attr.palette * 4) as usize;
                    for i in 0..8 {
                        let (idx, x, y) = {
                            let idx = {
                                let i = if attr.flip_sprite_horizontally {
                                    i
                                } else {
                                    7 - i
                                };

                                let r = (sprite_row & (0b1 << i) != 0) as u16;
                                let h = (sprite_high & (0b1 << i) != 0) as u16;
                                h << 1 | r
                            };
                            let x = (pos_x.wrapping_add(i)) as i32;
                            let y = self.drawing_line as i32;
                            (idx, x, y)
                        };
                        let mut color_idx = ppu_map.sprite_pallet[pallet_base_idx + idx as usize];
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }
                        let square_texture = if (color_idx as usize) < textures.len() {
                            &textures[color_idx as usize]
                        } else {
                            &textures[(color_idx as usize) - textures.len()]
                        };

                        self.canvas.copy(
                            square_texture,
                            None,
                            Rect::new(x, y, SPRITE_SIZE, SPRITE_SIZE),
                        )?;
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }
}
